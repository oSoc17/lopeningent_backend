//! Error type crate.
//!
//! This is the module-level error type. Feel free to add extra enum variants to the
//! type if necessary.

/// Error type.
#[derive(Debug, PartialEq)]
pub enum Error {
    /// An edge has been added to the graph, connecting two nodes that don't exist.
    ///
    /// # Examples
    /// ```
    /// use Graph;
    /// use error::Error;
    /// let graph = Graph::new(
    ///             vec![(0, "A"), (5, "B")],
    ///             vec![(1, "Edge from _ to _", 2)]
    ///     );
    ///
    /// assert_eq!(graph.unwrap_err(), Error::MissingID);
    /// ```
    ///
    /// Note that only the source node matters:
    ///
    /// ```
    /// # use Graph;
    /// let graph = Graph::new(
    ///             vec![(0, "A"), (5, "B")],
    ///             vec![(0, "Edge from A to _", 2)]
    ///     );
    ///
    /// assert!(graph.is_ok());
    /// ```
    ///
    /// However, other methods might fail.
    ///
    /// ```
    /// # use Graph;
    /// # let graph = Graph::new(
    /// #             vec![(0, "A"), (5, "B")],
    /// #             vec![(0, "Edge from A to _", 2)]
    /// #     );
    /// #
    /// # assert!(graph.is_ok());
    /// # let graph = graph.unwrap();
    /// #
    /// let iter = graph.get_connids(0).unwrap().filter_map(|n| graph.get(n));
    /// assert_eq!(iter.count(), 0);
    /// ```
    MissingID,
    /// Something else.
    Variant(String),
}

use std::fmt;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

use std::error as stderr;
impl stderr::Error for Error {
    fn description(&self) -> &str {
        "Graph error"
    }
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

impl_from!(String, Error::Variant);
