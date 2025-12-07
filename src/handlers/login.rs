use axum::{
extract::State,
http::StatusCode,
response::Json,
};
use serde::{Serialize};
use chrono::{Utc, Duration};
use std::sync::Arc;
use crate::services::agent_service::AgentService;
use crate::jwt::create_jwt;

#[derive(Serialize)]
pub struct LoginResponse {
pub token: String,
pub expires_at: String,
}

// POST /login
pub async fn login_handler(
State(service): State<Arc<AgentService>>,
headers: axum::http::HeaderMap,
) -> Result<Json<LoginResponse>, (StatusCode, String)> {

    let api_key = headers
        .get("x-api-key")
        .and_then(|v| v.to_str().ok())
        .ok_or((StatusCode::UNAUTHORIZED, "Missing API Key".into()))?;

    let username = headers
        .get("username")
        .and_then(|v| v.to_str().ok())
        .ok_or((StatusCode::UNAUTHORIZED, "Missing Username".into()))?;

    let user = service.get_user_by_username_and_api_key(&username, &api_key)
    .await
    .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid username or API key".into()))?;

    let token = create_jwt(user.id, 3600);
    let expires_at = Utc::now() + Duration::hours(1);

    Ok(Json(LoginResponse {
        token,
        expires_at: expires_at.to_rfc3339(),
    }))
}
