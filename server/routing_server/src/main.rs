#![warn(missing_docs)]
extern crate iron_frontend;

use std::env;

fn main() {
    let config = env::args().nth(1).unwrap_or_else(|| "config.txt".to_string());
    iron_frontend::fire(&config).unwrap();
}
