use sqlx::PgPool;
use crate::{errors, models::Url};
use errors::Result;

#[derive(Debug, Clone)]
pub struct UrlRepository {
    pool: PgPool,
}

impl UrlRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    
    pub async fn create(&self, long_url: &str, short_url: &str) -> Result<Url> {
        let url = sqlx::query_as!(
            Url,
            r#"
            INSERT INTO urls (long_url, short_url)
            VALUES ($1, $2)
            RETURNING id, long_url, short_url, created_at
            "#,
            long_url,
            short_url
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(url)
    }
    
    pub async fn find_by_short_url(&self, short_url: &str) -> Result<Url> {
        let url = sqlx::query_as!(
            Url,
            r#"
            SELECT id, long_url, short_url, created_at
            FROM urls
            WHERE short_url = $1
            "#,
            short_url
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(url)
    }
}