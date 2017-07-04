use curl::easy::Easy;
use error::Error;
use get_host_port;
use get_lat_lon;
use parse_link;
use std::io;
use std::io::Write;
use std::mem;
use std::time;

/// Test if all methods that should return a 200 return a 200.
pub fn run() -> Result<(), Error> {
    // Test 200's
    let host = get_host_port()?;
    let (lat, lon) = get_lat_lon()?;

    let source = format!("{}", host);
    for url in vec![format!("/route/generate?lat={}&lon={}", lat, lon)] {
        for query in vec![
            "&type=indices",
            "&type=coordinates",
            "&type=geojson",
            "&type=directions",
            "&type=length",
            ""] {
            for lengths in vec![
                "&min_length=18.0&max_length=20.0",
                "&min_length=45.0&max_length=50.0",
                "&min_length=27.0&max_length=30.0",
                ""] {
                for poison in vec![
                    "&poison_max_value=2.0&poison_max_distance=1.0",
                    "&poison_max_value=120.0&poison_max_distance=10.0",
                    ""] {
                    test_return_code(&format!("{}{}{}{}{}", source, url, query, lengths, poison), 200)?;
                }
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
        let _ = test_return_code(&format!("{}{}", source, url), 200).err().ok_or_else(|| Error::CodeError("Shouldn't be a 200!".to_string(), 200, url.to_string()))?;
    }
    Ok(())
}

fn test_return_code(link : &str, res : usize) -> Result<(), Error> {
    let now = time::Instant::now();
    print!("Trying...... {} ", parse_link(link));
    io::stdout().flush();
    let mut easy = Easy::new();
    easy.url(link)?;
    easy.perform()?;
    let res_code = easy.response_code()? as usize;
    mem::drop(easy);
    let dur = (time::Instant::now() - now);
    println!("\rTook {:7.04}", dur.as_secs() as f64 + dur.subsec_nanos() as f64 /1e9);
    if res_code == res {
        Ok(())
    } else {
        Err(Error::CodeError("Wrong return code!".to_string(), res_code, link.to_string()))
    }
}
