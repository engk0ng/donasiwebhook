
use serde_json;
use std::str;
use actix_web::{web::{Bytes, Data}, Responder, HttpResponse};
use slog::{error, o, Logger};
use http::StatusCode;

use crate::errors::{AppErrorType, AppError};
use crate::posting;
use crate::models::{AppState, TelegramReq, Message, Status};

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
            let json: TelegramReq = serde_json::from_str(ss.as_str()).unwrap();
            if json.message.chat.chat_type == "group" {
                if json.message.text.as_str() == "/start" {
                    let mut name: String = json.message.from.first_name;
                    let lmp = json.message.from.last_name;
    
                    if lmp != "" {
                        name.push_str(" ");
                        name.push_str(lmp.as_str());
                    }
                    
                    let msg = format!("السلام عليكم ورحمة الله\n\nAhlan wa Sahlan {}, ini adalah bot Grup Amanah Muhsinin MTQS.\n\n Ketik \\bantuan untuk melihat menu yang ada.", name);
                    let req = Message {chat_id: json.message.chat.id, text: msg, parse_mode: String::from(html(&TypeHtml))};
                    let _ = posting::update(&req, sublog.clone(), state.token.clone(), String::from("/sendMessage")).await.unwrap();
                }
            }
            Ok("".with_status(StatusCode::OK))
        },
        Err(_) => Err(AppError {cause: None, message: Some(String::from("")), error_type: AppErrorType::NotFoundError}),
    }
    
}



