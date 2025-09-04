use mongodb::Database;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::State;
use serde::Deserialize;
use crate::AuthenticatedUser;
use crate::services::auth::{authenticate};

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[post("/login", format = "json", data = "<login>")]
pub async fn login(auth_user: AuthenticatedUser, db: &State<Database>, login: Json<LoginRequest>) -> status::Custom<String> {
    match authenticate(db, &login.email, &login.password).await {
        Ok(token) => status::Custom(Status::Ok, token),
        Err(e) => status::Custom(Status::InternalServerError, e)
    }
}