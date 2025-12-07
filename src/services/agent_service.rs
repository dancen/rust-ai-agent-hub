use sqlx::MySqlPool;
use crate::models::{AiAgent, ApiUser};
use rand::{distributions::Alphanumeric, Rng};
use crate::constants::*;
use chrono::{Utc, Duration};


pub struct AgentService {
    pub pool: MySqlPool,
}

impl AgentService {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }   

    pub async fn get_user_by_api_key(&self, api_key: &str) -> Result<ApiUser, sqlx::Error> { sqlx::query_as::<_, ApiUser>("SELECT * FROM api_user WHERE api_key = ?") .bind(api_key) .fetch_one(&self.pool) .await }

    pub async fn create_agent(&self, user: &ApiUser, name: &str, data: &str, ttl: Option<u32>) -> Result<String, sqlx::Error> {
       
    let token: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(64) 
            .map(char::from)
            .collect();


        sqlx::query!(
            "INSERT INTO ai_agent (token, user_id, name, data, ttl, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, NOW(), NOW())",
            token,
            user.id,
            name,
            data,
            ttl.unwrap_or(AGENT_TIME_EXPIRATION)
        )
        .execute(&self.pool)
        .await?;

        Ok(token)
    }

    pub async fn log_audit(&self, user_id: u64, action: &str, target_table: &str, target_id: u64, description: &str, ip: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO audit_log (user_id, action_type, target_table, target_id, description, ip_address, created_at)
             VALUES (?, ?, ?, ?, ?, ?, NOW())",
            user_id,
            action,
            target_table,
            target_id,
            description,
            ip
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
    

    pub async fn get_agent_by_user(
            &self,
            token: &str,
            ) -> Result<AiAgent, sqlx::Error> {
            sqlx::query_as::<_, AiAgent>(
            "SELECT * FROM ai_agent WHERE token = ?"
            )
            .bind(token)
            .fetch_one(&self.pool)
            .await
            }
    
    pub async fn purge_tokens(&self) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(
            "
            DELETE FROM ai_agent
            WHERE NOW() > DATE_ADD(created_at, INTERVAL ttl SECOND)
            "
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }

    pub async fn get_user_by_username_and_api_key(&self, username: &str, api_key: &str) -> Result<ApiUser, sqlx::Error> {
        sqlx::query_as::<_, ApiUser>(
        "SELECT * FROM api_user WHERE username = ? AND api_key = ?"
        )
        .bind(username)
        .bind(api_key)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn check_rate_limit(&self, api_key: &str) -> Result<bool, sqlx::Error> {

        let now = Utc::now().naive_utc();
        let mut tx = self.pool.begin().await?;

        let user = sqlx::query!(
            r#"SELECT id, rate_limit, rate_count, rate_reset_at, ttl FROM api_user WHERE api_key = ? FOR UPDATE"#,
            api_key
        )
        .fetch_one(&mut *tx)
        .await?;

        let mut rate_count = user.rate_count.unwrap_or(0);
        let rate_limit = user.rate_limit.unwrap_or(1000);
        let rate_reset_at = user.rate_reset_at.unwrap_or(now);
        let ttl_seconds = user.ttl.unwrap_or(3600) as i64;
        
        if now >= rate_reset_at {
            rate_count = 0;
        }

        if rate_count >= rate_limit {
            tx.rollback().await?;
            return Ok(false);
        }

        let new_rate_count = rate_count + 1;

        let new_reset_at = if now >= rate_reset_at {
            now + Duration::seconds(ttl_seconds)
        } else {
            rate_reset_at
        };

      
        sqlx::query!(
            r#"UPDATE api_user SET rate_count = ?, rate_reset_at = ? WHERE id = ?"#,
            new_rate_count,
            new_reset_at,
            user.id
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(true)
    }


}
