//! Collection of iterators over a graph.
//!
//! This module contains various iterators, meant to be returned from the different
//! iterator functions.

use std::collections::HashMap;
use std::collections::btree_map::Iter as BTreeIter;
use std::cell::Ref;

use graph::graph::Element;
use vec_map::Keys;

macro_rules! iter_impl {
    (
        $name:ident < $($typaram:ident),* >,
        $subtype:ident < $($stparam:ty),* >
    ) => {
        /// Iterator for a Graph structure (see [Graph](../struct.Graph.html)).
        pub struct $name <'a, $($typaram),*>
            where $($typaram: 'a),*
        {
            element : $subtype <'a, $($stparam),* >
        }

        impl<'a, $($typaram),*> $name< 'a, $($typaram),*>
            where $($typaram: 'a),*
        {
            /// Create a new iterator from a parent structure.
            pub fn new(element : $subtype <'a, $($stparam),* > ) -> Self {
                $name {
                    element : element
                }
            }
        }
    }
}

iter_impl!(ConnIdVal<E>, BTreeIter<usize, E>);
iter_impl!(IterEdges<E>, ConnIdVal<E>);
iter_impl!(IterConnIds<E>, ConnIdVal<E>);
iter_impl!(ListIds<V, E>, Keys<Element<V, E>>);

/// Iterator for a root structure.
///
/// The root iterator transforms a linked hashmap into a string of indices.
pub struct Root<'a> {
    tree : Ref<'a, HashMap<usize, Option<usize>>>,
    start : usize,
}

impl<'a> Root<'a> {
    /// Create a new iterator for a linked hashmap.
    pub fn new(tree : Ref<'a, HashMap<usize, Option<usize>>>, start : usize) -> Root<'a> {
        Root {tree : tree, start : start}
    }
}

impl<'a, E : 'a> Iterator for ConnIdVal<'a, E> {
    type Item = (usize, &'a E);
    fn next(&mut self) -> Option<(usize, &'a E)> {
        self.element.next().map(|(&n, e)| (n, e))
    }
}

impl<'a, E : 'a> Iterator for IterEdges<'a, E> {
    type Item = &'a E;
    fn next(&mut self) -> Option<&'a E> {
        self.element.next().map(|x| x.1)
    }
}

impl<'a, E : 'a> Iterator for IterConnIds<'a, E> {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        self.element.next().map(|x| x.0)
    }
}

impl<'a, V : 'a, E : 'a> Iterator for ListIds<'a, V, E> {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        self.element.next()
    }
}

impl<'a> Iterator for Root<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        let res = self.tree.get(&self.start).and_then(|&t| t);
        if let Some(t) = res {self.start = t;}
        res
    }
}
