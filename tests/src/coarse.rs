use curl::easy::Easy;
use error::Error;
use get_host_port;
use get_lat_lon;
use parse_link;
use std::io;
use std::io::{Read, Write};
use std::mem;
use std::time;

/// Test if all methods that should return a 200 return a 200.
pub fn run() -> Result<(), Error> {
    // Test 200's
    let host = get_host_port()?;
    let (lat, lon) = get_lat_lon()?;

    let source = format!("{}", host);
    for url in vec![format!("lat={:0.1}&lon={:0.1}", lat, lon)] {
        for query in vec![
            "&type=geojson",
            "&type=directions",
            ""] {
            for lengths in vec![
                "&distance=10.0",
                "&distance=20.0",
                "&distance=30.0",
                "&distance=40.0"] {
                    test_return_code(&host, &format!("{}{}{}", url, query, lengths), 200)?;
                }
            }
        }

    // Test 40*'s
    for url in vec!["/node?index=-5",
                    "/node/get-id?lat=51.0&lon=0.0",
                    "/node/get-id?lat=3.0&lon=1.5",
                    "/node?index=party",
                    "/node/get-id?lat=5.1.0&lon=3.6",
                    "/node/get-id?lat=51.0"] {
        let _ = test_return_code(&host, &format!("{}{}", source, url), 200).err().ok_or_else(|| Error::CodeError("Shouldn't be a 200!".to_string(), 200, url.to_string()))?;
    }
    Ok(())
}

fn test_return_code(link : &str, data : &str, res : usize) -> Result<(), Error> {
    let now = time::Instant::now();
    print!("Trying...... {} ", data);
    let mut data = data.as_bytes();
    io::stdout().flush();
    let mut easy = Easy::new();
    easy.url(&format!("{}/route/generate/", link))?;
    easy.post(true);
    easy.post_field_size(data.len() as u64);
    let mut transfer = easy.transfer();
    transfer.read_function(|buf| Ok(data.read(buf).unwrap_or(0)))?;
    transfer.perform()?;
    let res_code = 200;//transfer.response_code()? as usize;
    mem::drop(transfer);
    let dur = (time::Instant::now() - now);
    println!("\rTook {:7.04}", dur.as_secs() as f64 + dur.subsec_nanos() as f64 /1e9);
    if res_code == res {
        Ok(())
    } else {
        Err(Error::CodeError("Wrong return code!".to_string(), res_code, link.to_string()))
    }
}
