mod todo;
use axum::{
    response::Html, routing::{delete, get, post}, serve, Router
};
use eyre::Result;
use todo::{add_todo, get_todo, remove_todo, AppState};
use tokio::net::TcpListener;
type State = AppState;

#[tokio::main]
async fn main() -> Result<()> {
    let state = State::new();
    let listener = TcpListener::bind("0.0.0.0:7879")
        .await
        .expect("Failed to bind Ip ");
    println!("Server serving on http://localhost:7879");
    let app: Router = Router::new()
        .route("/", get(index))
        .route("/todos", get(get_todo))
        .route("/todos", post(add_todo))
        .route("/todos/{id}", delete(remove_todo))
        .fallback(|| async {r#"<h1>PAGE NOT FOUND</h1><br><h2>404</h2>"#})
        .with_state(state);
    serve(listener, app).await.expect("Failed to start server");
    Ok(())
}

async fn index() -> Html<String> {
    Html(std::include_str!("../public/index.html").to_string())
}
