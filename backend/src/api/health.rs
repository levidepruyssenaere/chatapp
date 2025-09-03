use rocket::http::Status;

#[get("/health")]
pub async fn health() -> Status {
    Status::Ok
}