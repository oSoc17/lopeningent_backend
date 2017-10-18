//! Collection of iterators over a graph.
//!
//! This module contains various iterators, meant to be returned from the different
//! iterator functions.

use std::collections::HashMap;
use std::collections::btree_map::Iter as BTreeIter;
use std::cell::Ref;

use graph::Element;
use vec_map::Keys;
use vec_map::Values;

use NodeID;
use EdgeID;

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

iter_impl!(ConnIdVal<E>, BTreeIter<NodeID, E>);
iter_impl!(IterEdges<E>, ConnIdVal<E>);
iter_impl!(IterConnIds<E>, ConnIdVal<E>);
iter_impl!(ListIds<V, E>, Keys<Element<V, E>>);
iter_impl!(ListAllNodes<V, E>,  Values<Element<V, E>>);

/// Iterator for a root structure.
///
/// The root iterator transforms a linked hashmap into a string of indices.
pub struct Root<'a> {
    tree : Ref<'a, HashMap<NodeID, Option<NodeID>>>,
    start : NodeID,
}

impl<'a> Root<'a> {
    /// Create a new iterator for a linked hashmap.
    pub fn new(tree : Ref<'a, HashMap<NodeID, Option<NodeID>>>, start : NodeID) -> Root<'a> {
        Root {tree : tree, start : start}
    }
}

impl<'a, E : 'a> Iterator for ConnIdVal<'a, E> {
    type Item = (EdgeID, &'a E);
    fn next(&mut self) -> Option<(NodeID, &'a E)> {
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
    type Item = EdgeID;
    fn next(&mut self) -> Option<NodeID> {
        self.element.next().map(|x| x.0)
    }
}

impl<'a, V : 'a, E : 'a> Iterator for ListIds<'a, V, E> {
    type Item = NodeID;
    fn next(&mut self) -> Option<NodeID> {
        self.element.next().map(|x| x as NodeID)
    }
}

impl<'a, V : 'a, E : 'a> Iterator for ListAllNodes<'a, V, E> {
    type Item = &'a V;
    fn next(&mut self) -> Option<&'a V> {
        self.element.next().map(|el| &el.v)
    }
}

impl<'a> Iterator for Root<'a> {
    type Item = NodeID;
    fn next(&mut self) -> Option<NodeID> {
        let res = self.tree.get(&self.start).and_then(|&t| t);
        if let Some(t) = res {self.start = t;}
        res
    }
}
