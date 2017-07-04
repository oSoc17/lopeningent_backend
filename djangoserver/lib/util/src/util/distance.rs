//! Module for distance related stuff.

use newtypes::Km;
use newtypes::ToF64;

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
