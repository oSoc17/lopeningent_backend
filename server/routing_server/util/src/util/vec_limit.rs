use std::error::Error;

const DEFAULT_MAX_SIZE : usize = 5_000_000;

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

pub struct LimitedVec<T> {
    inner : Vec<T>,
    max_size : usize,
}

impl<T> LimitedVec<T> {
    pub fn new(vec : Vec<T>) -> Self {
        LimitedVec {
            inner : vec,
            max_size : DEFAULT_MAX_SIZE,
        }
    }

    fn assert_valid(&self) -> bool {
        self.inner.len() <= self.max_size
    }

    pub fn push(&mut self, data : T) -> Result<(), OOMError> {
        if self.assert_valid() {
            self.inner.push(data);
            Ok(())
        } else {
            Err(OOMError)
        }
    }

    pub fn inner(&self) -> &Vec<T> {
        &self.inner
    }

    pub fn get(&self, index : usize) -> Option<&T> {
        self.inner.get(index)
    }

    pub fn get_mut(&mut self, index : usize) -> Option<&mut T> {
        self.inner.get_mut(index)
    }

    pub fn into_inner(self) -> Vec<T> {
        self.inner
    }
}
