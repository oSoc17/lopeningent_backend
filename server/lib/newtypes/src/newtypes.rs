//! Newtype structures.
//!
//! Mostly implementation of a fixed point Km representation.

use std::ops::{Add, Sub, Mul, Div};
use num::Zero;
use std::f64;

/// Distance measure in kilometers
#[derive(Clone, Copy, PartialEq, Debug, Eq, PartialOrd, Ord)]
pub struct Km(u64);

const POINT : usize = 32;

/// Can be cast to a f64
pub trait ToF64 {
    /// Cast
    fn to_f64(&self) -> f64;
}

impl Km {
    /// Create a new Km struct, or Km::zero if something goes wrong (NaN, Out of Bounds).
    pub fn from_f64(f: f64) -> Km {
        Km((f * (1u64<<POINT) as f64) as u64)
    }

    /// Create a new Km struct, or None if something goes wrong (NaN, Out of Bounds).
    ///
    /// # Examples
    /// ```
    /// use newtypes::Km;
    /// use std::u32;
    /// let valid = Km::from_f64_checked(0.0);
    /// assert!(valid.is_some());
    /// let invalid = Km::from_f64_checked(u32::MAX as f64 + 1.0);
    /// assert_eq!(invalid, None, "Failed at 1");
    /// let invalid = Km::from_f64_checked(-1.0);
    /// assert_eq!(invalid, None, "Failed at 2");
    /// ```
    pub fn from_f64_checked(f : f64) -> Option<Km> {
        // Beware rounding errors!
        if f >= Km(u64::max_value()).to_f64() {
            None
        } else if f < Km(u64::min_value()).to_f64() {
            None
        } else {Some(Km::from_f64(f))}
    }
}

impl ToF64 for Km {
    fn to_f64(&self) -> f64 {
        self.0 as f64/((1u64 << POINT) as f64)
    }
}

impl ToF64 for Option<Km> {
    fn to_f64(&self) -> f64 {
        self.map(|Km(u)| u as f64/((1u64 << POINT) as f64)).unwrap_or(f64::INFINITY)
    }
}

impl Add<Km> for Km {
    type Output = Km;
    fn add(self, other: Km) -> Km {
        Km(self.0 + other.0)
    }
}

impl Sub<Km> for Km {
    type Output = Km;
    fn sub(self, other: Km) -> Km {
        Km(self.0 - other.0)
    }
}

impl Mul<f64> for Km {
    type Output = Km;
    fn mul(self, other: f64) -> Km {
        Km::from_f64(self.to_f64() * other)
    }
}

impl Div<Km> for Km {
    type Output = f64;
    fn div(self, other: Km) -> f64 {
        (self.0 as f64 / other.0 as f64)
    }
}


impl Div<Option<Km>> for Km {
    type Output = f64;
    fn div(self, other: Option<Km>) -> f64 {
        other.map(|o| (self.0 as f64 / o.0 as f64))
        .unwrap_or(0.0)
    }
}

impl Zero for Km {
    fn zero() -> Km {
        Km(0)
    }

    fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

use std::fmt;
impl fmt::Display for Km {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.to_f64().fmt(fmt)?;
        fmt.write_str(" Km")
    }
}
