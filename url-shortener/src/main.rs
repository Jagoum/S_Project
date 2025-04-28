mod config;
use config::logic;
use logic::{get_number, shortened_url, start_server};

#[tokio::main]
async fn main() {
    start_server().await.unwrap();
    let short_url = shortened_url(get_number());
    println!("\nNew Url: {}", short_url);
}


// fn unshorten_url(short_url: String) -> usize {
//     let mut id = 0;
//     let last: u32 = (short_url.len() - 1) as u32;
//     for i in 0..short_url.len() {
//         if 'a' as u8 <= short_url.as_bytes()[i] && short_url.as_bytes()[i] <= 'z' as u8 {
//             let index =  short_url.as_bytes()[i] - 'a' as u8 + 10;
//             id = id + 62_u8.pow(last - i as u32) * index
//         }
//         if 'A' as u8 <= short_url.as_bytes()[i] && short_url.as_bytes()[i] <= 'Z' as u8 {
//             let index =  short_url.as_bytes()[i] - 'A' as u8 + 36;
//             id = id + 62_u8.pow(last-i as u32) * index
//         }
//         if '0' as u8 <= short_url.as_bytes()[i] && short_url.as_bytes()[i] <= '9' as u8 {
//             let index =  short_url.as_bytes()[i] - '0' as u8;
//             id = id + 62_u8.pow(last-i as u32) * index
//         }
//     }
//     id.into()
// }
