use std::cmp::PartialEq;

pub trait Majorising : PartialEq {
    fn majorises(&self, other : &Self) -> bool;
    fn majorises_strict(&self, other : &Self) -> bool {
        if self == other {false} else {self.majorises(other)}
    }
}


macro_rules! implMajorising {
    (
        $type : ty
    ) => {
        impl Majorising for $type {
            fn majorises(&self, other : &Self) -> bool {
                self >= other
            }
        }
    }
}

implMajorising!(usize);
implMajorising!(isize);
implMajorising!(u8);
implMajorising!(u16);
implMajorising!(u32);
implMajorising!(u64);
implMajorising!(i8);
implMajorising!(i16);
implMajorising!(i32);
implMajorising!(i64);
implMajorising!(f32);
implMajorising!(f64);

macro_rules! implTupleMajorising {
    ($v0 : ident $(,$v : ident)* ; $i0 : tt $(,$i : tt)*)
    => {
        impl<$v0 : Majorising, $($v : Majorising),*> Majorising for ($v0, $($v,)*)
        {
            fn majorises(&self, other : &Self) -> bool {
                if !self.$i0.majorises(&other.$i0) {return false;}
                $(if !self.$i.majorises(&other.$i) {return false;})*
                return true;
            }
        }
        implTupleMajorising!($($v),* ; $($i),*);
    };
    (;) => {};
}

implTupleMajorising!(A, B, C, D, E, F, G, H; 7, 6, 5, 4, 3, 2, 1, 0);

#[test]
fn test_majorisation() {
    assert!(55.majorises(&55));
}
