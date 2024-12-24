use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use reqwest::Client;
use std::fs::OpenOptions;
use std::io::Write;
use tokio;

fn pattern_generator(rng: &mut StdRng) -> String {
    // MODIFY THIS BELOW --------//
    ///////////////////////////////
    // INPUT GUILD ID(s) HERE /////
    let guild_id: u64 = 1234567890;
    // INPUT GUILD IDs HERE ///////
    ///////////////////////////////
    ///////////////////////////////
    // INPUT CHANNEL ID(s) HERE ///
    let channel_id_list = vec![ ///
        "1234567890", /////////////
        "1234567890", /////////////
    ]; // INPUT CHANNEL IDs HERE //
    ///////////////////////////////
    // DO NOT MODIFY BELOW ------//

    // RAND CHANNEL ID
    let channel_id = choose_one(rng, &channel_id_list).unwrap(); // Unwrap safely since we know the list is not empty

    // GEN HEX STRINGS
    let first_hex_8: String = hash_gen(rng, 8); // 8 hex chars
    let second_hex_8: String = hash_gen(rng, 8); // 8 hex chars
    let third_hex_8: String = hash_gen(rng, 8); // 8 hex chars
    let hex_64: String = hash_gen(rng, 64); // 64 hex chars
    let format_width: u16 = rng.gen_range(1..=999); // 3 digits
    let format_height: u16 = rng.gen_range(1..=999); // 3 digits

    // FINAL URL
    let url: String = format!(
        "https://media.discordapp.net/attachments/{}/{}/image.png?ex={}&is={}&hm={}&=&format=webp&quality=lossless{}&width={}&height={}",
        guild_id, channel_id, first_hex_8, second_hex_8, third_hex_8, hex_64, format_width, format_height
    );

    return url;
}

fn hash_gen(rng: &mut StdRng, length: usize) -> String {
    let hex_chars: &str = "0123456789abcdef";
    let mut hex_string: String = String::with_capacity(length);

    // GEN HEX CHAR, LOOP 'LENGTH' TIMES
    for _ in 0..length {
        let index: usize = rng.gen_range(0..16);
        hex_string.push(hex_chars.chars().nth(index).unwrap());
    }
    hex_string
}

fn choose_one<'a, T>(rng: &mut StdRng, list: &'a [T]) -> Option<&'a T> {
    if list.is_empty() {
        return None; //prevent panic
    }
    let index: usize = rng.gen_range(0..list.len()); //gen rand index
    Some(&list[index]) //return ref to item
}

fn append_to_txt(file_path: &str, url: &str, channel_id: &str) {
    // OPEN FILE AND APPEND URL + CHANNEL
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)
        .expect("Unable to open or create file");
    // WRITE URL TO FILE
    if let Err(e) = writeln!(file, "{} from: {}", url, channel_id) {
        eprintln!("Error writing to file: {}", e);
    }
}

async fn url_valid(url: &str, channel_id: &str) -> Result<bool, reqwest::Error> {
    let client: Client = Client::new();
    let response: reqwest::Response = client.get(url).send().await?;
    let status: reqwest::StatusCode = response.status();
    // CHECK RESPONSE SUCCESS
    if status.is_success() {
        return Ok(true);
    }
    // GET RESPONSE AS TEXT
    match response.text().await {
        Ok(body) => {
            if body.contains("This content is no longer available.") {
                return Ok(false); //return invalid if 404
            } else {
                append_to_txt("valid.txt", &url, &channel_id);
                return Ok(true); //return valid if not 404
            }
        }
        Err(_) => {
            // ERR: BODY CANNOT BE CONVERTED TO STRING
            append_to_txt("error.txt", &url, &channel_id); //just in case err is img
            println!(
                "Error: Unable to convert response body to string for URL: {}",
                url
            );
            return Ok(false); //keep searching
        }
    }
}

#[tokio::main]
async fn main() {
    let mut iteration: u128 = 0; //made it a u128 just in case it hits a limit
    let mut rng: StdRng = SeedableRng::seed_from_u64(173842069800850911);
    // MAIN LOOP
    loop {
        iteration += 1; //count for fun
        let pattern: String = pattern_generator(&mut rng);
        let channel_id: String = pattern.split("/").nth(5).unwrap().to_string();
        if let Ok(valid) = url_valid(&pattern, &channel_id).await {
            if valid == true {
                println!("Iteration: {} Valid URL: {}", iteration, pattern);
                break;
            } else if valid == false {
                println!("Iteration: {} Invalid URL: {}", iteration, pattern);
            } else {
                println!("Iteration: {} Error parsing URL: {}", iteration, pattern);
            }
        }
    }
}
