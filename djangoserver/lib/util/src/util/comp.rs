//! Comparisor.
//!
//! This structure contains a way to save and sort both a perceived and actual distance,
//! as needed by the routing algorithm.

use newtypes::Km;
use num::Zero;

use std::ops::Add;
use std::cmp::Ordering;

/// Comparisor structure.
#[derive(PartialEq, Eq, Debug)]
pub struct Comp {
    size : Option<Km>,
    actual : Km,
}

impl Comp {
    /// Create a new comparisor
    pub fn new(size : Option<Km>, actual : Km) -> Comp {
        Comp {
            size : size,
            actual : actual,
        }
    }

    /// Returns whether the perceived size is finite.
    ///
    /// # Examples
    /// ```
    /// # extern crate newtypes;
    /// # extern crate util;
    /// # fn main() {
    /// use util::comp::Comp;
    /// use newtypes::Km;
    ///
    /// let finitecomp = Comp::new(Km::from_f64_checked(0.5), Km::from_f64(0.5));
    /// assert!(finitecomp.is_finite());
    ///
    /// let infinitecomp = Comp::new(Km::from_f64_checked(1e200), Km::from_f64(0.5));
    /// assert_eq!(infinitecomp.is_finite(), false)
    /// # }
    /// ```
    pub fn is_finite(&self) -> bool {
        self.size.is_some()
    }

    /// Return the size
    pub fn size(&self) -> Option<Km> {
        self.size
    }

    /// Return the actual distance
    pub fn actual(&self) -> Km {
        self.actual
    }
}

impl PartialOrd for Comp {
    fn partial_cmp(&self, other : &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Comp {
    fn cmp(&self, other : &Self) -> Ordering {
        match (self.size, other.size) {
            (None, None) => Ordering::Equal,
            (None, Some(_)) => Ordering::Greater,
            (Some(_), None) => Ordering::Less,
            (Some(t), Some(u)) => t.cmp(&u)
        }
    }
}

impl<'a, 'b> Add<&'b Comp> for &'a Comp {
    type Output = Comp;
    fn add(self, other : &'b Comp) -> Comp {
        Comp {
            size : self.size.and_then(|u| other.size.map(|v| u + v)),
            actual : self.actual + other.actual,
        }
    }
}

impl Add for Comp {
    type Output = Comp;
    fn add(self, other : Self) -> Comp {
        &self + &other
    }
}

impl Zero for Comp {
    fn zero() -> Comp {
        Comp {
            size : Some(Km::zero()),
            actual : Km::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}
