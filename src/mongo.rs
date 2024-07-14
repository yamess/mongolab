use std::fmt::Debug;
use mongodb::{Client, Collection, IndexModel};
use mongodb::bson::doc;
use mongodb::error::{BulkWriteError, ErrorKind};
use mongodb::options::{IndexOptions, InsertManyOptions};
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::configs::MongoDbConfig;
use crate::errors::{Error, Result};

#[derive(Debug, Clone)]
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

    pub async fn create_index<P>(&self, collection: &str, index: &str) -> Result<()>
    where
        P: Serialize + DeserializeOwned + Sync + Send + Clone + Unpin,
    {
        let collection: Collection<P> = self.get_collection(collection);
        let index_option = IndexOptions::builder().unique(true).build();
        let index_model = IndexModel::builder()
            .keys(doc! {index: 1})
            .options(index_option)
            .build();
        let result = collection.create_index(index_model).await;
        match result {
            Ok(_) => {
                log::debug!("Unique Index {} created successfully", index);
                Ok(())
            }
            Err(e) => Err(Error::OtherError(e.to_string())),
        }
    }

    pub async fn check_connection(&self) -> Result<()> {
        match self.client.list_database_names().await {
            Ok(_) => {
                log::info!("MongoDB connection established successfully");
                Ok(())
            },
            Err(e) => {
                log::error!("MongoDB connection failed: {:?}", e);
                Err(Error::MongoError(e))
            },
        }
    }

    pub fn get_collection<P>(&self, collection: &str) -> Collection<P>
    where
        P: Serialize + DeserializeOwned + Sync + Send + Clone,
    {
        self.client.database(&self.db_name).collection(collection)
    }

    pub async fn add<P>(&self, collection: &str, payload: &P) -> Result<String>
    where
        P: Serialize + DeserializeOwned + Sync + Send + Clone,
    {
        let collection: Collection<P> = self.get_collection(collection);
        let result = collection
            .insert_one(payload.clone())
            .await
            .map_err(|e| Error::MongoError(e))
            .map(|res| {
                let re = res.inserted_id.as_str().unwrap_or_default().to_string();
                re
            })?;
        Ok(result)
    }

    pub async fn bulk_add<P>(&self, collection: &str, payload: Vec<P>) -> Result<()>
    where P: Serialize + DeserializeOwned + Sync + Send + Clone + Debug
    {
        let collection: Collection<P> = self.get_collection(collection);

        let mut data = Vec::new();
        for p in payload.iter() {
            match collection.insert_one_model(p) {
                Ok(d) => data.push(d),
                Err(e) => {
                    log::error!("Bulk insert failed: {:?}", e);
                    return Err(Error::MongoError(e));
                }
            }
        }

        let mut total_failed = 0;

        match self.client.bulk_write(data).ordered(false).await {
            Ok(_) => {
                log::info!("Bulk insert successful");
                Ok(())
            },
            Err(e) => {
                if let ErrorKind::BulkWrite(BulkWriteError { ref write_errors, .. }) = *e.kind {
                    for (index, error) in write_errors {
                        total_failed += 1;
                        if error.code == 11000 {
                            log::error!("Failed to insert: {:?}. Cause: Duplicate key error",
                                payload[*index]);
                        } else {
                            log::error!("Failed to insert: {:?}. Cause: {:?}",payload[*index],
                                error.message);
                        }
                    }
                } else {
                    log::error!("Bulk insert failed: {:?}", e);
                    return Err(Error::MongoError(e))
                }
                log::info!("Total inserted: {}", payload.len() - total_failed);
                Ok(())
            }
        }
    }

    pub async fn bulk_update<P>(&self, collection: &str, payload: Vec<P>) -> Result<()>
    where P: Serialize + DeserializeOwned + Sync + Send + Clone + Debug
    {
        let collection: Collection<P> = self.get_collection(collection);

        let mut data = Vec::new();
        for p in payload.iter() {
            let filter = doc! {"email": p.email.clone()};
            match collection.replace_one_model(filter, p) {
                Ok(d) => data.push(d),
                Err(e) => {
                    log::error!("Bulk update failed: {:?}", e);
                    return Err(Error::MongoError(e));
                }
            }
        }

        let mut total_failed = 0;

        match self.client.bulk_write(data).ordered(false).await {
            Ok(_) => {
                log::info!("Bulk update successful");
                Ok(())
            },
            Err(e) => {
                if let ErrorKind::BulkWrite(BulkWriteError { ref write_errors, .. }) = *e.kind {
                    for (index, error) in write_errors {
                        total_failed += 1;
                        log::error!("Failed to update: {:?}. Cause: {:?}", payload[*index],
                            error.message);
                    }
                } else {
                    log::error!("Bulk update failed: {:?}", e);
                    return Err(Error::MongoError(e))
                }
                log::info!("Total updated: {}", payload.len() - total_failed);
                Ok(())
            }
        }
    }
}