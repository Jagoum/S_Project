use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let id: usize = match args[1].trim().parse() {
        Ok(num) => num,
        Err(err) => {
            println!("Please enter a valid number.\nError : {err}");
            0
        }
    };
    let short_url = shortened_url(id);
    let id = unshorten_url(short_url);
    println!("\nNew Url: {}",id);
}

fn shortened_url(mut id: usize) -> String {
    let mut short_url = String::new();
    let map: Vec<char> = vec![
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
        'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];

    while id > 1 {
        short_url.push(map[id % 62]);
        id = id / 62;
    }
    unsafe { short_url.as_bytes_mut().reverse(); }
    println!("The shortened Url is : {short_url}");
    short_url
}
fn unshorten_url(short_url: String) -> usize {
    let mut id = 0;
    for i in 0..short_url.len() {
        if 'a' as u8 <= short_url.as_bytes()[i] && short_url.as_bytes()[i] <= 'z' as u8 {
            id = id * 62 + short_url.as_bytes()[i] - 'a' as u8 + 10;
        }
        if 'A' as u8 <= short_url.as_bytes()[i] && short_url.as_bytes()[i] <= 'Z' as u8 {
            id = id * 62 + short_url.as_bytes()[i] - 'A' as u8 + 36;
        }
        if '0' as u8 <= short_url.as_bytes()[i] && short_url.as_bytes()[i] <= '9' as u8 {
            id = id * 62 + short_url.as_bytes()[i] - '0' as u8;
        }
    }
    id.into()
}
