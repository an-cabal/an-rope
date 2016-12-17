//! A generic binary tree implementation.

use std::iter::{Chain, Iterator, IntoIterator};
use std::mem;

use self::Node::*;

#[derive(Debug)]
pub enum Node<T> { /// A leaf node
                   Leaf(T)
                 , /// A branch node
                   Branch { l: Box<Node<T>>, r: Box<Node<T>> }
                 , /// Nothing
                   None
}


trait Take<T> {
    fn take(&mut self) -> Node<T>;
}

impl<T> Take<T> for Box<Node<T>> {

    /// Take the value out of a `Node`, replacing it with `None`
    #[inline]
    fn take(&mut self) -> Node<T> {
        mem::replace(self, None)
    }
}
//
// impl<'a, T, N, I> IntoIterator for &'a Node<T>
// where T: IntoIterator<Item = &'a I, IntoIter = N>
//     , N: Iterator<Item = &'a I>
//     , I: 'a {
//     type Item = &'a I;
//     type IntoIter = Chain<N, N>;
//
//     fn into_iter(self) -> Self::IntoIter {
//         match self {
//
//         }
//     }
// }

impl<T> Node<T> {

    /// Concatenate together two nodes, returning a new `Branch` node
    pub fn branch(self, other: Node<T>) -> Node<T> {
        unimplemented!()
    }

    /// Returns the height in the tree of a node
    #[inline]
    fn height(&self) -> usize {
        use std::cmp::max;

        match *self { Node::Leaf(_) => 1
                    , Node::Branch { box ref l, box ref r} =>
                        max(r.height(), l.height()) + 1
                    , Node::None => 0
                    }
    }
}
