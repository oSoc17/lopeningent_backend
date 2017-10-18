///
/// Implementation of a grid.
///
/// A grid contains a twodimensional grid of buckets, each of them saving the data element T.

use newtypes::Km;
use interval::Interval;
use num::Zero;


/// Grid structure.
pub struct Grid<T> {
    x: Km,
    y: Km,
    width: usize,
    height: usize,
    binsize: Km,
    data: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    /// Creates a grid with maximal range interval and buckets each of size binsize * binsize.
    pub fn from(interval: Interval, binsize: Km) -> Grid<T> {
        let width = ((interval.max().0 - interval.min().0) / binsize) as usize;
        let height = ((interval.max().1 - interval.min().1) / binsize) as usize;
        let x = (interval.max().0 + interval.min().0 - binsize * width as f64) * 0.5;
        let y = (interval.max().1 + interval.min().1 - binsize * height as f64) * 0.5;
        Grid {
            x: x,
            y: y,
            width: width,
            height: height,
            binsize: binsize,
            data: (0..width * height).map(|_| Vec::new()).collect(),
        }
    }

    #[allow(missing_docs)]
    pub fn get_max_x(&self) -> Km {
        self.x + self.binsize * self.width as f64
    }

    #[allow(missing_docs)]
    pub fn get_max_y(&self) -> Km {
        self.y + self.binsize * self.height as f64
    }

    /// Returns the interval index the coordinate x is probably in.
    /// For example:
    ///
    /// ```
    /// #extern crate newtypes;
    /// #fn main() {
    /// #use newtypes::{Km, FromF64};
    /// let x = Km::from_f64(21.0);
    /// let index = clamp(x, Km::from_f64(0.0), Km::from_f64(5.0), 20);
    /// // 20 buckets with size 5 each, starting at 0.0? Then x must fall in the fifth bucket.
    /// assert_eq!(x, 4);
    /// #}
    /// ```
    fn clamp(x: Km, min: Km, binsize: Km, max: usize) -> usize {
        let x = if x < min { Km::zero() } else { x };
        let x = (x / binsize) as usize;
        if x >= max { max - 1 } else { x }
    }

    /// Returns the bucket coordinate that has been hit.
    pub fn get_xy(&self, coord: (Km, Km)) -> (usize, usize) {
        let x = Self::clamp(coord.0 - self.x, Km::zero(), self.binsize, self.width);
        let y = Self::clamp(coord.1 - self.y, Km::zero(), self.binsize, self.height);
        (x, y)
    }

    /// Returns all elements in a bucket.
    pub fn get_indexed(&self, index: (usize, usize)) -> Option<&[T]> {
        if index.0 >= self.width || index.1 >= self.height {
            None
        } else {
            Some(&self.data[self.get_index(index)])
        }
    }

    /// Returns the index of the underlying vector.
    fn get_index(&self, index: (usize, usize)) -> usize {
        self.width * index.1 + index.0
    }

    /// Retrieves a list of intervals overlapping with (a square containing the) coordinate.
    pub fn get(&self, coord: (Km, Km)) -> &[T] {
        self.get_indexed(self.get_xy(coord))
            .expect("Implementation error")
    }
}

use std::fmt;
impl<T: fmt::Debug> fmt::Debug for Grid<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt.debug_struct("Grid")
            .field("size", &(self.width, self.height))
            .field("length",
                   &self.data
                        .iter()
                        .map(|el| el.len())
                        .fold(0, |x: usize, y: usize| x + y))
            .finish()
    }
}

impl<T: Clone> Grid<T> {
    /// Adds an interval containing data to the grid, by cloning it to every possible bucket.
    /// Use a Grid<&'a T> if that bothers you.
    pub fn add(&mut self, interval: Interval, t: &T) {
        let min = self.get_xy(interval.min());
        let max = self.get_xy(interval.max());
        for x in min.0..max.0 + 1 {
            for y in min.1..max.1 + 1 {
                let id = self.get_index((x, y));
                self.data[id].push(t.clone())
            }
        }
    }
}
