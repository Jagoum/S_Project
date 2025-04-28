use dotenvy::{dotenv, var};
use sqlx::{PgPool, Result};

use crate::config::urls::Url;

pub async fn store_url(remote_urls: Url) {
    let pool = connect_to_database().await.unwrap();
    sqlx::query!(
        "INSERT INTO urls_tests (long_url, short_url) VALUES($1,$2)",
        remote_urls.long_url(),
        remote_urls.short_url()
    )
    .execute(&pool)
    .await
    .expect("Failed to insert url into database");
}

pub async fn connect_to_database() -> Result<PgPool, sqlx::Error> {
    dotenv().ok();
    let durl = var("DATABASE_URL").expect("Failed to get database url");
    let pool = PgPool::connect(&durl).await?;
    Ok(pool)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::urls::Url;
    use sqlx::{Executor, PgPool};

    #[tokio::test]
    async fn test_store_url() {
        // Set up a test database connection
        let pool = connect_to_database().await.unwrap();

        // Create a test URL
        let long_url = "https://example.com".to_string();
        let short_url = "exmpl".to_string();
        let test_url = Url::new(long_url, short_url);

        // Call the function to store the URL
        store_url(test_url.clone()).await;

        // Verify the URL was inserted into the database
        let row = sqlx::query!(
            "SELECT long_url, short_url FROM test_urls WHERE short_url = $1",
            test_url.short_url()
        )
        .fetch_one(&pool)
        .await
        .expect("Failed to fetch URL from database");

        assert_eq!(row.long_url, test_url.long_url());
        assert_eq!(row.short_url, test_url.short_url());

        // Clean up the test database
        pool.execute("DROP TABLE urls_tests")
            .await
            .expect("Failed to drop test table");
    }
}
