extern crate util;
extern crate newtypes;

fn main() {
    println!("{} ~ 111.2", util::distance::distance_lon_lat(
        &newtypes::Location::new(3.7, 51.0),
        &newtypes::Location::new(3.7, 50.0),
        newtypes::Km::from_f64(6371.0)));
    println!("{} ~ 258.1", util::distance::distance_lon_lat(
        &newtypes::Location::new(3.7, 51.0),
        &newtypes::Location::new(0.0, 51.0),
        newtypes::Km::from_f64(6371.0)));
    println!("{}", util::distance::distance_lon_lat(
        &newtypes::Location::new(180.0, 45.0),
        &newtypes::Location::new(0.0, 45.0),
        newtypes::Km::from_f64(6371.0)));
    println!("{}", util::distance::distance_lon_lat(
        &newtypes::Location::new(3.7255717, 51.0536801),
        &newtypes::Location::new(3.7255726, 51.0536798),
        newtypes::Km::from_f64(6371.0)));
    
}
