use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Claims {
    pub exp: i64, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    pub iat: i64, // Optional. Issued at (as UTC timestamp)
    pub nbf: i64, // Optional. Not Before (as UTC timestamp)
    pub aud: String, // Optional. Audience
    pub iss: String, // Optional. Issuer
    pub sub: String, // Optional. Subject (whom token refers to)
}

impl Claims {
    pub fn new(uid: uuid::Uuid) -> Self {
        let exp = match chrono::Utc::now().checked_add_days(chrono::Days::new(2)) {
            Some(t) => t.timestamp(),
            None => chrono::Utc::now().timestamp(),
        };
        let iat = chrono::Utc::now().timestamp();
        let nbf = chrono::Utc::now().timestamp();
        let aud = "user".to_string();
        let iss = "Xorsense".to_string();
        let sub = uid.to_string();
        Claims {
            exp,
            iat,
            nbf,
            aud,
            iss,
            sub,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Token(String);

#[derive(Debug)]
pub enum TokenError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = TokenError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.headers().get_one("Authorization") {
            Some(key) => {
                let key = key.split_whitespace().last().unwrap().trim();
                let token = Token(key.to_owned());
                match token.claims() {
                    Some(_claims) => Outcome::Success(token),
                    None => Outcome::Failure((Status::Unauthorized, TokenError::Invalid)),
                }
            }
            None => Outcome::Failure((Status::Unauthorized, TokenError::Missing)),
        }
    }
}

impl Token {
    pub fn new(uid: uuid::Uuid) -> Option<Self> {
        let secret = std::env::var("JWT_SECRET").unwrap();
        let claims = Claims::new(uid);
        let token = encode(
            &Header::new(Algorithm::HS512),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        );

        if let Err(err) = token {
            println!("error encoding new token: {}", err);
            return None;
        };
        Some(Token(token.unwrap()))
    }

    pub fn claims(&self) -> Option<Claims> {
        let secret = std::env::var("JWT_SECRET").unwrap();
        let token = decode::<Claims>(
            self.0.as_str(),
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::new(Algorithm::HS512),
        );
        match token {
            Ok(t) => Some(t.claims),
            Err(err) => {
                println!("error decoding token: {}", err);
                None
            }
        }
    }
}
