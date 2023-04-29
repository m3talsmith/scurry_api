#[macro_use]
extern crate rocket;

use std::env;

use postgrest::Postgrest;

mod api;

#[get("/<id>")]
fn hello(id: String) -> String { format!("Hello, {}!", id) }

#[get("/")]
fn hello_world() -> String {
    String::from("Hello, World!")
}

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    let api_url = env::var("SUPABASE_API_URL").unwrap();
    let api_key = env::var("SUPABASE_API_KEY").unwrap();
    let client = Postgrest::new(api_url.clone()).insert_header("apiKey", api_key.clone());
    rocket::build()
        .mount("/", routes![hello, hello_world])
        .mount("/api", routes![api::auth::authenticate, api::auth::register])
        .manage(api::state::Supabase{api_url, api_key, client})
        .attach(api::cors::Cors)
        .launch().await?;
    Ok(())
}
