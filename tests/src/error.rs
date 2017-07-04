//! Error type.
//!
//! No further comments.

use curl;
use std::num::ParseIntError;
use std::num::ParseFloatError;
use std::sync::mpsc::RecvError;

#[derive(Debug)]
pub enum Error {
    CurlError(curl::Error),
    StringError(String),
    CodeError(String, usize, String),
    ParseIntError(ParseIntError),
    ParseFloatError(ParseFloatError),
    RcvxError(RecvError),
}

macro_rules! impl_from {
    ($from:ty, $into:expr) => {
        impl From<$from> for Error {
            fn from(err : $from) -> Error {
                $into(err)
            }
        }
    }
}

impl_from!(curl::Error, Error::CurlError);
impl_from!(String, Error::StringError);
impl_from!(ParseIntError, Error::ParseIntError);
impl_from!(ParseFloatError, Error::ParseFloatError);
impl_from!(RecvError, Error::RcvxError);
