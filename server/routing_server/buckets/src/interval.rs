use newtypes::Km;
use std::ops::Add;
use num::Zero;

/// The interval structure.
/// This structure contains data of a rectangle.
///
/// Addition of two intervals is defined as the smallest interval containing both of them.

/// Structure representing a twodimensional array.
#[derive(Clone, Copy)]
pub struct Interval {
    minx: Km,
    maxx: Km,
    miny: Km,
    maxy: Km,
}

impl Interval {
    /// Create an interval from two diagonal corners, and a tolerance (adding a boundary to the interval).
    pub fn from(start: (Km, Km), end: (Km, Km), tolerance: Km) -> Interval {
        let x = if start.0 < end.0 {
                     (start.0, end.0)
                 } else {
                     (end.0, start.0)
                 };
        let y = if start.1 < end.1 {
                     (start.1, end.1)
                 } else {
                     (end.1, start.1)
                 };
        Interval {
            minx: x.0 - tolerance,
            maxx: x.1 + tolerance,
            miny: y.0 - tolerance,
            maxy: y.1 + tolerance,
        }
    }

    #[allow(missing_docs)]
    pub fn min(&self) -> (Km, Km) {
        (self.minx, self.miny)
    }

    #[allow(missing_docs)]
    pub fn max(&self) -> (Km, Km) {
        (self.maxx, self.maxy)
    }
}

impl<'a, 'b> Add<&'a Interval> for &'b Interval {
    type Output = Interval;
    fn add(self, other: &'a Interval) -> Interval {
        let min = Interval::from(self.min(), other.min(), Km::zero()).min();
        let max = Interval::from(self.max(), other.max(), Km::zero()).max();
        Interval::from(min, max, Km::zero())
    }
}
