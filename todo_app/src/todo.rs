use std::{fs::{self, OpenOptions}, io::Read};
use serde_json::{Map, Value, json};
pub enum status {
    Todo,
    Pending,
    Done,
}

pub fn add_todo(id: String, title: String,  items: &mut  Map<String, Value>) ->() {
    items.insert(id.clone(), json!(title));
    write_to_file("./task.json", items);
    println!(r#"Adding task : "{title}" with id {id}"#);    
}

pub fn remove_todo(id: String, mut items: &mut Map<String, Value>) {
    items.remove(&id);
    write_to_file("./task.json", items);
    println!("Remove task with id {id}");

}

pub fn get_todo(items: &mut Map<String, Value>) {
    println!("List Of Available Tasks");
    for (id, title) in items {
        println!("{id}: {title}");
    }
}
// this function is to save the different tasks in my file
pub fn write_to_file(file_name: &str, items: &mut Map<String, Value>) {
    let new_data = json!(items);
    fs::write(file_name.to_string(), new_data.to_string()).expect("Failed to write to file");
}

pub fn read_file(file_name: &str) -> Map<String, Value> {
    let mut file = match OpenOptions::new().read(true).write(true).create(true).open(file_name) {
        Ok(file) => {file},
        Err(err) => {eprintln!("Could not Open or even create file");return Err("()").unwrap();},
    };

    let mut file_data = String::new();
    file.read_to_string(&mut file_data).unwrap();

    let json: Value = match serde_json::from_str(&file_data) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Failed To Parse Data {}", e);
            return Map::default();
        }
    };
    let state = json.as_object().unwrap().clone();
    state
}
