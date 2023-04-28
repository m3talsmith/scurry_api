use uuid::Timestamp;
use rocket::log::private::log;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};
use rocket::{log, State};

use sha2::{Digest, Sha512};

use crate::api::jwt::Token;
use crate::api::user::{self, User};

#[derive(Debug)]
enum AuthError {
    DuplicateRegistration,
    Unknown,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Auth {
    name: String,
    password: String,
}

impl Auth {
    fn hashed_password(&self) -> String {
        let mut hf = Sha512::new();
        hf.update(self.password.clone());
        let hp = hf.finalize();

        format!("{:x}", hp)
    }
    async fn authenticate(&self, db: &DatabaseConnection) -> Option<user::User> {
        let hp = self.hashed_password();
        let user = sqlx::query!(
            "select * from users where name = ? and password = ?",
            self.name, hp
        )
            .fetch_one(db)
            .await?;
        match user
        {
            Ok(user) => user,
            Err(err) => {
                println!("error authenticating: {}", err);
                None
            }
        }
    }
    async fn register(&self, db: &DatabaseConnection) -> Result<user::User, AuthError> {
        let hp = self.hashed_password();
        match sqlx::query!(
            "select * from users where name = ?",
            self.name
        ).fetch_one(db).await? {
            Ok(user) => {
                println!("user found: {:?}", user);
                return Err(AuthError::DuplicateRegistration);
            }
            Err(_) => {}
        };
        let user = user::User {
            uid: uuid::Uuid::new_v4(),
            name: self.name.clone(),
            password: hp,
            created_at: Timestamp::now(()),
        };
        
        match User::insert(user.clone()).exec(db).await {
            Ok(_) => Ok(user.try_into_model().unwrap()),
            Err(err) => {
                println!("err: {:?}", err);
                Err(AuthError::Unknown)
            }
        }
    }
}

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
enum AuthStatus {
    Success,
    Error,
}

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
struct Response {
    status: AuthStatus,
    message: String,
    token: Option<Token>,
    user: Option<user::Response>,
    expires: Option<i64>,
}

impl Response {
    fn success(message: String, token: Token, user: user::Model) -> Self {
        let status = AuthStatus::Success;
        let expires = Some(token.clone().claims().unwrap().exp);
        let token = Some(token.clone());
        let user = Some(user::Response {
            uid: user.uid,
            name: user.email,
        });
        Response {
            status,
            message,
            token,
            expires,
            user,
        }
    }

    fn error(message: String) -> Self {
        let status = AuthStatus::Error;
        let token = None;
        let expires = None;
        let user = None;
        Response {
            status,
            message,
            token,
            expires,
            user,
        }
    }
}

#[post("/authenticate", format = "application/json", data = "<auth>")]
pub async fn authenticate<'a>(db: &State<DatabaseConnection>, auth: Json<Auth>) -> Value {
    match auth.authenticate(db).await {
        Some(user) => match Token::new(user.uid) {
            Some(token) => json!(Response::success(
                "you are now logged in".to_owned(),
                token,
                user
            )),
            None => json!(Response::error("internal server error".to_owned())),
        },
        None => json!(Response::error("invalid credentials sent".to_owned())),
    }
}

#[post("/register", format = "application/json", data = "<auth>")]
pub async fn register<'a>(db: &State<DatabaseConnection>, auth: Json<Auth>) -> Value {
    match auth.register(db).await {
        Ok(user) => match Token::new(user.uid) {
            Some(token) => json!(Response::success(
                "you are now registered and logged in".to_owned(),
                token,
                user
            )),
            None => json!(Response::error("internal server error".to_owned())),
        },
        Err(err) => json!(Response::error(format!("{:?}", err))),
    }
}
