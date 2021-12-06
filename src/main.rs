use axum::{
    routing::{get, post,put,delete},
    Router,
};
mod api_server;
//use axum::response::Json;
//use rand::{thread_rng, Rng};
//use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/", get(api_server::get_frut))
    .route("/frut", get(api_server::get_frut_name))
    .route("/createuser",post(api_server::create_user))
    .route("/updateuser",put(api_server::update_user))
    .route("/deleteuser",delete(api_server::delete_user));
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
        
}


