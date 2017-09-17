               :use newtypes::Km;
               :use interval::Interval;
               :use num::Zero;
               :
               :pub struct Grid<T> {
               :    x: Km,
               :    y: Km,
               :    width: usize,
               :    height: usize,
               :    binsize: Km,
               :    data: Vec<Vec<T>>,
               :}
               :
               :impl<T> Grid<T> {
               :    pub fn from(interval: Interval, binsize: Km) -> Grid<T> {
               :        let width = ((interval.max().0 - interval.min().0) / binsize) as usize;
               :        let height = ((interval.max().1 - interval.min().1) / binsize) as usize;
               :        let x = (interval.max().0 + interval.min().0 - binsize * width as f64) * 0.5;
               :        let y = (interval.max().1 + interval.min().1 - binsize * height as f64) * 0.5;
               :        Grid {
               :            x: x,
               :            y: y,
               :            width: width,
               :            height: height,
               :            binsize: binsize,
               :            data: (0..width * height).map(|_| Vec::new()).collect(),
               :        }
               :    }
               :
               :    pub fn get_max_x(&self) -> Km {
               :        self.x + self.binsize * self.width as f64
               :    }
               :
               :    pub fn get_max_y(&self) -> Km {
               :        self.y + self.binsize * self.height as f64
               :    }
               :
               :    fn clamp(x: Km, min: Km, binsize: Km, max: usize) -> usize {
     6 1.6e-04 :        let x = if x < min { Km::zero() } else { x };
    45  0.0012 :        let x = (x / binsize) as usize;
    25 6.6e-04 :        if x >= max { max - 1 } else { x }
               :    }
               :
    19 5.0e-04 :    pub fn get_xy(&self, coord: (Km, Km)) -> (usize, usize) { /* _$LT$buckets..grid..Grid$LT$T$GT$$GT$::get_xy::h91235a3d73b34be2 total:    128  0.0034 */
     3 7.9e-05 :        let x = Self::clamp(coord.0 - self.x, Km::zero(), self.binsize, self.width);
    14 3.7e-04 :        let y = Self::clamp(coord.1 - self.y, Km::zero(), self.binsize, self.height);
     1 2.6e-05 :        (x, y)
    15 4.0e-04 :    }
               :
               :    pub fn get_indexed(&self, index: (usize, usize)) -> Option<&[T]> {
               :        if index.0 >= self.width || index.1 >= self.height {
               :            None
               :        } else {
               :            Some(&self.data[self.get_index(index)])
               :        }
               :    }
               :
               :    fn get_index(&self, index: (usize, usize)) -> usize {
               :        self.width * index.1 + index.0
               :    }
               :
               :    pub fn get(&self, coord: (Km, Km)) -> &[T] {
               :        self.get_indexed(self.get_xy(coord))
               :            .expect("Implementation error")
               :    }
               :}
               :
               :use std::fmt;
               :impl<T: fmt::Debug> fmt::Debug for Grid<T> {
               :    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
               :        fmt.debug_struct("Grid")
               :            .field("size", &(self.width, self.height))
               :            .field("length",
               :                   &self.data
               :                        .iter()
               :                        .map(|el| el.len())
               :                        .fold(0, |x: usize, y: usize| x + y))
               :            .finish()
               :    }
               :}
               :
               :impl<T: Clone> Grid<T> {
    35 9.2e-04 :    pub fn add(&mut self, interval: Interval, t: &T) {
     1 2.6e-05 :        let min = self.get_xy(interval.min());
               :        let max = self.get_xy(interval.max());
               :        for x in min.0..max.0 + 1 {
     1 2.6e-05 :            for y in min.1..max.1 + 1 {
               :                let id = self.get_index((x, y));
               :                self.data[id].push(t.clone())
               :            }
               :        }
               :    }
               :}
/* 
 * Total samples for file : "/home/gerwin/Documents/projects/stujob/lopeningent_backend/server/routing_server/buckets/src/grid.rs"
 * 
 *    165  0.0044
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
