#![warn(missing_docs)]

//! Crate for managing tags, like tourism, water, or monuments.
//!
//! Adding a new tag to the structure is simple: just look in the file lib.rs and add the necessary tag to the system.
//!
//! The tags are lexically connected to the tags in the database: a field named 'tourism' corresponds with the "tourism" string.

#[macro_use]
mod macros;

struct_tag! {
    toerisme,
    monumenten,
    water,
    park,
    ugent
}
