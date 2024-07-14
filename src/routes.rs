use actix_web::{get, HttpResponse, post, Responder, web};
use actix_web::web::Json;
use crate::dependencies::AppState;
use crate::schemas::{CreateUser, User};

#[get("/health")]
async fn health() -> impl Responder {
    log::info!("Health check");
    HttpResponse::Ok().json("OK")
}

#[post("/users")]
async fn add_user(state: web::Data<AppState>, user: Json<CreateUser>) -> impl Responder {
    let doc_store = state.document_store.clone();
    let user = user.into_inner();
    let user = User::new(user.email);
    log::info!("Adding user: {:?}", user);
    let result = doc_store.add("users", &user).await;
    match result {
        Ok(id) => {
            log::info!("User added successfully");
            HttpResponse::Ok().json(id)
        },
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

#[post("/users/bulk")]
async fn add_users(state: web::Data<AppState>, users: Json<Vec<CreateUser>>) -> impl Responder {
    let doc_store = state.document_store.clone();
    let users = users.into_inner();
    let users: Vec<User> = users.into_iter().map(|u| User::new(u.email)).collect();
    let result = doc_store.bulk_add("users", users).await;
    match result {
        Ok(_) => {
            log::info!("Users added successfully");
            HttpResponse::Ok().json("Users added successfully")
        },
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}