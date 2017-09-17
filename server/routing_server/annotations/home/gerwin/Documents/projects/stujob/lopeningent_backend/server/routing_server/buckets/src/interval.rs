               :use newtypes::Km;
               :use std::ops::Add;
               :use num::Zero;
               :
               :#[derive(Clone, Copy)]
               :pub struct Interval {
               :    minx: Km,
               :    maxx: Km,
               :    miny: Km,
               :    maxy: Km,
               :}
               :
               :impl Interval {
    15 4.0e-04 :    pub fn from(start: (Km, Km), end: (Km, Km), tolerance: Km) -> Interval { /* buckets::interval::Interval::from::h956ec825a4365356 total:     43  0.0011 */
    11 2.9e-04 :        let x = if start.0 < end.0 {
               :                     (start.0, end.0)
               :                 } else {
               :                     (end.0, start.0)
               :                 };
    10 2.6e-04 :        let y = if start.1 < end.1 {
               :                     (start.1, end.1)
               :                 } else {
               :                     (end.1, start.1)
               :                 };
    10 2.6e-04 :        Interval {
     2 5.3e-05 :            minx: x.0 - tolerance,
     3 7.9e-05 :            maxx: x.1 + tolerance,
     8 2.1e-04 :            miny: y.0 - tolerance,
    13 3.4e-04 :            maxy: y.1 + tolerance,
               :        }
     6 1.6e-04 :    }
     3 7.9e-05 :    pub fn min(&self) -> (Km, Km) { /* buckets::interval::Interval::min::hf0561eb7f482c442 total:     20 5.3e-04 */
    14 3.7e-04 :        (self.minx, self.miny)
     4 1.1e-04 :    }
               :
     3 7.9e-05 :    pub fn max(&self) -> (Km, Km) { /* buckets::interval::Interval::max::hbce28c85ba4b4f55 total:      6 1.6e-04 */
     8 2.1e-04 :        (self.maxx, self.maxy)
     1 2.6e-05 :    }
               :}
               :
               :impl<'a, 'b> Add<&'a Interval> for &'b Interval {
               :    type Output = Interval;
     3 7.9e-05 :    fn add(self, other: &'a Interval) -> Interval { /* _$LT$$RF$$u27$b$u20$buckets..interval..Interval$u20$as$u20$core..ops..Add$LT$$RF$$u27$a$u20$buckets..interval..Interval$GT$$GT$::add::hff5f1ad3db8853c1 total:     63  0.0017 */
               :        let min = Interval::from(self.min(), other.min(), Km::zero()).min();
               :        let max = Interval::from(self.max(), other.max(), Km::zero()).max();
               :        Interval::from(min, max, Km::zero())
     2 5.3e-05 :    }
               :}
/* 
 * Total samples for file : "/home/gerwin/Documents/projects/stujob/lopeningent_backend/server/routing_server/buckets/src/interval.rs"
 * 
 *    116  0.0031
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
