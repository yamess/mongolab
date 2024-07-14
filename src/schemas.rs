use mongodb::bson::{Uuid};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub struct User {
    #[serde(rename(serialize = "_id", deserialize = "_id"))]
    pub id: String,
    pub email: String,
    pub created_at: i64,
}

impl User {
    pub fn new(email: String) -> Self {
        User {
            id: Uuid::new().to_string(),
            email,
            created_at: chrono::Utc::now().timestamp(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateUser {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateUser {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub created_at: i64,
}