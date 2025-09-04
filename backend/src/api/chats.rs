use mongodb::bson::doc;
use mongodb::Database;
use rocket::{get, State};
use rocket::serde::json::Json;
use crate::AuthenticatedUser;
use crate::models::message::Message;
//use crate::db::chats::get_public_chats_db;
use crate::services::mock_service;

#[get("/get-public-chats")]
pub async fn get_public_chats(auth_user: AuthenticatedUser, db: &State<Database>) -> Result<Json<Vec<Message>>, String> {
    //let messages = get_public_chats_db(db).await;
    let messages = mock_service::get_public_mock_messages();
    Ok(Json(messages))
}