use std::net::SocketAddr;

use axum::{
    Router,
    routing::{get, post},
    response::{Html, IntoResponse}, Json, http::StatusCode,
};
use serde::{Serialize, Deserialize};
use rand::Rng;

#[tokio::main]
async fn main() {
    let app = create_app();

    let addr = SocketAddr::from(([0,0,0,0],3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}



// Routing
fn create_app() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/user", post(response_json))
}



// handlers
async fn index() -> Html<&'static str> {
    Html("
        <h1>Hello World</h1>
    ")
}

async fn response_json(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    let user = User::new(payload);

    (StatusCode::CREATED, Json(user))
} 



// Json-Response
#[derive(Debug, Serialize)]
pub struct User {
    user_id: u32,
    user_name: String,
}

impl User {
    pub fn new(payload: CreateUser) -> Self {
        Self {
            user_id: generate_number(),
            user_name: payload.user_name,
            
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    user_name: String,
}

pub fn generate_number()-> u32 {
    let number = rand::thread_rng().gen_range(0 .. 101);
    number
}