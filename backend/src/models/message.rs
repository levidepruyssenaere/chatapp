use mongodb::bson::oid::ObjectId;
use rocket::serde::Serialize;
use serde::Deserialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub message: String,
    pub author: String,
    pub timestamp: String,
}