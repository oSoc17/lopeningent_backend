//! Module for distance related stuff.

use newtypes::Km;
use newtypes::ToF64;
use newtypes::Located;

/// Return the distance between two points.
///
/// # Example
/// ```
/// # extern crate newtypes;
/// # extern crate util;
/// # fn main() {
/// use util::distance::distance;
/// use newtypes::Km;
/// let zero = Km::from_f64(0.0);
/// let three = Km::from_f64(3.0);
/// let four = Km::from_f64(4.0);
/// let five = Km::from_f64(5.0);
/// assert_eq!(distance((zero, zero), (three, four)), five);
/// # }
/// ```
pub fn distance<I : Into<(Km, Km)>>(a : I, b : I) -> Km {
    let a = a.into();
    let b = b.into();
    let dx = a.0.to_f64() - b.0.to_f64();
    let dy = a.1.to_f64() - b.1.to_f64();
    Km::from_f64((dx*dx + dy*dy).sqrt())
}

use std::f64::consts::PI;

fn hav(a: f64) -> f64
{
    (1. - a.cos()) / 2.
}

pub fn distance_lon_lat<A : Located, B : Located>(a : &A, b: &B, radius : Km) -> Km {
    let a = a.located();
    let b = b.located();
    let phi1 = a.lat * PI / 180.0;
    let phi2 = b.lat * PI / 180.0;
    let dl = (a.lon - b.lon) * PI / 180.0;
    let angle = hav(phi1 - phi2) + phi1.cos() * phi2.cos() * hav(dl);
    radius * ((1. - angle * 2.).acos())
}

use na::Vector2;

pub fn distance_to_edge(point: (Km, Km), start: (Km, Km), end: (Km, Km)) -> Km {
    let point = Vector2::new((point.0).to_f64(), (point.1).to_f64());
    let start = Vector2::new((start.0).to_f64(), (start.1).to_f64());
    let end = Vector2::new((end.0).to_f64(), (end.1).to_f64());
    if (start - point).dot(&(start - end)) < 0.0 {
        Km::from_f64((start - point).norm())
    } else if (end - point).dot(&(end - start)) < 0.0 {
        Km::from_f64((end - point).norm())
    } else {
        let res = (start - point).perp(&(start - end));
        Km::from_f64(res.abs() / (start - end).norm())
    }
}

#[test]
fn test_distance_to_edge() {
    let points = [(Km::from_f64(0.), Km::from_f64(0.)),
                  (Km::from_f64(1.), Km::from_f64(1.)),
                  (Km::from_f64(2.), Km::from_f64(1.)),
                  (Km::from_f64(2.), Km::from_f64(0.))];
    assert!((distance_to_edge(points[0], points[1], points[2]) -
             Km::from_f64((2. as f64).sqrt()))
                    .to_f64()
                    .abs() < 1.0e-6);
    assert!((distance_to_edge(points[2], points[3], points[1]) -
             Km::from_f64((0.5 as f64).sqrt()))
                    .to_f64()
                    .abs() < 1.0e-6);
    assert!((distance_to_edge(points[0], points[2], points[1]) -
             Km::from_f64((2. as f64).sqrt()))
                    .to_f64()
                    .abs() < 1.0e-6);
}
