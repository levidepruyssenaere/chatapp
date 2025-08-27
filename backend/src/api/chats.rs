use mongodb::bson::doc;
use mongodb::Database;
use rocket::{get, State};
use rocket::serde::json::Json;
use crate::models::message::Message;
use crate::db::chats::get_public_chats_db;

#[get("/")]
pub async fn get_public_chats(db: &State<Database>) -> Result<Json<Vec<Message>>, String> {
    let messages = get_public_chats_db(db).await;
    Ok(messages)
}