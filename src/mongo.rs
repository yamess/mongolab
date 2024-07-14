use mongodb::{Client, Collection};
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::configs::MongoDbConfig;
use crate::configs::Result;

#[derive(Debug)]
pub struct DocumentStore {
    client: Client,
    db_name: String,
}

impl DocumentStore {
    pub async fn new(config: &MongoDbConfig) -> Self {
        let client = Client::with_uri_str(&config.mongo_uri).await.unwrap();
        Self {
            client,
            db_name: config.mongo_db.clone(),
        }
    }


    pub fn get_collection<P>(&self, collection: &str) -> Collection<P>
    where
        P: Serialize + DeserializeOwned + Sync + Send + Clone + Unpin,
    {
        self.client.database(&self.db_name).collection(collection)
    }

    pub async fn add<P>(&self, collection: &str, payload: &P) -> Result<String>
    where
        P: Serialize + DeserializeOwned + Sync + Send + Clone,
    {
        let collection: Collection<P> = self.get_collection(collection);
        let result = collection.insert_one(payload.clone()).await;
        match result {
            Ok(_) => Ok("Document added successfully".to_string()),
            Err(e) => Err(e.into()),
        }
    }
}