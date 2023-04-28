#[macro_use]
extern crate rocket;

use std::env;

use rocket::{State, http::Status};

use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

mod api;

#[get("/<id>")]
fn hello(pool: &State<Pool<Postgres>>, id: String) -> Result<String, Status> {
    match id {
        name => Ok(format!("Hello, {}!", name)),
        _ => Err(Status::NotFound)
    }
}

#[get("/")]
fn hello_world() -> String {
    String::from("Hello, World!")
}

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    let database_url = env::var("DATABASE_URL");
    let pool = PgPoolOptions::new()
        .connect(&*database_url.unwrap()).await?;
    rocket::build()
        .mount("/", routes![hello, hello_world])
        .mount("/api", routes![api::auth::authenticate, api::auth::register])
        .manage(pool)
        .attach(api::cors)
        .launch().await?;
    Ok(())
}
