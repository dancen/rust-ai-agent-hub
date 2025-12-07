use axum::extract::Path;
use serde::Serialize;
use axum::{extract::State, response::{Html, IntoResponse, Json}};
use crate::services::agent_service::AgentService;
use std::sync::Arc;


#[derive(Serialize)]
pub struct Message {
    pub message: String,
}

pub async fn root() -> Json<Message> {        
    let msg = Message {
        message: "Hello, World!".to_string(),
    };
    Json(msg)
}

pub async fn greet(Path(name): Path<String>) -> Json<Message> {
    let msg = Message {
        message: format!("Hello, {}!", name),
    };
    Json(msg)
}

pub async fn health_check(State(service): State<Arc<AgentService>>) -> String {
    let row: (i64,) = sqlx::query_as("SELECT 1")
        .fetch_one(&service.pool) 
        .await
        .unwrap();

    format!("DB OK: {}", row.0)
}

pub async fn doc_handler() -> impl IntoResponse {
    let html_content = std::fs::read_to_string("src/static/doc.html").unwrap();
    Html(html_content)
}


