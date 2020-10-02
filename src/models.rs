use serde::{Deserialize, Serialize};
use slog;

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
    pub text: String,
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
