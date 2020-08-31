
use serde_json;
use std::str;
use actix_web::{web::{Bytes, Data}, Responder, HttpResponse};
use slog::{error, o, Logger};
use http::StatusCode;

use crate::errors::{AppErrorType, AppError};
use crate::posting;
use crate::models::{AppState, TelegramReq, Message, Status};
use crate::db::DbProcessor;

struct TypeHtml;
struct TypeMarkdown;

trait Html {}
trait Markdown {}

impl Html for TypeHtml {}
impl Markdown for TypeMarkdown {}

fn html<T: Html>(_: &T) -> &'static str { "html" }
fn markdown<T: Markdown>(_: &T) -> &'static str { "markdown" }

fn log_error(log: Logger) -> impl Fn(AppError) -> AppError {
    move |err| {
        let log = log.new(o!(
            "cause" => err.cause.clone()
        ));
        error!(log, "{}", err.message());
        err
    }
}

pub async fn status() -> Result<impl Responder, AppError> {
    Ok(HttpResponse::Ok().json(Status {status: "up".to_string()}))
}

pub async fn process(state: Data<AppState>, bytes: Bytes) -> Result<impl Responder, AppError> {
    let sublog = state.log.new(o!("handler" => "process"));
    match String::from_utf8(bytes.to_vec()) {
        Ok(ss) => {
            //println!("{}", ss);
            let json: TelegramReq = serde_json::from_str(ss.as_str()).unwrap();
            if json.message.chat.chat_type == "group" {
                let message_text = json.message.text;
                let split = message_text.split("@");
                let arr = split.collect::<Vec<&str>>();
                let mut name: String = json.message.from.first_name;
                let lmp = json.message.from.last_name;

                if lmp != "" {
                    name.push_str(" ");
                    name.push_str(lmp.as_str());
                }
                match *(arr.first().unwrap()) {
                    "/start" => {
                        let req = message_start(&name, json.message.chat.id);
                        let _ = posting::update(&req, sublog.clone(), state.token.clone(), String::from("/sendMessage")).await.unwrap();
                    },
                    "/bantuan" => {
                        let req = message_bantuan(&name, json.message.chat.id);
                        let _ = posting::update(&req, sublog.clone(), state.token.clone(), String::from("/sendMessage")).await.unwrap();
                    },
                    "/saldo" => {
                        let path = state.path.clone();
                        let req = message_saldo(&name, json.message.chat.id, path).await;
                        let _ = posting::update(&req, sublog.clone(), state.token.clone(), String::from("/sendMessage")).await.unwrap();
                    },
                    _ => {
                        let msg = format!("السلام عليكم ورحمة الله\n\nAhlan wa Sahlan {}", name);
                        let req = Message {chat_id: json.message.chat.id, text: msg, parse_mode: String::from(html(&TypeHtml))};
                        let _ = posting::update(&req, sublog.clone(), state.token.clone(), String::from("/sendMessage")).await.unwrap();
                    }
                }
            }
            
            Ok("".with_status(StatusCode::OK))
        },
        Err(_) => Err(AppError {cause: None, message: Some(String::from("")), error_type: AppErrorType::NotFoundError}),
    }
}

fn message_start(name: &String, id: i64) -> Message {
    let msg = format!("السلام عليكم ورحمة الله\n\nAhlan wa Sahlan <b>{}</b>, ini adalah bot Grup Amanah Muhsinin MTQS.\n\n Ketik /bantuan untuk melihat menu yang ada.\n\n <b><i>Layanan ini InsyaAllah ada 24 jam</i></b>", name);
    let req = Message {chat_id: id, text: msg, parse_mode: String::from(html(&TypeHtml))};
    req
}

fn message_bantuan(name: &String, id: i64) -> Message {
    let msg = format!("<b>{}</b>,\n\nBerikut adalah menu yang tersedia di bot Amanah Muhsinin MTQS\n\n1. Cek saldo ketik /saldo\n\nMenu yang InsyaAllah akan diupdate.\n\n <b>Admin</b>.\n<b><i>Layanan ini InsyaAllah ada 24 jam</i></b>", name);
    let req = Message {chat_id: id, text: msg, parse_mode: String::from(html(&TypeHtml))};
    req
}

async fn message_saldo(name: &String, id: i64, url_s: String) -> Message {
    let db = DbProcessor{url: url_s};
    let data = db.get_data_donasi().await.unwrap();
    let mut msg = format!("<b>{}</b>,\n<b>Saldo donasi MTQS sekarang adalah:</b>\n\n", name);
    msg.push_str("<pre>");
    for item in data.0 {
        msg.push_str(item.as_str());
    }
    msg.push_str("</pre>");
    msg.push_str("\n");
    let total = format!("<b>Total saldo: Rp {}.</b>\n\nSemoga bermanfaat.\nUntuk info lainnya ketik /bantuan", data.1);
    msg.push_str(total.as_str());
    let req = Message {chat_id: id, text: msg, parse_mode: String::from(html(&TypeHtml))};
    req
}



