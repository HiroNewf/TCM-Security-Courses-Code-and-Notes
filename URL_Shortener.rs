#![allow(unused)]

use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use rand::{distributions::Alphanumeric, Rng};

fn main() {
    let mapping_path = "src/mapping.txt";
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: ./url_shortener <url>");
        return;
    }

    let url = &args[1];

    if url.starts_with("http") {
        // Convert long-form URL to short-form URL.
        let mut rng = rand::thread_rng();
        let short_url: String = std::iter::repeat(())
            .map(|()| rng.sample(Alphanumeric) as char)
            .take(8)
            .collect();

        println!("Long URL: {}", url);
        println!("Short URL: {}", short_url);

        // Store the mapping between the short URL and the long URL in a file.
        let mut mapping_file = match OpenOptions::new().write(true).append(true).open(mapping_path) {
            Ok(file) => file,
            Err(_) => {
                println!("Error opening mapping file");
                return;
            }
        };
        let mapping = format!("{},{}\n", short_url, url);
        if let Err(_) = mapping_file.write_all(mapping.as_bytes()) {
            println!("Error writing to mapping file");
            return;
        }
    } else {
        // Read the mapping file and redirect the user to the corresponding long URL.
        let mapping_file = match File::open(mapping_path) {
            Ok(file) => file,
            Err(_) => {
                println!("Error opening mapping file");
                return;
            }
        };
        let reader = BufReader::new(mapping_file);
        for line in reader.lines() {
            let mapping = match line {
                Ok(line) => line,
                Err(_) => {
                    println!("Error reading mapping file");
                    continue;
                }
            };
            let parts: Vec<&str> = mapping.split(',').collect();
            if parts.len() != 2 {
                continue;
            }
            let short = parts[0];
            let long = parts[1];
            if short == url {
                println!("Redirecting to {}", long);
                return;
            }
        }
        println!("Short URL not found");
    }
}
