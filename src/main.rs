//dotenv
use dotenv::dotenv;
//server
use axum::{
    http::Method,
    routing::{delete, get, post, put},
    Router,
};
use tower_http::cors::{CorsLayer, Origin};
mod api_task;
mod db;

#[tokio::main]
async fn main() {
    dotenv().ok();
    db::connect().await;
    let app = Router::new()
        .route("/", get(api_task::get_tasks))
        .route("/task/get/:id", get(api_task::get_task))
        .route("/task/create", post(api_task::create_task))
        .route("/task/update/:id", put(api_task::update_task))
        .route("/task/delete/:id", delete(api_task::delete_task))
        .layer(
            // see https://docs.rs/tower-http/latest/tower_http/cors/index.html
            CorsLayer::new()
                .allow_origin(Origin::exact("*".parse().unwrap()))
                .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE]),
        );
    println!("{}", "Server on port 3000");
    
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
