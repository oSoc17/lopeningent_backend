extern crate database;
use database::load;
use std::time;

fn main() {
    let now = time::Instant::now();
    let scheme = load("postgresql://postgres:0987654321@localhost").unwrap();
    let duration = time::Instant::now() - now;

    println!("{}.{:09}", duration.as_secs(), duration.subsec_nanos());
    println!("{}", scheme.edges.len());
    println!("{:?}", scheme.edges.iter().filter(|edge| edge.tags.trues() > 1).next());
}
