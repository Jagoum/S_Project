use url_shortener::config::Config;
use url_shortener::routes::create_router;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = Config::new();
    
    // Set up database connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;
    
    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;
    
    // Create router
    let app = create_router(pool);
    
    // Start server
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
        .await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}