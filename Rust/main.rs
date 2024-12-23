/////////////////////////////////////////////////////////////////////////////////////////////
//                                                                                         //
//    Discord5050: A recursive search algorithm that generates and validates randomized    //
//    Discord media links until it finds a valid image, and then outputs the URL.          //
//                                                                                         //
//    Copyright (C) <2024>  <sappyduckie>                                                  //
//                                                                                         //
//    This program is free software; you can redistribute it and/or modify                 //
//    it under the terms of the GNU General Public License as published by                 //
//    the Free Software Foundation; either version 2 of the License, or                    //
//    (at your option) any later version.                                                  //
//                                                                                         //
//    This program is distributed in the hope that it will be useful,                      //
//    but WITHOUT ANY WARRANTY; without even the implied warranty of                       //
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the                        //
//    GNU General Public License for more details.                                         //
//                                                                                         //
//    You should have received a copy of the GNU General Public License along              //
//    with this program; if not, write to the Free Software Foundation, Inc.,              //
//    51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.                          //
//                                                                                         //
/////////////////////////////////////////////////////////////////////////////////////////////

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use reqwest::Client;
use std::fs::OpenOptions;
use std::io::Write;
use tokio;

fn hash_gen(rng: &mut StdRng, length: usize) -> String {
    let hex_chars: &str = "0123456789abcdef";
    let mut hex_string: String = String::with_capacity(length);

    // Generate hex char, loop 'length' times
    for _ in 0..length {
        let index: usize = rng.gen_range(0..16);
        hex_string.push(hex_chars.chars().nth(index).unwrap());
    }

    hex_string
}

fn pattern_generator(rng: &mut StdRng) -> String {
    // randomized variables
    let guild_id: u64 = rng.gen_range(1..=999999999999999999); //18 digits + 1
    let one_two: u8 = rng.gen_range(1..=2); //1 digit: one or two
    let channel_id: u64 = rng.gen_range(1..=9999999999999999); //16 digits + 3
    let first_hex_8: String = hash_gen(rng, 8); //8 hex chars
    let second_hex_8: String = hash_gen(rng, 8); //8 hex chars
    let third_hex_8: String = hash_gen(rng, 8); //8 hex chars
    let hex_64: String = hash_gen(rng, 64); //64 hex chars
    let format_width: u16 = rng.gen_range(1..=999); //3 digits
    let format_height: u16 = rng.gen_range(1..=999); //3 digits

    // final url
    let url: String = format!("https://media.discordapp.net/attachments/1{}/13{}{}/image.png?ex={}&is={}&hm={}&=&format=webp&quality=lossless{}&width={}&height={}", guild_id, one_two, channel_id, first_hex_8, second_hex_8, third_hex_8, hex_64, format_width, format_height);

    return url;
}

fn append_to_txt(file_path: &str, url: &str) {
    // open the file and append, create if not exist
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)
        .expect("Unable to open or create file");
    // write the url to the file followed by a newline
    if let Err(e) = writeln!(file, "{}", url) {
        eprintln!("Error writing to file: {}", e);
    }
}

async fn url_valid(url: &str) -> Result<bool, reqwest::Error> {
    let client: Client = Client::new();
    let response: reqwest::Response = client.get(url).send().await?;
    let status: reqwest::StatusCode = response.status();
    // check if response is successful
    if status.is_success() {
        return Ok(true);
    }
    // attempt to get the response body as text
    match response.text().await {
        Ok(body) => {
            if body.contains("This content is no longer available.") {
                return Ok(false); //return invalid if 404
            } else {
                append_to_txt("valid.txt", &url);
                return Ok(true); //return valid if not 404
            }
        }
        Err(_) => {
            // body cannot be converted to a string
            append_to_txt("valid.txt", &url);
            println!(
                "Error: Unable to convert response body to string for URL: {}",
                url
            );
            return Ok(true); //return valid if error
        }
    }
}

#[tokio::main]
async fn main() {
    let mut iteration: u64 = 0;
    let mut rng: StdRng = SeedableRng::seed_from_u64(173842069800850911); // Initialize RNG with the static seed

    loop {
        iteration += 1;
        let pattern: String = pattern_generator(&mut rng);
        if let Ok(valid) = url_valid(&pattern).await {
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
