use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::serde_json::json;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub uid: uuid::Uuid,
    pub name: String,
    pub password: String,
}

impl User {
    pub fn into_json(self) -> String {
        let v = json!(Response{uid: self.uid, name: self.name});
        v.to_string()
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub uid: uuid::Uuid,
    pub name: String,
}