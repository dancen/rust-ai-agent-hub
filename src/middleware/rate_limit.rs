use axum::{
    extract::State,
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use crate::services::agent_service::AgentService;
use std::sync::Arc;
use http::Request;

pub async fn rate_limit<B>(
    State(service): State<Arc<AgentService>>,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, (StatusCode, String)>
where
    B: Send,
{   
    let api_key = req
        .headers()
        .get("x-api-key")
        .and_then(|v| v.to_str().ok())
        .ok_or((StatusCode::UNAUTHORIZED, "Missing APi Key header".into()))?;

    let allowed = service.check_rate_limit(api_key)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Rate limit check failed".into()))?;

    if !allowed {
        return Err((StatusCode::TOO_MANY_REQUESTS, "Rate limit exceeded".into()));
    }

    Ok(next.run(req).await)
}
