use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;
use uuid::Timestamp;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub uid: uuid::Uuid,
    pub name: String,
    pub password: String,
    pub created_at: Timestamp
}

impl User {
    pub fn into_json(&self) -> Value {
        json!(Response{uid: self.uid, name: self.name})
    }
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub uid: uuid::Uuid,
    pub name: String,
}