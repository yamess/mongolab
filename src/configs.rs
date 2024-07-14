use serde::Deserialize;




#[derive(Debug, Deserialize, Clone)]
pub struct MongoDbConfig {
    pub mongo_uri: String,
    pub mongo_db: String,
}

impl Default for MongoDbConfig {
    fn default() -> Self {
        Self {
            mongo_uri: "mongodb://admin:secret@0.0.0.0:27017".to_string(),
            mongo_db: "lab".to_string(),
        }
    }
}

impl MongoDbConfig {
    pub fn new() -> Self {
        dotenv::dotenv().ok();
        envy::from_env::<Self>().unwrap_or_default()
    }
}