use axum::{
    Json,
    extract::State,
    http::StatusCode,
};
use serde_json::json;
use std::sync::Arc;
use axum::extract::ConnectInfo;
use std::net::SocketAddr;
use crate::constants::*;

use crate::services::agent_service::AgentService;


#[derive(serde::Deserialize)]
pub struct PublishRequest {
    pub name: String,
    pub data: String,
    pub ttl: Option<u32>,
}

// POST /secure/agent/publish
pub async fn publish_agent(
    State(service): State<Arc<AgentService>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<PublishRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    
    let api_key = headers
        .get("x-api-key")
        .and_then(|v| v.to_str().ok())
        .ok_or((StatusCode::UNAUTHORIZED, "Missing API Key".into()))?;
       
    let user = service.get_user_by_api_key(api_key).await
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid API Key".into()))?;
     
    let token = service.create_agent(&user, &payload.name, &payload.data, payload.ttl)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

     let ip_address = addr.ip().to_string();

    service.log_audit(user.id, AUDIT_CREATE_TOKEN, AUDIT_AGENT, 0, &format!("Token generato: {}", token), &ip_address)
        .await
        .ok(); 


    Ok(Json(json!({ "status": "ok", "token": token })))
}

// GET /secure/agent/:token
pub async fn get_agent(
    State(service): State<Arc<AgentService>>,
    axum::extract::Path(token): axum::extract::Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
   
    
    let agent = service.get_agent_by_user(&token)
        .await
        .map_err(|_| (StatusCode::NOT_FOUND, "Agent Data Expired".into()))?;

    Ok(Json(json!({
        "token": agent.token,
        "name": agent.name,
        "data": agent.data,
        "ttl": agent.ttl,
        "created_at": agent.created_at
    })))
}
