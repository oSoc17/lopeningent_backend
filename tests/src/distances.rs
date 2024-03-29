use error::Error;
use geo::LineString;
use geo::Point;
use geo::algorithm::haversine_distance::HaversineDistance;
use curl::easy::Easy;
use std::sync::mpsc::channel;
use get_host_port;
use get_lat_lon;
use parse_link;
use serde_json;
use std::time;
use std::io;
use std::io::{Read, Write};

/**
    Some serialisation boilerplate.
**/
#[derive(Serialize, Deserialize)]
struct Distance {
    length : f64,
    perceived : f64
}

#[derive(Serialize, Deserialize)]
struct GeoJson {
    features : Vec<Feature>,
}

#[derive(Serialize, Deserialize)]
struct Feature {
    geometry : Geometry,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum Geometry {
    LineString{coordinates : Vec<(f64, f64)>},
    Point{coordinates : (f64, f64)}
}

impl Into<LineString<f64>> for GeoJson {
    fn into(self) -> LineString<f64> {
        let iter = self.features.into_iter()
            .filter_map(|f| match f.geometry  {
                Geometry::LineString { coordinates } => Some(coordinates),
                _ => None,
            }).next().unwrap().into_iter();
        LineString(iter.map(|(x, y)| Point::new(x, y)).collect())
    }
}

/// Check whether routes are generated with the correct length
pub fn distances() -> Result<(), Error>{
    let host = get_host_port()?;
    let (lat, lon) = get_lat_lon()?;
    for (min, max) in (5..51).map(|n| n as usize as f64).map(|f| (0.8*f, f)) {
        let url = format!("distance={:0.1}&lat={:0.1}&lon={:0.1}&type=geojson", max, lat, lon);
        let res = get(&host, &url)?;
        if res > max || res < min {
            Err(format!("Length of {} found, which is not between {} and {}.", res, min, max))?;
        }
    }
    Ok(())
}

/// Returns the length of a geoJSON line.
pub fn length(linestring : &LineString<f64>) -> f64 {
    linestring.0.iter().zip((&linestring.0[1..]).iter()).map(
        |(a, b)| a.haversine_distance(b)
    ).fold(0.0, |a, b| a + b)
}

fn get<'a>(link : &'a str, data : &'a str) -> Result<f64, Error> {
    print!("Loading..... {}", data);
    let mut data = data.as_bytes();
    let now = time::Instant::now();
    let mut easy = Easy::new();
    io::stdout().flush();
    easy.url(&format!("{}/route/generate/", link))?;
    easy.post(true)?;
    easy.post_field_size(data.len() as u64)?;
    let (sx, rx) = channel();
    let mut transfer = easy.transfer();
    transfer.write_function(move |data|
        {
            data.iter().map(|&b| sx.send(b)).collect::<Result<Vec<()>, _>>().unwrap();
            Ok(data.len())
        })?;
    transfer.read_function(|buf| Ok(data.read(buf).unwrap_or(0)))?;
    transfer.perform()?;
    use std::mem;
    mem::drop(transfer);
    let dur = (time::Instant::now() - now);
    println!("\rTook {:7.04}", dur.as_secs() as f64 + dur.subsec_nanos() as f64 /1e9);
    let buf : Vec<_> = rx.into_iter().collect();
    let linestring : LineString<f64> = serde_json::from_slice::<GeoJson>(&buf).unwrap().into();
    Ok(length(&linestring) / 1000.0)
}
