use mongodb::Database;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::State;
use serde::Deserialize;
use crate::services::auth::{authenticate, register_user};

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    email: String,
    username: String,
    password: String,
    password_verify: String,
}

#[post("/login", format = "json", data = "<login>")]
pub async fn login(db: &State<Database>, login: Json<LoginRequest>) -> status::Custom<String> {
    match authenticate(db, &login.email, &login.password).await {
        Ok(token) => status::Custom(Status::Ok, token),
        Err(e) => status::Custom(Status::Unauthorized, e)
    }
}

#[post("/register", format = "json", data = "<register>")]
pub async fn register(db: &State<Database>, register: Json<RegisterRequest>) -> status::Custom<String> {
    if register.password != register.password_verify {
        return status::Custom(Status::InternalServerError, String::from("Passwords do not match!"));
    }
    match register_user(db, &register.email, &register.username, &register.password).await {
        Ok(user_id) => status::Custom(Status::Ok, user_id),
        Err(e) => status::Custom(Status::InternalServerError, e)
    }
}