               ://! Module for distance related stuff.
               :
               :use newtypes::Km;
               :use newtypes::ToF64;
               :use newtypes::Located;
               :
               :/// Return the distance between two points.
               :///
               :/// # Example
               :/// ```
               :/// # extern crate newtypes;
               :/// # extern crate util;
               :/// # fn main() {
               :/// use util::distance::distance;
               :/// use newtypes::Km;
               :/// let zero = Km::from_f64(0.0);
               :/// let three = Km::from_f64(3.0);
               :/// let four = Km::from_f64(4.0);
               :/// let five = Km::from_f64(5.0);
               :/// assert_eq!(distance((zero, zero), (three, four)), five);
               :/// # }
               :/// ```
               :pub fn distance<I : Into<(Km, Km)>>(a : I, b : I) -> Km {
               :    let a = a.into();
               :    let b = b.into();
               :    let dx = a.0.to_f64() - b.0.to_f64();
               :    let dy = a.1.to_f64() - b.1.to_f64();
               :    Km::from_f64((dx*dx + dy*dy).sqrt())
               :}
               :
               :use std::f64::consts::PI;
               :
     1 2.6e-05 :fn hav(a: f64) -> f64 /* util::util::distance::hav::h1f0b3314ff14ef19 total:    340  0.0090 */
               :{
   302  0.0080 :    (1. - a.cos()) / 2.
    28 7.4e-04 :}
               :
               :pub fn distance_lon_lat<A : Located, B : Located>(a : &A, b: &B, radius : Km) -> Km {
     9 2.4e-04 :    let a = a.located();
     4 1.1e-04 :    let b = b.located();
     6 1.6e-04 :    let phi1 = a.lat * PI / 180.0;
    54  0.0014 :    let phi2 = b.lat * PI / 180.0;
    24 6.3e-04 :    let dl = (a.lon - b.lon) * PI / 180.0;
   232  0.0061 :    let angle = hav(phi1 - phi2) + phi1.cos() * phi2.cos() * hav(dl);
    25 6.6e-04 :    radius * ((1. - angle * 2.).acos())
               :}
               :
               :use na::Vector2;
               :
    31 8.2e-04 :pub fn distance_to_edge(point: (Km, Km), start: (Km, Km), end: (Km, Km)) -> Km { /* util::util::distance::distance_to_edge::had582d2d782b1904 total:    226  0.0060 */
    10 2.6e-04 :    let point = Vector2::new((point.0).to_f64(), (point.1).to_f64());
     6 1.6e-04 :    let start = Vector2::new((start.0).to_f64(), (start.1).to_f64());
     8 2.1e-04 :    let end = Vector2::new((end.0).to_f64(), (end.1).to_f64());
    22 5.8e-04 :    if (start - point).dot(&(start - end)) < 0.0 {
               :        Km::from_f64((start - point).norm())
     5 1.3e-04 :    } else if (end - point).dot(&(end - start)) < 0.0 {
               :        Km::from_f64((end - point).norm())
               :    } else {
               :        let res = (start - point).perp(&(start - end));
     6 1.6e-04 :        Km::from_f64(res.abs() / (start - end).norm())
               :    }
     3 7.9e-05 :}
               :
               :#[test]
               :fn test_distance_to_edge() {
               :    let points = [(Km::from_f64(0.), Km::from_f64(0.)),
               :                  (Km::from_f64(1.), Km::from_f64(1.)),
               :                  (Km::from_f64(2.), Km::from_f64(1.)),
               :                  (Km::from_f64(2.), Km::from_f64(0.))];
               :    assert!((distance_to_edge(points[0], points[1], points[2]) -
               :             Km::from_f64((2. as f64).sqrt()))
               :                    .to_f64()
               :                    .abs() < 1.0e-6);
               :    assert!((distance_to_edge(points[2], points[3], points[1]) -
               :             Km::from_f64((0.5 as f64).sqrt()))
               :                    .to_f64()
               :                    .abs() < 1.0e-6);
               :    assert!((distance_to_edge(points[0], points[2], points[1]) -
               :             Km::from_f64((2. as f64).sqrt()))
               :                    .to_f64()
               :                    .abs() < 1.0e-6);
               :}
/* 
 * Total samples for file : "/home/gerwin/Documents/projects/stujob/lopeningent_backend/server/routing_server/util/src/util/distance.rs"
 * 
 *    776  0.0205
 */


/* 
 * Command line: opannotate --source --output-dir=annotations ./target/release/routing_server 
 * 
 * Interpretation of command line:
 * Output annotated source file with samples
 * Output all files
 * 
 * CPU: Intel Ivy Bridge microarchitecture, speed 3100 MHz (estimated)
 * Counted CPU_CLK_UNHALTED events (Clock cycles when not halted) with a unit mask of 0x00 (No unit mask) count 90000
 */
