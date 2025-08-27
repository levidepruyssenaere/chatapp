#[macro_use] extern crate rocket;
mod api;
mod db;
mod models;

use mongodb::{Client, options::ClientOptions};

#[launch]
async fn rocket() -> _ {
    let client_uri = "mongodb://user:pass@127.0.0.1:27017";
    let options = ClientOptions::parse(client_uri).await.expect("Failed to parse client options");
    let client = Client::with_options(options).expect("Failed to initialize MongoDB client");

    let db = client.database("db");

    rocket::build()
        .manage(db)
        .mount("/", routes![api::health::health])
        .mount("/public-chats", routes![api::chats::get_public_chats])
}