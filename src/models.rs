
use anyhow::Result;
use serde::{Deserialize, Serialize};
use slog;
use std::{iter::Sum};
use sqlx::PgPool;

#[derive(Debug, Serialize)]
pub struct Status {
    pub status: String
}

#[derive(Serialize, Debug)]
pub struct Message {
    pub chat_id: i64,
    pub text: String,
    pub parse_mode: String,
}

impl Message {
    pub fn new(_chat_id: i64, _text: String, _parse_mode: String) -> Self {
        Self{
            chat_id: _chat_id,
            text: _text,
            parse_mode: _parse_mode,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(Clone)]
pub struct AppState {
    pub log: slog::Logger,
    pub token: String,
    pub path: String,
    pub pool: sqlx::PgPool,
}

#[derive(Serialize, Deserialize)]
pub struct TelegramReq {
    update_id: i64,
    pub message: Messages,
}

#[derive(Serialize, Deserialize)]
pub struct Messages {
    message_id: i64,
    pub from: From,
    pub chat: Chat,
    date: i64,
    pub text: Option<String>,
    entities: Option<Vec<Entity>>,
}

#[derive(Serialize, Deserialize)]
pub struct Chat {
    pub id: i64,
    title: Option<String>,
    #[serde(rename = "type")]
    pub chat_type: String,
    all_members_are_administrators: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct Entity {
    offset: i64,
    length: i64,
    #[serde(rename = "type")]
    entity_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct From {
    id: i64,
    is_bot: bool,
    pub first_name: String,
    pub last_name: Option<String>,
    username: Option<String>,
    language_code: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct DataRekap {
    pub data: Vec<Rekap>,
    pub message: String,
    pub status: String,
}

#[derive(Serialize, Deserialize)]
pub struct Rekap {
    pub code: String,
    pub name: String,
    pub nominal: i64,
}

#[derive(Debug, Clone)]
pub struct SumberDana {
    pub id_sumber_dana: i32,
    pub nama: String,
    pub kode: String,
    pub bg: String,
}

impl SumberDana {
    pub fn new(nm: String, kd: String, _bg: String) -> Self {
        Self {
            id_sumber_dana: 0,
            nama: nm,
            kode: kd,
            bg: _bg,
        }
    }
}

impl SumberDana {
    pub async fn count_debet(&self, pool: &PgPool) -> Result<i64> {
         let k = count_sd_debet(&self.kode, &pool).await;
         Ok(k)
    }

    pub async fn count_kredit(&self, pool: &PgPool) -> Result<i64> {
        let l = count_sd_kredit(&self.kode, &pool).await;
        Ok(l)
    }
}

async fn count_sd_debet(kode: &String, pool: &PgPool) -> i64 {
    let rec = sqlx::query!(r#"
    select nominal from donasi.debet where kode = $1 and status_delete = $2
    "#, kode, false)
    .fetch_all(pool)
    .await;
    let mut count: i64 = 0;
    match rec {
        Ok(res) => {
            for item in res {
                count += item.nominal;
            }
        },
        Err(_e) => count = 0
    }
    count
}

async fn count_sd_kredit(kode: &String, pool: &PgPool) -> i64 {
    let rec = sqlx::query!(r#"
    select nominal from donasi.kredit where kode = $1 and status_delete = $2
    "#, kode, false)
    .fetch_all(pool)
    .await;
    let mut count: i64 = 0;
    match rec {
        Ok(res) => {
            for item in res {
                count += item.nominal;
            }
        },
        Err(_e) => count = 0
    }
    count
}
