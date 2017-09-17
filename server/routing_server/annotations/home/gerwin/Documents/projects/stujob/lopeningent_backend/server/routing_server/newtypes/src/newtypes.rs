               ://! Newtype structures.
               ://!
               ://! Mostly implementation of a fixed point Km representation.
               :
               :use std::ops::{Add, Sub, Mul, Div};
               :use num::Zero;
               :use std::f64;
               :use nalgebra::Vector3;
               :
               :use std::f64::consts::PI;
               :
               :/// Distance measure in kilometers
               :#[derive(Clone, Copy, PartialEq, Debug, Eq, PartialOrd, Ord, Default)]
    18 4.8e-04 :pub struct Km(i64);
               :
               :const POINT : usize = 32;
               :
               :/// Can be cast to a f64
               :pub trait ToF64 {
               :    /// Cast
               :    fn to_f64(&self) -> f64;
               :}
               :
               :impl Km {
               :    /// Create a new Km struct, or Km::zero if something goes wrong (NaN, Out of Bounds).
     8 2.1e-04 :    pub fn from_f64(f: f64) -> Km { /* newtypes::newtypes::Km::from_f64::hc0a1391f7a7924ec total:     50  0.0013 */
   135  0.0036 :        Km((f * (1u64<<POINT) as f64) as i64)
    25 6.6e-04 :    }
               :
               :    /// Create a new Km struct, or None if something goes wrong (NaN, Out of Bounds).
               :    ///
               :    /// # Examples
               :    /// ```
               :    /// use newtypes::Km;
               :    /// use std::u32;
               :    /// let valid = Km::from_f64_checked(0.0);
               :    /// assert!(valid.is_some());
               :    /// let invalid = Km::from_f64_checked(u32::MAX as f64 + 1.0);
               :    /// assert_eq!(invalid, None, "Failed at 1");
               :    /// let invalid = Km::from_f64_checked(-1.0);
               :    /// assert_eq!(invalid, None, "Failed at 2");
               :    /// ```
               :    pub fn from_f64_checked(f : f64) -> Option<Km> {
               :        // Beware rounding errors!
               :        if f >= Km(i64::max_value()).to_f64() {
               :            None
               :        } else if f < Km(i64::min_value()).to_f64() {
               :            None
               :        } else {Some(Km::from_f64(f))}
               :    }
               :}
               :
               :impl ToF64 for Km {
   794  0.0210 :    fn to_f64(&self) -> f64 { /* _$LT$newtypes..newtypes..Km$u20$as$u20$newtypes..newtypes..ToF64$GT$::to_f64::h707d6ae62cbc3108 total:   7582  0.2003 */
  4624  0.1221 :        self.0 as f64/((1u64 << POINT) as f64)
  2190  0.0579 :    }
               :}
               :
               :impl ToF64 for Option<Km> {
               :    fn to_f64(&self) -> f64 {
               :        self.map(|Km(u)| u as f64/((1u64 << POINT) as f64)).unwrap_or(f64::INFINITY)
               :    }
               :}
               :
               :impl Add<Km> for Km {
               :    type Output = Km;
    14 3.7e-04 :    fn add(self, other: Km) -> Km { /* _$LT$newtypes..newtypes..Km$u20$as$u20$core..ops..Add$GT$::add::h053f7e849ffe9d13 total:     27 7.1e-04 */
     4 1.1e-04 :        Km(self.0 + other.0)
     9 2.4e-04 :    }
               :}
               :
               :impl Sub<Km> for Km {
               :    type Output = Km;
    24 6.3e-04 :    fn sub(self, other: Km) -> Km { /* _$LT$newtypes..newtypes..Km$u20$as$u20$core..ops..Sub$GT$::sub::h6c37aa2f60a0ac12 total:     50  0.0013 */
     9 2.4e-04 :        Km(self.0 - other.0)
    17 4.5e-04 :    }
               :}
               :
               :impl Mul<f64> for Km {
               :    type Output = Km;
    20 5.3e-04 :    fn mul(self, other: f64) -> Km { /* _$LT$newtypes..newtypes..Km$u20$as$u20$core..ops..Mul$LT$f64$GT$$GT$::mul::h761606b809329ca3 total:    262  0.0069 */
     6 1.6e-04 :        Km::from_f64(self.to_f64() * other)
    92  0.0024 :    }
               :}
               :
               :impl Div<Km> for Km {
               :    type Output = f64;
    18 4.8e-04 :    fn div(self, other: Km) -> f64 { /* _$LT$newtypes..newtypes..Km$u20$as$u20$core..ops..Div$GT$::div::hde04d5987997ab54 total:     49  0.0013 */
    10 2.6e-04 :        (self.0 as f64 / other.0 as f64)
    21 5.5e-04 :    }
               :}
               :
               :
               :impl Div<Option<Km>> for Km {
               :    type Output = f64;
               :    fn div(self, other: Option<Km>) -> f64 {
               :        other.map(|o| (self.0 as f64 / o.0 as f64))
               :        .unwrap_or(0.0)
               :    }
               :}
               :
               :impl Zero for Km {
     6 1.6e-04 :    fn zero() -> Km { /* _$LT$newtypes..newtypes..Km$u20$as$u20$num_traits..identities..Zero$GT$::zero::hf831f5a64684b6e0 total:     23 6.1e-04 */
               :        Km(0)
    17 4.5e-04 :    }
               :
               :    fn is_zero(&self) -> bool {
               :        self.0 == 0
               :    }
               :}
               :
               :use std::fmt;
               :impl fmt::Display for Km {
               :    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
               :        self.to_f64().fmt(fmt)?;
               :        fmt.write_str(" Km")
               :    }
               :}
               :
               :#[derive(PartialEq, PartialOrd, Debug, Clone)]
               :pub struct Location {
               :    pub lon : f64,
     1 2.6e-05 :    pub lat : f64
               :}
               :
               :impl Location {
     9 2.4e-04 :    pub fn new(lon : f64, lat : f64) -> Location { /* newtypes::newtypes::Location::new::h4dadd1238fd1c66f total:     27 7.1e-04 */
               :        Location { lon : lon, lat : lat }
    18 4.8e-04 :    }
               :
               :    pub fn average(a : &Location, b : &Location) -> Location { /* newtypes::newtypes::Location::average::ha858586a920e9ab4 total:      8 2.1e-04 */
     8 2.1e-04 :        Location::new((a.lon + b.lon) / 2.0, (a.lat + b.lat) / 2.0)
               :    }
               :
               :    pub fn into_radians(self) -> (f64, f64) {
               :        (self.lat * PI / 180.0,
               :         self.lon * PI / 180.0)
               :    }
               :
  3090  0.0816 :    pub fn into_3d(&self) -> Vector3<f64> { /* newtypes::newtypes::Location::into_3d::h4132c4b148cd68c7 total: 384533 10.1579 */
  4678  0.1236 :        let rlon = self.lon * PI / 180.0;
 51898  1.3710 :        let rlat = self.lat * PI / 180.0;
 83456  2.2046 :        Vector3::new(rlon.sin() * rlat.cos(), rlon.cos() * rlat.cos(), rlat.sin())
  2226  0.0588 :    }
               :}
               :
               :use std::cmp::Eq;
               :impl Eq for Location {}
               :
               :use std::hash::{Hash, Hasher};
               :impl Hash for Location {
               :    fn hash<H>(&self, state: &mut H) where H: Hasher {
               :        use std::mem;
               :        let c : [u64; 2] = unsafe {mem::transmute_copy(self)};
               :        c.hash(state)
               :    }
               :}
               :
               :pub trait Located {
               :    fn located(&self) -> Location;
               :}
               :
               :impl Located for Location {
    12 3.2e-04 :    fn located(&self) -> Location {self.clone()} /* _$LT$newtypes..newtypes..Location$u20$as$u20$newtypes..newtypes..Located$GT$::located::hb7a453b3af13b1e2 total:     12 3.2e-04 */
               :}
/* 
 * Total samples for file : "/home/gerwin/Documents/projects/stujob/lopeningent_backend/server/routing_server/newtypes/src/newtypes.rs"
 * 
 * 153457  4.0538
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
