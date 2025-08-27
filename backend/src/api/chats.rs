use mongodb::bson::doc;
use mongodb::Database;
use rocket::{get, State};
use rocket::futures::TryStreamExt;
use rocket::serde::json::Json;
use crate::models::message::Message;
use chrono::{Local};

#[get("/")]
pub async fn get_public_chats(db: &State<Database>) -> Result<Json<Vec<Message>>, String> {
    let collection = db.collection::<Message>("public_messages");
    let dt = Local::now();
    let message: Message = Message {
        id: None,
        message: "Sup dawg".to_string(),
        author: "Jan Deploige".to_string(),
        timestamp: dt.to_string()
    };
    collection.insert_one(message)
        .await
        .map_err(|e| e.to_string())?;
    let cursor = collection.find(doc! {})
        .await
        .map_err(|e| e.to_string())?;
    let messages: Vec<Message> = cursor
        .try_collect()
        .await
        .map_err(|e| e.to_string())?;
    Ok(Json(messages))
}