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

#[derive(Clone)]
pub struct AppState {
    pub log: slog::Logger,
    pub token: String
}

#[derive(Serialize, Deserialize)]
pub struct TelegramReq {
    #[serde(rename = "update_id")]
    update_id: i64,

    #[serde(rename = "message")]
    pub message: Messages,
}

#[derive(Serialize, Deserialize)]
pub struct Messages {
    #[serde(rename = "message_id")]
    message_id: i64,

    #[serde(rename = "from")]
    from: From,

    #[serde(rename = "chat")]
    pub chat: Chat,

    #[serde(rename = "date")]
    date: i64,

    #[serde(rename = "text")]
    pub text: String,

    #[serde(rename = "entities")]
    entities: Vec<Entity>,
}

#[derive(Serialize, Deserialize)]
pub struct Chat {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "first_name")]
    pub first_name: String,

    #[serde(rename = "last_name")]
    pub last_name: String,

    #[serde(rename = "username")]
    username: String,

    #[serde(rename = "type")]
    chat_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct Entity {
    #[serde(rename = "offset")]
    offset: i64,

    #[serde(rename = "length")]
    length: i64,

    #[serde(rename = "type")]
    entity_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct From {
    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "is_bot")]
    is_bot: bool,

    #[serde(rename = "first_name")]
    first_name: String,

    #[serde(rename = "last_name")]
    last_name: String,

    #[serde(rename = "username")]
    username: String,

    #[serde(rename = "language_code")]
    language_code: String,
}