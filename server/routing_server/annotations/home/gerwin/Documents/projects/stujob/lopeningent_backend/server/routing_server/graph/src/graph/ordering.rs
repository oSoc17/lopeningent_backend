               :use std::cmp::PartialEq;
               :
               :pub trait Majorising : PartialEq {
               :    fn majorises(&self, other : &Self) -> bool;
               :    fn majorises_strict(&self, other : &Self) -> bool {
               :        if self == other {false} else {self.majorises(other)}
               :    }
               :}
               :
               :
               :macro_rules! implMajorising {
               :    (
               :        $type : ty
               :    ) => {
               :        impl Majorising for $type {
 83815  2.2141 :            fn majorises(&self, other : &Self) -> bool { /* _$LT$f64$u20$as$u20$graph..graph..ordering..Majorising$GT$::majorises::hb7dd2046e4772836 total: 316249  8.3541 */
               :                self >= other
 51523  1.3610 :            }
               :        }
               :    }
               :}
               :
               :implMajorising!(usize);
               :implMajorising!(isize);
               :implMajorising!(u8);
               :implMajorising!(u16);
               :implMajorising!(u32);
               :implMajorising!(u64);
               :implMajorising!(i8);
               :implMajorising!(i16);
               :implMajorising!(i32);
               :implMajorising!(i64);
               :implMajorising!(f32);
               :implMajorising!(f64);
               :
               :macro_rules! implTupleMajorising {
               :    ($v0 : ident $(,$v : ident)* ; $i0 : tt $(,$i : tt)*)
               :    => {
               :        impl<$v0 : Majorising, $($v : Majorising),*> Majorising for ($v0, $($v,)*)
               :        {
               :            fn majorises(&self, other : &Self) -> bool {
  6984  0.1845 :                if !self.$i0.majorises(&other.$i0) {return false;}
 23365  0.6172 :                $(if !self.$i.majorises(&other.$i) {return false;})*
               :                return true;
               :            }
               :        }
               :        //implTupleMajorising!($($v),* ; $($i),*);
               :    };
               :    (;) => {};
               :}
               :
               :implTupleMajorising!(A ; 0);
               :implTupleMajorising!(A, B ; 1, 0);
               :
               :#[test]
               :fn test_majorisation() {
               :    assert!(55.majorises(&55));
               :}
/* 
 * Total samples for file : "/home/gerwin/Documents/projects/stujob/lopeningent_backend/server/routing_server/graph/src/graph/ordering.rs"
 * 
 * 165687  4.3768
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
