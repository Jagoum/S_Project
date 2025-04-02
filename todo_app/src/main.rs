use std::env::args;

use axum::{http::Response, Router};
use todo::{add_todo, get_todo, read_file, remove_todo};
mod todo;
fn main() {
    // let app = Router::new().route("/", method_router)
}



fn run_todo() {
    let mut items = read_file("./task.json");
    let args: Vec<String> = args().collect();
    let binding = String::from("0");
    let id = args.get(2).unwrap_or(&binding);
    let binding = String::from("value");
    let title = args.get(3).unwrap_or(&binding);

    let command = args.get(1).unwrap().as_str();

    match command {
        "add" => {
            let items = add_todo(id.to_string(), title.into(), &mut items);
            println!("Items: {items:?}")
        }
        "delete" => {
            let items = remove_todo(id.to_string(), &mut items);
            println!("Items: {items:?}")
        }
        "list" => {
            get_todo(&mut items);
        }
        _ => println!("Invalid Command"),
    }
}
