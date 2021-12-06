use serde_json::{Value, json};
use axum::{
    response::Json,
};
use axum::{
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
//let array_frutas = vec!["manzana", "pera", "uva"];

pub async fn get_frut() -> Json<Value> {
    Json(json!({ "data": 42, "message": "Hello, world!" }))
}

pub async fn get_frut_name() -> Json<Value> {
    Json(json!({ "data": "manzanas", "message": "Hello, world!" }))
}
pub async fn create_user(Json(payload): Json<CreateUser>,) -> impl IntoResponse {
    let user = User {
        id: 1337,
        username: payload.username,
    };
    (StatusCode::CREATED, Json(user))
}

pub async fn update_user(Json(payload): Json<CreateUser>,) -> impl IntoResponse {
    let user = User {
        id: 1337,
        username: payload.username,
    };
    (StatusCode::CREATED, Json(user))
}

pub async fn delete_user() -> impl IntoResponse {
    let response = json!({
        "status": "ok",
        "message": "User deleted",
        "error": false,
        "delete":true
    });
    (StatusCode::CREATED, Json(response))
}