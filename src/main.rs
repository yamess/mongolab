use actix_web::{App, HttpServer, web};
use mongodb::{Collection, IndexModel};
use mongodb::bson::doc;
use mongodb::options::IndexOptions;
use crate::dependencies::AppState;
use crate::logger::init_logger;
use crate::schemas::User;

mod mongo;
mod configs;
mod schemas;
mod logger;
mod routes;
mod dependencies;
mod errors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger(
        "mongolab",
        "debug",
        "debug",
        "/tmp/logs/mongolab.log",
    );

    let state = AppState::new().await;
    let app_state = web::Data::new(state);

    log::info!("Starting server at http://0.0.0.0:8080");

    let doc_store = app_state.document_store.clone();
    doc_store.check_connection().await.unwrap();
    doc_store.create_index::<User>("users", "email").await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(routes::health)
            .service(routes::add_user)
            .service(routes::add_users)
    })
        .bind("0.0.0.0:8080")?
        .workers(1)
        .run()
        .await
}
