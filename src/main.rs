mod mongo;
mod configs;
mod schemas;

#[tokio::main]
async fn main() {
    let config = configs::MongoDbConfig::new();
    let store = mongo::DocumentStore::new(&config).await;
    println!("{:?}", store);
}
