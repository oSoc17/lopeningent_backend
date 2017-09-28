macro_rules! struct_tag {
    {$($field : ident),*} => {
        #[derive(Debug, Default)]
        pub struct Tags {
            $(pub $field : bool),*
        }

        impl Tags {
            pub fn from(t : Vec<String>) -> Tags {
                let mut res = Tags::default();
                for i in t {
                    match i.as_ref() {
                        $(stringify!($field) => res.$field = true,)*
                        _ => ()
                    }
                }
                res
            }
            pub fn trues(&self) -> usize {
                $((if self.$field {1} else {0})+)* 0
            }
        }

        pub trait TagModifier {
            fn tag_modifier(&self, tag : &Tags) -> f64;
            fn tag_bounds() -> (f64, f64) {
                (0.5, 2.0)
            }
        }

        #[derive(Debug, Default)]
        pub struct TagConverter {
            $(pub $field : f64),*
        }

        impl TagConverter {
            pub fn add(&mut self, tag : &str, size : f64) {
                match tag.as_ref() {
                    $(stringify!($field) => self.$field += size,)*
                    _ => (),
                }
            }
        }

        impl TagModifier for TagConverter {
            fn tag_modifier(&self, tag : &Tags) -> f64 {
                $((if tag.$field {self.$field} else {0.0}) + )* 0.0
            }
        }
    }
}
