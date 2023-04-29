use rocket::serde::json::{json, Json, serde_json, Value};
use rocket::serde::{Deserialize, Serialize};
use rocket::State;

use sha2::{Digest, Sha512};

use crate::api::jwt::Token;
use crate::api::state;
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
    async fn authenticate(&self, supa: &state::Supabase) -> Option<User> {
        let hp = self.hashed_password();
        let resp = supa.client
            .from("users")
            .select("*")
            .eq("name", self.name.clone())
            .eq("password", hp)
            .execute()
            .await
            .ok()?;
        match resp.text().await {
            Ok(data) => {
                match serde_json::from_str(data.as_str()) {
                    Ok(u) => Some(u),
                    Err(_) => None
                }
            },
            Err(_) => None
        }
    }
    async fn register(&self, supa: &state::Supabase) -> Result<User, AuthError> {
        let hp = self.hashed_password();
        let resp = supa.client
            .from("users")
            .select("*")
            .eq("name", self.name.clone())
            .limit(1)
            .execute()
            .await;
        match resp {
            Ok(user) => {
                println!("user found: {:?}", user);
                return Err(AuthError::DuplicateRegistration);
            }
            Err(_) => {}
        };
        let user = User {
            uid: uuid::Uuid::new_v4(),
            name: self.name.clone(),
            password: hp,
        };
        let resp = supa.client
            .from("users")
            .insert(user.clone().into_json())
            .execute()
            .await;
        match resp {
            Ok(_) => Ok(user),
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
    fn success(message: String, token: Token, user: User) -> Self {
        let status = AuthStatus::Success;
        let expires = Some(token.clone().claims().unwrap().exp);
        let token = Some(token.clone());
        let user = Some(user::Response {
            uid: user.uid,
            name: user.name,
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
pub async fn authenticate<'a>(client: &State<state::Supabase>, auth: Json<Auth>) -> Value {
    match auth.authenticate(client.inner()).await {
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
pub async fn register<'a>(client: &State<state::Supabase>, auth: Json<Auth>) -> Value {
    match auth.register(client.inner()).await {
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
