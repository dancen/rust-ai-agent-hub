use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct ApiUser {
    pub id: u64,
    pub username: String,
    pub api_key: String,
    pub rate_limit: u32,
    pub ttl: u32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct AiAgent {
    pub id: u64,
    pub token: String,
    pub user_id: u64,
    pub name: String,
    pub data: String, 
    pub ttl: u32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
