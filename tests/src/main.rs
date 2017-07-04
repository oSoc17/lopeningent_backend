extern crate curl;
extern crate geo;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod error;
mod coarse;
mod distances;

use error::Error;
use std::env;

fn main() {
    distances::distances().expect("\n\nTHE SERVER FAILED!\n\n");
    coarse::run().expect("\n\nTHE SERVER FAILED!\n\n");
    distances::distances().expect("\n\nTHE SERVER FAILED!\n\n");
    println!("Done. Please check if the delays at the start match up with the delays at the end.");
}

pub fn get_host_port() -> Result<String, Error> {
    let usage = format!("Usage: {} <host>:<port>", env::args().next().unwrap_or_else(|| "./tests".to_string()));
    let host = env::args().nth(1).ok_or_else(|| usage.clone())?;
    Ok(host)
}

pub fn get_lat_lon() -> Result<(f64, f64), Error> {
    let lat = env::args().nth(2).unwrap_or_else(||"51.0".to_string()).parse()?;
    let lon = env::args().nth(3).unwrap_or_else(||"3.8".to_string()).parse()?;
    Ok((lat, lon))
}

pub fn parse_link(link : &str) -> &str {
    let index = link.bytes().enumerate().filter(|&(_, c)| c == '?' as u8).next().unwrap_or((0, 0)).0;
    &link[index..]
}
