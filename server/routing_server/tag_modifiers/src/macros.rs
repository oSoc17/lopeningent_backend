
/// Creates the entire structure.
///
/// This macro also generates structure and trait definitions, so calling it twice in the same module will create naming collisions.
macro_rules! struct_tag {
    {$($field : ident),*} => {
        /// Structure holding the tags.
        #[derive(Debug, Default)]
        pub struct Tags {

            $(
                /// Whether a poi with tag $field exists close to this edge.
                pub $field : bool
            ),*
        }

        impl Tags {
            /// Retrieve a tag
            pub fn from<I : IntoIterator<Item=S>, S : AsRef<str>>(t : I) -> Tags {
                let mut res = Tags::default();
                for i in t {
                    match i.as_ref() {
                        $(stringify!($field) => res.$field = true,)*
                        _ => ()
                    }
                }
                res
            }

            /// Counts the number of true's in this struct.
            pub fn trues(&self) -> usize {
                $((if self.$field {1} else {0})+)* 0
            }
        }
        /// Translate an abstract tag in a concrete potential cost.
        pub trait TagModifier {
            /// Map the abstract tag onto a concrete cost.
            fn tag_modifier(&self, tag : &Tags) -> f64;
            /// Limit the range of the potential.
            fn tag_bounds() -> (f64, f64) {
                (0.5, 2.0)
            }
        }

        /// Simple implementation of a `TagModifier`.
        #[derive(Debug, Default, Clone)]
        pub struct TagConverter {
            $(
                /// The importance of $field during generation.
                pub $field : f64
            ),*
        }

        impl TagConverter {
            /// Adds a tag with a certain importance to the converter.
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
