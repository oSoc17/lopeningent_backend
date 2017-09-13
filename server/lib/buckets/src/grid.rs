use newtypes::Km;
use interval::Interval;
use num::Zero;

pub struct Grid<T> {
    x: Km,
    y: Km,
    width: usize,
    height: usize,
    binsize: Km,
    data: Vec<Vec<T>>,
}

impl<T> Grid<T> {
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

    fn clamp(x: Km, min: Km, binsize: Km, max: usize) -> usize {
        let x = if x < min { Km::zero() } else { x };
        let x = (x / binsize) as usize;
        if x >= max { max - 1 } else { x }
    }

    pub fn get_xy(&self, coord: (Km, Km)) -> (usize, usize) {
        let x = Self::clamp(coord.0 - self.x, Km::zero(), self.binsize, self.width);
        let y = Self::clamp(coord.1 - self.y, Km::zero(), self.binsize, self.height);
        (x, y)
    }

    pub fn get_indexed(&self, index: (usize, usize)) -> Option<&[T]> {
        if index.0 >= self.width || index.1 >= self.height {
            None
        } else {
            Some(&self.data[self.get_index(index)])
        }
    }

    fn get_index(&self, index: (usize, usize)) -> usize {
        self.width * index.1 + index.0
    }

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
