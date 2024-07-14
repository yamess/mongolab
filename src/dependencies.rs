use crate::configs::MongoDbConfig;
use crate::mongo::DocumentStore;

#[derive(Clone)]
pub struct AppState{
    pub config: MongoDbConfig,
    pub document_store: DocumentStore,
}


impl AppState {
    pub async fn new() -> Self {
        dotenv::dotenv().ok();
        let config = MongoDbConfig::new();
        let document_store = DocumentStore::new(&config).await;
        Self {
            config,
            document_store
        }
    }
}