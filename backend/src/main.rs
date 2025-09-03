#[macro_use] extern crate rocket;
mod api;
mod db;
mod models;
mod services;

use mongodb::{Client, options::ClientOptions};

use rocket::request::{FromRequest, Outcome};
use rocket::{Request, http::Status};
use crate::services::auth::verify_jwt;

pub struct AuthenticatedUser {
    pub user_id: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(auth_header) = request.headers().get_one("Authorization") {
            if auth_header.starts_with("Bearer ") {
                let token = &auth_header[7..]; // strip "Bearer "

                if let Ok(claims) = verify_jwt(token) {
                    return Outcome::Success(AuthenticatedUser {
                        user_id: claims.sub,
                    });
                }
            }
        }

        Outcome::Error((Status::Unauthorized, ()))
    }
}


#[launch]
async fn rocket() -> _ {
    let client_uri = "mongodb://user:pass@127.0.0.1:27017";
    let options = ClientOptions::parse(client_uri).await.expect("Failed to parse client options");
    let client = Client::with_options(options).expect("Failed to initialize MongoDB client");

    let db = client.database("db");

    rocket::build()
        .manage(db)
        .mount("/", routes![api::auth::login, api::chats::get_public_chats, api::health::health])
}