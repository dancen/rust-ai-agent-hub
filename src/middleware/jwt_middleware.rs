use axum::{
    http::Request,
    middleware::Next,
    response::Response,
    extract::State,
};
use crate::services::agent_service::AgentService;
use std::sync::Arc;
use crate::jwt::verify_jwt;
use axum::http::StatusCode;

pub async fn jwt_middleware<B>(
    State(_service): State<Arc<AgentService>>,
    req: Request<B>,
    next: Next<B>,    
) -> Result<Response, (StatusCode, String)>
where
    B: Send,
{
    let auth_header = req
        .headers()
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or((StatusCode::UNAUTHORIZED, "Missing Authorization header".into()))?;

    if !auth_header.starts_with("Bearer ") {
        return Err((StatusCode::UNAUTHORIZED, "Invalid Authorization format".into()));
    }

    let token = auth_header.trim_start_matches("Bearer ").trim();
    verify_jwt(token)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid or expired JWT".into()))?;

    Ok(next.run(req).await)
}
