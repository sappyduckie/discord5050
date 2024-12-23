use rand::Rng;
use reqwest::Client;
use tokio;

fn hash_gen(length: usize) -> String {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    let hex_chars: &str = "0123456789abcdef";
    let mut hex_string: String = String::with_capacity(length);

    // generate hex char, loop 'length' times
    for _ in 0..length {
        let index: usize = rng.gen_range(0..16);
        hex_string.push(hex_chars.chars().nth(index).unwrap());
    }

    hex_string
}

fn pattern_generator() -> String {
    // randomized variables
    let guild_id: u64 = rand::thread_rng().gen_range(1..=999999999999999999); //18 digits + 1
    let one_two: u8 = rand::thread_rng().gen_range(1..=2); //1 digit: one or two
    let channel_id: u64 = rand::thread_rng().gen_range(1..=9999999999999999); //16 digits + 3
    let first_hex_8: String = hash_gen(8); //8 hex chars
    let second_hex_8: String = hash_gen(8); //8 hex chars
    let third_hex_8: String = hash_gen(8); //8 hex chars
    let hex_64: String = hash_gen(64); //64 hex chars
    let format_width: u16 = rand::thread_rng().gen_range(1..=999); //3 digits
    let format_height: u16 = rand::thread_rng().gen_range(1..=999); //3 digits

    // final url
    let url: String = format!("https://media.discordapp.net/attachments/1{}/13{}{}/img.png?ex={}&is={}&hm={}&=&format=webp&quality=lossless{}&width={}&height={}", guild_id, one_two, channel_id, first_hex_8, second_hex_8, third_hex_8, hex_64, format_width, format_height);

    return url;
}

async fn url_valid(url: &str) -> Result<bool, reqwest::Error> {
    let client: Client = Client::new();
    let response: reqwest::Response = client.get(url).send().await?;
    let status: reqwest::StatusCode = response.status();
    Ok(status.is_success())
}

#[tokio::main]
async fn main() {
    let mut iteration: u128 = 0;
    loop {
        iteration += 1;
        let pattern: String = pattern_generator();
        if let Ok(valid) = url_valid(&pattern).await {
            if valid {
                println!("Iteration: {} Valid URL: {}", iteration, pattern);
                break;
            } else {
                println!("Iteration: {} Invalid URL: {}", iteration, pattern);
            }
        }
    }
}
