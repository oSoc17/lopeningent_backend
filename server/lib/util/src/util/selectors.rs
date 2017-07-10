//! Random Value Selector
//!
//! This module contains the Selector struct: a structure that takes in a bunch of
//! elements and probabilities, and yields a random element.
//!
//! # Examples
//! ```
//! use util::selectors::Selector;
//! let data = vec!["A", "B", "C", "D", "E"];
//!
//! let mut selector = Selector::new_default_rng();
//!
//! for val in &data {
//!     // every element has an equal probability.
//!     selector.update(1.0, val);
//! }
//!
//! let random_element = selector.decompose().unwrap();
//! assert!(data.contains(&random_element));
//! ```

use rand::Rng;
use std::mem;

/// Selector struct
pub struct Selector<T, R : Rng> {
    counter : f64,
    rng : R,
    value : Option<T>,
}

use rand::ThreadRng;
impl<T> Selector<T, ThreadRng> {
    /// Create a new selector using a default random number generator.
    ///
    /// Equivalent to `Selector::new(rand::thread_rng())`.
    pub fn new_default_rng() -> Self {
        use rand::thread_rng;
        Selector::new(thread_rng())
    }
}

impl<T, R : Rng> Selector<T, R> {
    /// Create a new selector
    pub fn new(rng : R) -> Self {
        Selector {
            counter : 0.0,
            rng : rng,
            value : None,
        }
    }

    /// Consume the iterator, creating a new selector
    pub fn from_iterator<I, F>(it : I, oddf : F, rng : R) -> Selector<T, R>
        where I : IntoIterator<Item=T>,
            F : Fn(&T) -> f64,
    {
        let mut res = Selector::new(rng);
        for t in it {
            res.update(oddf(&t), t);
        }
        res
    }

    /// Update the selector, replacing the original value with a new value with
    /// a probability p/sum(p)
    pub fn update(&mut self, odds : f64, value : T) -> Option<T> {
        self.counter += odds;
        if self.rng.next_f64() * self.counter < odds {
            mem::replace(&mut self.value, Some(value))
        }
        else {
            Some(value)
        }
    }

    /// Consumes itself, yielding a random value, or nothing if the selector had no data.
    pub fn decompose(self) -> Option<T> {
        self.value
    }
}
