use axum::{
    extract::{Path, State},
    response::IntoResponse, Form
};
use chrono::Utc;
use futures::lock::Mutex;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

#[derive(Deserialize, Serialize, Clone)]
pub struct Todo {
    id: String,
    title: String,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub item: Arc<Mutex<HashMap<String, String>>>,
}
impl AppState {
    pub fn new() -> AppState {
        let item = Arc::new(Mutex::new(HashMap::new()));
        AppState { item }
    }
}


// Trying to extract path params to add a todo
pub async fn add_todo(State(app): State<AppState>, Form(item): Form<Todo>) -> impl IntoResponse {
    app.item
        .lock()
        .await
        .insert(item.id.clone(), item.title.clone());
    println!("{} : Task: {} with TaskId: {} added \n",Utc::now(), item.title, item.id);
    format!("Task: {} with TaskId: {} added AT {}\n", item.title, item.id, Utc::now())
}

pub async fn remove_todo(
    State(app): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    app.item.lock().await.remove(&id);
    println!("{} : Remove task with ID: {}\n",Utc::now(), id);
    format!("Remove task with ID: {} AT {}\n", id, Utc::now())
}

pub async fn get_todo(State(app): State<AppState>) -> impl IntoResponse {
    println!("List Of Available Tasks");
    let mut task = String::new();
    let app = app.item.try_lock().unwrap().clone();
    for (id, title) in app {
        print!("Task_Id: {}, Task: {}\n", id, title);
        task.push_str(&format!("Task_Id: {}, Task: {}\n", id, title));
    }
    format!("{}", task)
}
