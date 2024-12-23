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

    // gen hex char, loop 'length' times
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

async fn url_invalid(url: &str) -> Result<bool, String> {
    let client: Client = Client::new();
    match client.get(url).send().await {
        Ok(response) => {
            let status: reqwest::StatusCode = response.status();
            if status.is_success() {
                // read the response body as text
                let body_result: Result<String, _> = response.text().await;
                match body_result {
                    Ok(body) => {
                        // check for the specific string
                        if body.contains("This content is no longer available.") {
                            return Ok(true); //string found, URL is invalid
                        } else {
                            append_to_txt(url);
                            return Ok(false); //string not found, URL is valid
                        }
                    }
                    Err(e) => {
                        // handle the error while reading the body
                        eprintln!("Failed to read response body: {}", e);
                        append_to_txt(url); //append the URL even if there's an error
                        return Ok(false); //valid since we couldn't read the body
                    }
                }
            }
            Ok(false) //return false if the status is not successful
        }
        Err(e) => {
            eprintln!("Error while sending request to URL {}: {}", url, e);
            Err(format!("Failed to validate URL: {}", e))
        }
    }
}

fn append_to_txt(url: &str) {
    let file_path: &str = "valid_urls.txt";
    let mut file: std::fs::File = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(file_path)
        .expect("Failed to open or create file");
    writeln!(file, "URL: {}", url).expect("Failed to write to file");
}

#[tokio::main]
async fn main() {
    let mut iteration: u64 = 0;
    let mut rng: StdRng = SeedableRng::seed_from_u64(173842069800850911);

    loop {
        iteration += 1;
        let pattern: String = pattern_generator(&mut rng);
        if let Ok(invalid) = url_invalid(&pattern).await {
            if invalid {
                println!("Iteration: {} Invalid URL: {}", iteration, pattern);
            } else {
                println!("Iteration: {} Valid URL: {}", iteration, pattern);
                break;
            }
        }
    }
}
