//! Module for limited vectors.
//!
//! If your code can create arbitrary length vectors, going out of memory may bring down the entire server.
//! This class will simply error when the size is too large.

use std::error::Error;

const DEFAULT_MAX_SIZE : usize = 5_000_000;

/// Out of memory error.
#[derive(Debug)]
pub struct OOMError;

use std::fmt;
impl fmt::Display for OOMError {
    fn fmt(&self, f : &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Out of memory error!")
    }
}

impl Error for OOMError {
    fn description(&self) -> &str {
        "The vector got out of memory!"
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

/// Vector that is limited in length.
pub struct LimitedVec<T> {
    inner : Vec<T>,
    max_size : usize,
}

impl<T> LimitedVec<T> {
    /// Create a new limited vector from a blueprint.
    pub fn new(vec : Vec<T>) -> Self {
        LimitedVec {
            inner : vec,
            max_size : DEFAULT_MAX_SIZE,
        }
    }

    fn assert_valid(&self) -> bool {
        self.inner.len() <= self.max_size
    }

    /// Push an element on the vector.
    pub fn push(&mut self, data : T) -> Result<(), OOMError> {
        if self.assert_valid() {
            self.inner.push(data);
            Ok(())
        } else {
            Err(OOMError)
        }
    }

    /// Retrieve a reference to the inner vector.
    pub fn inner(&self) -> &Vec<T> {
        &self.inner
    }

    /// Get a reference to the element at an index.
    pub fn get(&self, index : usize) -> Option<&T> {
        self.inner.get(index)
    }

    /// Get a reference to the element at an index.
    pub fn get_mut(&mut self, index : usize) -> Option<&mut T> {
        self.inner.get_mut(index)
    }

    /// Unwraps this limited vec.
    pub fn into_inner(self) -> Vec<T> {
        self.inner
    }
}
