use metric::{Measured, Line, Grapheme, Metric};
use super::{NodeLink, LeafRepr };

use self::Value::*;

use std::cell::Cell;
use std::convert;
use std::default::Default;
use std::fmt;
use std::ops;



/// A lazily-evaluated field
#[derive(Clone)]
struct Lazy<T: Copy>(Cell<Option<T>>);

impl<T> Lazy<T>
where T: Copy {

    #[inline]
    pub fn get(&self) -> Option<T> { self.0.get() }

    #[inline]
    pub fn get_or_else<F>(&self, f: F) -> T
    where F: FnOnce() -> T {
        if let Some(value) = self.0.get() {
            value
        } else {
            let value = f();
            self.0.set(Some(value));
            value
        }
    }

    #[inline]
    pub fn new() -> Self {
        Lazy(Cell::new(None))
    }

}

impl<T> Default for Lazy<T>
where T: Copy {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> fmt::Debug for Lazy<T>
where T: fmt::Debug
    , T: Copy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0.get() { Some(value) => value.fmt(f)
                           , None => write!(f, "?")

        }
    }
}

macro_rules! lazy_field {
    ($method: ident, $field: ident, $ty:ty) => {
        #[inline] fn $method(&self) -> $ty {
            self.$field.get_or_else(|| { self.value.$method() })
        }

    }
}

/// A `Node`.
#[derive(Clone, Debug, Default)]
pub struct Node { len: Lazy<usize>
                , weight: Lazy<usize>
                , line_count: Lazy<Line>
                , line_weight: Lazy<Line>
                , grapheme_count: Lazy<Grapheme>
                , grapheme_weight: Lazy<Grapheme>
                , pub value: Value
                }

impl Node {
    pub fn new(value: Value) -> Self {
        Node { value: value, ..Default::default() }
    }

    pub fn spanning(&self, i: usize, span_len: usize) -> (&Node, usize)
    where Node: Measured<usize> {
        assert!(self.len() >= span_len);
        match **self {
            Branch { ref right, ref left } if <Node as Measured<usize>>::measure_weight(self) < i => {
                // if this node is a branch, and the weight is less than the
                // index, where the span begins, then the first index of the
                // span is on the right side
                let span_i = or_zero!(i, left.len());
                assert!(or_zero!(right.len(), span_i) >= span_len);
                right.spanning(span_i, span_len)
            }
          , Branch { ref left, .. }
            // if the left child is long enough to contain the entire span,
            // walk to the left child
            if or_zero!(left.len(), i) >= span_len => left.spanning(i, span_len)
         ,  Leaf(_) | Branch {..} =>
            // if this function has walked as far as a leaf node,
            // then that leaf must be the spanning node. return it;
            //
            // otherwise, if the node is a branch node and the span is longer
            // than the left child, then this node must be the minimum
            // spanning node
            (self, i)
        }
    }
}


impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.strings()
            .fold(Ok(()), |r, string| r.and_then(|_| write!(f, "{}", string)))
    }
}

impl convert::Into<NodeLink> for Node {
    #[inline] fn into(self) -> NodeLink {
        NodeLink::new(self)
    }
}

impl ops::Deref for Node {
    type Target = Value;
    fn deref(&self) -> &Value { &self.value }
}

impl Measured<usize> for Node {

        #[inline] fn to_byte_index(&self, index: usize) -> Option<usize>  {
             Some(index)
        }

        lazy_field!(measure, len, usize);
        lazy_field!(measure_weight, weight, usize);

}

impl Measured<Grapheme> for Node {

        #[inline] fn to_byte_index(&self, index: Grapheme) -> Option<usize>  {
            self.value.to_byte_index(index)
        }

        lazy_field!(measure, grapheme_count, Grapheme);
        lazy_field!(measure_weight, grapheme_weight, Grapheme);

}

impl Measured<Line> for Node {

        #[inline] fn to_byte_index(&self, index: Line) -> Option<usize>  {
            self.value.to_byte_index(index)
        }

        lazy_field!(measure, line_count, Line);
        lazy_field!(measure_weight, line_weight, Line);

}


impl<M> ops::Index<M> for Node
where M: Metric
    , Node: Measured<M>
    , LeafRepr: Measured<M>
    {
    type Output = str;

    fn index(&self, i: M) -> &str {
        let len = self.measure();
        assert!( i < len
               , "Node::index: index {:?} out of bounds (length {:?})", i, len);
        match **self {
            Leaf(ref string) => {
                let idx = string.to_byte_index(i)
                                .expect("index out of bounds!");
                &string[idx..idx+1]
            }
          , Branch { ref right, .. } if len < i =>
                &right[i - len]
          , Branch { ref left, .. } => &left[i]
        }
    }
}


/// A `Node` in the `Rope`'s tree.
///
/// A `Node` is either a `Leaf` holding a `String`, or a
/// a `Branch` concatenating together two `Node`s.
#[derive(Clone, Debug)]
pub enum Value {
    /// A leaf node
    Leaf(LeafRepr)
  , /// A branch concatenating together `l`eft and `r`ight nodes.
    Branch { /// The left branch node
             left: NodeLink
           , /// The right branch node
             right: NodeLink }
}

impl Value {
    #[inline]
    pub fn new_branch(left: NodeLink, right: NodeLink) -> Self {
        Branch { left: left, right: right }
    }
}

impl<M> Measured<M> for Value
where M: Metric
    , LeafRepr: Measured<M>
    , Node: Measured<M>
{
    fn to_byte_index(&self, index: M) -> Option<usize> {
        unimplemented!()
    }

    fn measure(&self) -> M {
        match *self {
            Leaf(ref r) => r.measure()
          , Branch { ref left, ref right } =>
                left.measure() + right.measure()
        }
    }

    fn measure_weight(&self) -> M {
        match *self {
            Leaf(ref r) => r.measure_weight()
          , Branch { ref left, ref right } =>
                left.measure()
        }
    }

}

impl convert::Into<Node> for Value {
    #[inline] fn into(self) -> Node {
        Node::new(self)
    }
}

impl Default for Value {
    fn default() -> Self {
        Leaf(LeafRepr::default())
    }
}
