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
#[derive(Serialize)]
struct Dataper{
    id: u64,
    username: String,
    nombre: String,
    apellido: String,
    email: String,
    telefono: String,
}

pub async fn get_frut() -> Json<Value> {
    let datauser = Dataper{
        id: 1,
        username: "undefined".to_string(),
        nombre: "jacobo".to_string(),
        apellido: "hernandez mendieta".to_string(),
        email: "jacobo@gmail.com".to_string(),
        telefono: "2471313141".to_string(),
    };
    Json(json!({ "data": datauser, "message": "Hello, world!" }))
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