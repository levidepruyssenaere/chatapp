use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use serde::Deserialize;
use crate::services::auth::generate_jwt;

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[post("/login", format = "json", data = "<login>")]
pub fn login(login: Json<LoginRequest>) -> Result<String, status::Custom<String>> {
    if login.email != "jan@freedombox.be" {
        return Err(status::Custom(Status::Unauthorized, "Unauthorized".into()));
    }

    // Normally you'd verify credentials here
    Ok(generate_jwt(&login.email))
}