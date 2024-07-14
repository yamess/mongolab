pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Mongodb error. Cause: {0}")]
    MongoError(mongodb::error::Error),
    #[error("Failed to create collection. Cause: {0}")]
    OtherError(String),
}