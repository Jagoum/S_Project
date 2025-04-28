use axum::{Router, routing::get};
use eyre::{self, Result};
use std::env;
use tokio::net::TcpListener;

pub fn shortened_url(mut id: u128) -> String {
    let mut short_url = String::new();
    let map: Vec<char> = vec![
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
        'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];

    while id > 1 {
        short_url.push(map[id as usize % 62]);
        id = id / 62;
    }
    unsafe {
        short_url.as_bytes_mut().reverse();
    }
    short_url
}


pub async fn start_server() -> Result<()> {
    let app: Router =
        Router::new().route("/health_check", get(async || "hello . Am in good health"));
    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind ip");
    axum::serve(listener, app).await?;
    Ok(())
}

pub fn get_number() -> u128 {
    let args: Vec<String> = env::args().collect();

    let id: u128 = match args[1].trim().parse() {
        Ok(num) => num,
        Err(err) => {
            println!("Please enter a valid number.\nError : {err}");
            0
        }
    };
    id
}
