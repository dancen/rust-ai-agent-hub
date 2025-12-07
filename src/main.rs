mod db;
mod models;
mod constants;
mod middleware;
mod handlers;
mod services;
mod jwt;
mod processes;

use axum::{ Router,
    routing::{get, post},
    middleware::from_fn_with_state };
use crate::handlers::agent::{publish_agent, get_agent};
use crate::handlers::misc::{root, greet, health_check, doc_handler};
use crate::services::agent_service::AgentService;
use crate::middleware::jwt_middleware::jwt_middleware;
use crate::middleware::rate_limit::rate_limit;
use crate::handlers::login::login_handler; 


use processes::token_purge::start_token_purge_task;

use std::sync::Arc;
use std::net::SocketAddr;


#[tokio::main]
async fn main() {
         
     let pool = db::init_db().await;
     let service = Arc::new(AgentService::new(pool));
   
     start_token_purge_task(service.clone(), 20);

     let secure_routes = Router::new()
    .route("/agent/publish", post(publish_agent))
    .route("/retrieve/:token", get(get_agent))
    .layer(from_fn_with_state(service.clone(), jwt_middleware))
    .layer(from_fn_with_state(service.clone(), rate_limit)); 

    let app = Router::new()
        .route("/doc", get(doc_handler))
        .route("/login", post(login_handler))
        .route("/", get(root))
        .route("/greet/:name", get(greet))
        .route("/health", get(health_check))
        .nest("/secure", secure_routes)
        .with_state(service.clone());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();

}
