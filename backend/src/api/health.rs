use rocket::http::Status;

#[get("/")]
pub async fn health() -> Status {
    Status::Ok
}