//! An rope.
//!
//! A rope is an efficient data structure for large mutable strings. It's
//! essentially a binary tree whose leaves are strings.
//!
//! For more information, see the following resources:
//! + http://scienceblogs.com/goodmath/2009/01/26/ropes-twining-together-strings/
//! + https://www.ibm.com/developerworks/library/j-ropes/
//! + http://citeseer.ist.psu.edu/viewdoc/download?doi=10.1.1.14.9450&rep=rep1&type=pdf

#![feature(const_fn)]
#![feature(box_syntax, box_patterns)]

use std::cmp;
use std::ops;
use std::convert;

#[derive(Debug)]
pub struct Rope {
    // can we get away with having these be of &str or will they need
    // to be string?
    root: Node
}


use self::Node::*;

/// A `Node` in the `Rope`'s tree.
///
/// A `Node` is either a `Leaf` holding a `String`, or a
/// a `Branch` concatenating together two `Node`s.
#[derive(Debug)]
enum Node {
    /// A leaf node
    Leaf(String)
  , /// A branch concatenating together `l`eft and `r`ight nodes.
    Branch {
        /// The length of this node
        len: usize
      , /// The weight of a node is the summed weight of its left subtree
        weight: usize
      , /// The left branch node
        left: Option<Box<Node>>
      , /// The right branch node
        right: Option<Box<Node>>
    }
}

/// Returns the _n_th fibonacci number.
// TODO: replace with an iterative implementation and/or lookup table?
fn fibonacci(n: usize) -> usize {
    match n { 1 => 1
            , 2 => 1
            , _ => fibonacci(n - 1) + fibonacci(n - 2)
            }
}


impl Node {
    const fn none() -> Self {
        Branch { len: 0
               , weight: 0
               , left: None
               , right: None
               }
    }

    /// Concatenate two `Node`s to return a new `Branch` node.
    fn branch(left: Self, right: Self) -> Self {
        Branch { len: left.len() + right.len()
               , weight: left.weight()
               , left: Some(box left)
               , right: Some(box right)
        }
    }

    #[inline]
    fn leaf(string: String) -> Self {
        Leaf(string)
    }

    /// Returns true if this node is balanced
    ///
    /// "We define the depth of a leaf to be 0, and the depth of a
    /// concatenation to be one plus the maximum depth of its children. Let
    /// _Fn_ be the _n_th Fibonacci number. A rope of depth _n_ is balanced if
    /// its length is at least _Fn_+2, e.g. a balanced rope of depth 1 must
    /// have length at least 2. Note that balanced ropes may contain unbalanced
    /// subropes."
    /// – from "Ropes: An Alternative to Strings"
    #[inline]
    fn is_balanced(&self) -> bool {
        self.len() >= (fibonacci(self.depth()) + 2)
    }


    /// Returns the depth in the tree of a node
    #[inline]
    fn depth(&self) -> usize {
        use std::cmp::max;

        match self {
            &Node::Leaf(_) => 0
          , &Node::Branch { ref left, ref right, ..} =>
                max( left.as_ref().map(Box::as_ref).map_or(0, Node::depth)
                   , right.as_ref().map(Box::as_ref).map_or(0, Node::depth)
                   ) + 1
            }
    }


    /// Returns the length of a node
    //  TODO: do we want to cache this?
    fn len(&self) -> usize {
        match self { &Node::Leaf(ref s) => s.len()
                   , &Node::Branch { ref left, ref right, .. } =>
                        left.as_ref().map(Box::as_ref).map_or(0, Node::len) +
                        right.as_ref().map(Box::as_ref).map_or(0, Node::len)
                    }
    }

    /// Returns the weight of a node
    fn weight (&self) -> usize {
        match self { &Node::Leaf(ref s) => s.len()
                   , &Node::Branch { ref left, .. } =>
                        left.as_ref().map(Box::as_ref).map_or(0, Node::weight)
                    }
    }

    /// Rebalance the subrope starting at this `Node`, returning a new `Node`
    ///
    /// From "Ropes: An Alternative to Strings":
    /// > "The rebalancing operation maintains an ordered sequence of (empty
    /// > or) balanced ropes, one for each length interval [_Fn_, _Fn_+1), for
    /// > _n_ >= 2. We traverse the rope from left to right, inserting each
    /// > leaf into the appropriate sequence position, depending on its length.
    ///
    /// > The concatenation of the sequence of ropes in order of decreasing
    /// > length is equivalent to the prefix of the rope we have traversed so
    /// > far. Each new leaf _x_ is inserted into the appropriate entry of the
    /// > sequence. Assume that _x_’s length is in the interval [_Fn_, _Fn_+1),
    /// > and thus it should be put in slot _n_ (which also corresponds to
    /// > maximum depth _n_ − 2). If all lower and equal numbered levels are
    /// > empty, then this can be done directly. If not, then we concatenate
    /// > ropes in slots 2, ... ,(_n_ − 1) (concatenating onto the left), and
    /// > concatenate _x_ to the right of the result. We then continue to
    /// > concatenate ropes from the sequence in increasing order to the left
    /// > of this result, until the result fits into an empty slot in the
    /// > sequence."
    fn rebalance(self) -> Self {
        unimplemented!()
    }
}

impl ops::Add for Node {
    type Output = Self;
    /// Concatenate two `Node`s, returning a `Branch` node.
    fn add(self, right: Self) -> Self { Node::branch(self, right) }
}

impl Rope {

    /// Returns a new empty Rope
    ///
    /// # Examples
    /// ```
    /// use an_rope::Rope;
    /// let mut an_rope = Rope::<u8>::new();
    /// assert_eq!(an_rope.len(), 0);
    /// ```
    pub const fn new() -> Rope {
        Rope { root: Node::none() }
    }

    /// Returns the length of this Rope
    ///
    /// # Examples
    /// ```
    /// use an_rope::Rope;
    /// let mut an_empty_rope = Rope::new();
    /// assert_eq!(an_empty_rope.len(), 0);
    /// ```
    /// ```
    /// use an_rope::Rope;
    /// let mut an_empty_rope = Rope::from(String::from(""));
    /// assert_eq!(an_empty_rope.len(), 0);
    /// ```
    /// ```
    /// use an_rope::Rope;
    /// let mut an_rope = Rope::from(String::from("a string"));
    /// assert_eq!(an_rope.len(), "a string".len());
    /// ```
    pub fn len(&self) -> usize {
        // self.root.len()
        unimplemented!()
    }

    /// Appends a `Rope` to the end of this `Rope`, returning a new `Rope`
    ///
    /// Note that this is equivalent to using the `+` operator.
    ///
    /// # Examples
    /// ```
    /// use an_rope::Rope;
    /// let an_rope = Rope::from(String::from("abcd"));
    /// let another_rope = an_rope.merge(Rope::from(String::from("efgh")))
    /// assert_eq!(another_rope, Rope::from(String::from("abcdefgh")));
    /// ```
    pub fn merge(&self, other: &Rope) -> Rope {
        unimplemented!()
    }

    /// Appends a `Rope` to the end of this `Rope`, updating it in place.
    ///
    /// Note that this is equivalent to using the `+=` operator.
    ///
    /// # Examples
    /// ```
    /// use an_rope::Rope;
    /// let mut an_rope = Rope::from(String::from("abcd"));
    /// an_rope.update(Rope::from(String::from("efgh")))
    /// assert_eq!(an_rope, Rope::from(String::from("abcdefgh")));
    /// ```
    pub fn update(&mut self, other: Rope) {
        unimplemented!()
    }

    /// Splits the rope into two ropes at the given index.
    ///
    /// Consumes this rope.
    ///
    /// # Examples
    /// ```
    /// use an_rope::Rope;
    /// let mut an_rope = Rope::from(String::from("abcd"));
    /// let (ab, cd) = an_rope.split(2);
    /// assert_eq!(ab, Rope::from(String::from("ab")));
    /// assert_eq!(cd, Rope::from(String::from("cd")));
    /// ```
    pub fn split(self, index: usize) -> (Rope, Rope) {
        unimplemented!()
    }

    /// Rebalances this entire `Rope`, returning a balanced `Rope`.
    #[inline]
    fn rebalance(self) -> Self {
        if self.is_balanced() {
            // the rope is already balanced, do nothing
            self
        } else {
            // rebalance the rope
            Rope { root: self.root.rebalance() }
        }
    }

    /// Returns true if this `Rope` is balanced.
    ///
    /// Balancing invariant:
    /// the rope length needs to be less than _F_(rope_length) where F is fibonacci
    #[inline]
    fn is_balanced(&self) -> bool {
        self.root.is_balanced()
    }
}


impl ops::Index<usize> for Node {
    type Output = str;

    fn index(&self, i: usize) -> &str {
        let len = self.len();
        match *self { Node::Leaf(ref vec) => { &vec[i..i+1] }
                    , Node::Branch { right: Some(box ref r), .. } if len < i =>
                        &r[i - len]
                    , Node::Branch { left: Some(box ref l), .. } => &l[i]
                    , _ =>
                        panic!("Index out of bounds!")
                    }
    }
}

impl convert::Into<Vec<u8>> for Rope {
    fn into(self) -> Vec<u8> {
        unimplemented!()
    }

}

impl convert::From<String> for Rope {
    fn from(string: String) -> Rope {
        Rope {
            root: if string.len() == 0 { Node::none() }
                  else { Node::Leaf(string) }
        }
    }
}

//-- comparisons ----------------------------------------------------
impl cmp::PartialEq for Rope {
    fn eq(&self, other: &Rope) -> bool {
        unimplemented!()
    }
}

impl cmp::PartialEq<str> for Rope {
    fn eq(&self, other: &str) -> bool {
        unimplemented!()
    }
}

//-- concatenation --------------------------------------------------
impl<'a> ops::Add for &'a Rope {
    type Output = Rope;
    /// Non-destructively concatenate two `Rope`s, returning a new `Rope`.
    ///
    /// # Examples
    /// ```
    /// let rope = Rope::from(String::from("ab"));
    /// assert_eq!( &rope + &Rope::from(String::from("cd"))
    ///           , Rope::from(String::from("abcd")));
    /// ```
    #[inline] fn add(self, other: Self) -> Rope { self.merge(other) }

}

impl ops::Add for Rope {
    type Output = Rope;
    /// Non-destructively concatenate two `Rope`s, returning a new `Rope`.
    ///
    /// # Examples
    /// ```
    /// let rope = Rope::from(String::from("ab"));
    /// assert_eq!( rope + Rope::from(String::from("cd"))
    ///           , Rope::from(String::from("abcd")));
    /// ```
    #[inline] fn add(self, other: Self) -> Rope { self.merge(&other) }
}

impl ops::Add<String> for Rope {
    type Output = Rope;
    /// Non-destructively concatenate a `Rope` and a `String`.
    ///
    /// Returns a new `Rope`
    ///
    /// # Examples
    /// ```
    /// let rope = Rope::from(String::from("ab"));
    /// assert_eq!( rope + String::from("cd"))
    ///           , Rope::from(String::from("abcd")));
    /// ```
    #[inline] fn add(self, other: String) -> Rope {
         self.merge(&Rope::from(other))
    }
}


impl<'a, 'b> ops::Add<&'b str> for &'a Rope {
    type Output = Rope;
    /// Non-destructively concatenate a `Rope` and an `&str`.
    ///
    /// Returns a new `Rope`
    ///
    /// # Examples
    /// ```
    /// let rope = Rope::from(String::from("ab"));
    /// assert_eq!( &rope + "cd")
    ///           , Rope::from(String::from("abcd")));
    /// ```
    #[inline] fn add(self, other: &'b str) -> Rope {
         self.merge(&Rope::from(other.to_owned()))
     }

}

impl<'a> ops::Add<&'a str> for Rope {
    type Output = Rope;
    /// Non-destructively concatenate a `Rope` and an `&str`.
    ///
    /// Returns a new `Rope`
    ///
    /// # Examples
    /// ```
    /// let rope = Rope::from(String::from("ab"));
    /// assert_eq!( rope + "cd")
    ///           , Rope::from(String::from("abcd")));
    /// ```
    #[inline] fn add(self, other: &'a str) -> Rope {
         self.merge(&Rope::from(other.to_owned()))
     }

}


impl ops::AddAssign for Rope {

    /// Concatenate two `Rope`s mutably.
    ///
    /// # Examples
    /// ```
    /// let mut rope = Rope::from(String::from("ab"));
    /// rope += Rope::from(String::from("cd"));
    /// assert_eq!(rope, Rope::from(String::from("abcd")));
    /// ```
    #[inline]
    fn add_assign(&mut self, other: Rope) {
        self.update(other)
    }
}

impl ops::AddAssign<String> for Rope {

    /// Concatenate a `String` onto a `Rope` mutably.
    ///
    /// # Examples
    /// ```
    /// let mut rope = Rope::from(String::from("ab"));
    /// rope += String::from("cd");
    /// assert_eq!(rope, Rope::from(String::from("abcd")));
    /// ```
    #[inline]
    fn add_assign(&mut self, string: String) {
        self.update(Rope::from(string))
    }
}

impl<'a> ops::AddAssign<&'a str> for Rope {

    /// Concatenate an `&str` onto a `Rope` mutably.
    ///
    /// # Examples
    /// ```
    /// let mut rope = Rope::from(String::from("ab"));
    /// rope += String::from("cd");
    /// assert_eq!(rope, Rope::from(String::from("abcd")));
    /// ```
    #[inline]
    fn add_assign(&mut self, string: &'a str) {
        self.update(Rope::from(string.to_owned()))
    }
}

impl ops::Index<usize> for Rope {
    type Output = str;

    /// Recursively index the Rope to return the `i` th character.
    ///
    /// # Examples
    /// ```
    /// use an_rope::Rope;
    /// let an_rope = Rope::from(String::from("abcd"));
    /// assert_eq!(&an_rope[0], "a");
    /// assert_eq!(&an_rope[1], "b");
    /// assert_eq!(&an_rope[2], "c");
    /// assert_eq!(&an_rope[3], "d");
    /// ```
    ///
    /// # Time complexity
    /// _O_(log _n_)
    ///
    #[inline]
    fn index(&self, i: usize) -> &str {
        &self.root[i]
    }
}

//-- slicing operators ----------------------------------------------
impl ops::Index<ops::Range<usize>> for Rope {
    type Output = str;

    // Index a substring
    fn index(&self, i: ops::Range<usize>) -> &str {
        unimplemented!()
    }
}

impl ops::Index<ops::RangeTo<usize>> for Rope {
    type Output = str;

    fn index(&self, i: ops::RangeTo<usize>) -> &str {
        unimplemented!()
    }
}

impl ops::Index<ops::RangeFrom<usize>> for Rope {
    type Output = str;

    fn index(&self, i: ops::RangeFrom<usize>) -> &str {
        unimplemented!()
    }
}

impl ops::IndexMut<ops::Range<usize>> for Rope {
    fn index_mut(&mut self, i: ops::Range<usize>) -> &mut str {
        unimplemented!()
    }
}

impl ops::IndexMut<ops::RangeTo<usize>> for Rope {
    fn index_mut(&mut self, i: ops::RangeTo<usize>) -> &mut str {
        unimplemented!()
    }
}

impl ops::IndexMut<ops::RangeFrom<usize>> for Rope {
    fn index_mut(&mut self, i: ops::RangeFrom<usize>) -> &mut str {
        unimplemented!()
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
