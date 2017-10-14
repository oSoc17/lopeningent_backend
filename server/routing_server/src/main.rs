#![warn(missing_docs)]
extern crate iron_frontend;
extern crate fern;
extern crate chrono;
extern crate log;



use std::env;
use std::io;
use std::io::Write;

fn init_log() -> Result<(), Box<std::error::Error>> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LogLevelFilter::Info)
        .chain(fern::log_file("server.log")?)
        .apply()?;
        Ok(())
}

fn main() {
    init_log().unwrap_or_else(|e| {writeln!(io::stderr(), "Failed to initialize logger! {}", e);});
    let config = env::args().nth(1).unwrap_or_else(|| "config.txt".to_string());
    iron_frontend::fire(&config).unwrap();
}
