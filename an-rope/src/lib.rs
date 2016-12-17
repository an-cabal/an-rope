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
pub struct Rope<T> {
    // can we get away with having these be of &str or will they need
    // to be string?
    root: Node<T>
}


use self::Node::*;

/// A `Node` in the `Rope`'s tree.
///
/// A `Node` is either a `Leaf` holding a vector of `T`, or a
/// a `Branch` concatenating together two `Node`s.
#[derive(Debug)]
enum Node<T> {
    /// A leaf node
    Leaf(Vec<T>)
  , /// A branch concatenating together `l`eft and `r`ight nodes.
    Branch {
        /// The length of this node
        len: usize
      , /// The weight of a node is the summed weight of its left subtree
        weight: usize
      , /// The left branch node
        left: Option<Box<Node<T>>>
      , /// The right branch node
        right: Option<Box<Node<T>>>
    }
}


impl<T> Node<T> {
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
    fn leaf(data: Vec<T>) -> Self {
        Leaf(data)
    }


    /// Returns the height in the tree of a node
    #[inline]
    fn height(&self) -> usize {
        use std::cmp::max;

        match self {
            &Node::Leaf(_) => 1
          , &Node::Branch { ref left, ref right, ..} =>
                max( left.as_ref().map(Box::as_ref).map_or(0, Node::height)
                   , right.as_ref().map(Box::as_ref).map_or(0, Node::height)
                   ) + 1
            }
    }


    /// Returns the length of a node
    //  TODO: do we want to cache this?
    fn len(&self) -> usize {
        match self { &Node::Leaf(ref v) => v.len()
                   , &Node::Branch { ref left, ref right, .. } =>
                        left.as_ref().map(Box::as_ref).map_or(0, Node::len) +
                        right.as_ref().map(Box::as_ref).map_or(0, Node::len)
                    }
    }

    /// Returns the weight of a node
    fn weight (&self) -> usize {
        match self { &Node::Leaf(ref v) => v.len()
                   , &Node::Branch { ref left, .. } =>
                        left.as_ref().map(Box::as_ref).map_or(0, Node::weight)
                    }
    }
}

impl<T> ops::Add for Node<T> {
    type Output = Self;
    /// Concatenate two `Node`s, returning a `Branch` node.
    fn add(self, right: Self) -> Self { Node::branch(self, right) }
}

impl<T> Rope<T> {

    /// Returns a new empty Rope
    ///
    /// # Examples
    /// ```
    /// use an_rope::Rope;
    /// let mut an_rope = Rope::<u8>::new();
    /// assert_eq!(an_rope.len(), 0);
    /// ```
    pub const fn new() -> Rope<T> {
        Rope { root: Node::<T>::none() }
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
    pub fn merge(&self, other: &Rope<T>) -> Rope<T> {
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
    pub fn update(&mut self, other: Rope<T>) {
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
    pub fn split(self, index: usize) -> (Rope<T>, Rope<T>) {
        unimplemented!()
    }
}


impl<T> ops::Index<usize> for Node<T> {
    type Output = T;

    fn index(&self, i: usize) -> &T {
        // let len = self.len();
        // match self { &Node::Leaf(ref vec) => { &vec[i] }
        //             , &Node::Branch { right: Some(box ref r), .. } if len < i =>
        //                 &r[i - len]
        //             , &Node::Branch { left: Some(box ref l), .. } => &l[i]
        //             , &Node::Branch { left: None, right: None, .. } =>
        //                 panic!("Index out of bounds!")
        //             }
        unimplemented!()
    }
}

impl<T> convert::Into<Vec<T>> for Rope<T> {
    fn into(self) -> Vec<T> {
        unimplemented!()
    }

}

impl convert::From<String> for Rope<u8> {
    fn from(string: String) -> Rope<u8> {
        Rope {
            root: if string.len() == 0 { Node::none() }
                  else { Node::Leaf(string.into_bytes()) }
        }
    }
}

//-- comparisons ----------------------------------------------------
impl<T> cmp::PartialEq for Rope<T> {
    fn eq(&self, other: &Rope<T>) -> bool {
        unimplemented!()
    }
}

impl<T> cmp::PartialEq<str> for Rope<T> {
    fn eq(&self, other: &str) -> bool {
        unimplemented!()
    }
}

//-- concatenation --------------------------------------------------
impl<'a, T> ops::Add for &'a Rope<T> {
    type Output = Rope<T>;
    /// Non-destructively concatenate two `Rope`s, returning a new `Rope`.
    ///
    /// # Examples
    /// ```
    /// let rope = Rope::from(String::from("ab"));
    /// assert_eq!( &rope + &Rope::from(String::from("cd"))
    ///           , Rope::from(String::from("abcd")));
    /// ```
    #[inline] fn add(self, other: Self) -> Rope<T> { self.merge(other) }

}

impl<T> ops::Add for Rope<T> {
    type Output = Rope<T>;
    /// Non-destructively concatenate two `Rope`s, returning a new `Rope`.
    ///
    /// # Examples
    /// ```
    /// let rope = Rope::from(String::from("ab"));
    /// assert_eq!( rope + Rope::from(String::from("cd"))
    ///           , Rope::from(String::from("abcd")));
    /// ```
    #[inline] fn add(self, other: Self) -> Rope<T> { self.merge(&other) }
}

impl ops::Add<String> for Rope<u8> {
    type Output = Rope<u8>;
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
    #[inline] fn add(self, other: String) -> Rope<u8> {
         self.merge(&Rope::from(other))
    }
}


impl<'a, 'b> ops::Add<&'b str> for &'a Rope<u8> {
    type Output = Rope<u8>;
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
    #[inline] fn add(self, other: &'b str) -> Rope<u8> {
         self.merge(&Rope::from(other.to_owned()))
     }

}

impl<'a> ops::Add<&'a str> for Rope<u8> {
    type Output = Rope<u8>;
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
    #[inline] fn add(self, other: &'a str) -> Rope<u8> {
         self.merge(&Rope::from(other.to_owned()))
     }

}


impl<T> ops::AddAssign for Rope<T> {

    /// Concatenate two `Rope`s mutably.
    ///
    /// # Examples
    /// ```
    /// let mut rope = Rope::from(String::from("ab"));
    /// rope += Rope::from(String::from("cd"));
    /// assert_eq!(rope, Rope::from(String::from("abcd")));
    /// ```
    #[inline]
    fn add_assign(&mut self, other: Rope<T>) {
        self.update(other)
    }
}

impl ops::AddAssign<String> for Rope<u8> {

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

impl<'a> ops::AddAssign<&'a str> for Rope<u8> {

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

impl<T> ops::Index<usize> for Rope<T> {
    type Output = T;

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
    fn index(&self, i: usize) -> &T {
        &self.root[i]
    }
}

//-- slicing operators ----------------------------------------------
impl<T> ops::Index<ops::Range<usize>> for Rope<T> {
    type Output = [T];

    // Index a substring
    fn index(&self, i: ops::Range<usize>) -> &[T] {
        unimplemented!()
    }
}

impl<T> ops::Index<ops::RangeTo<usize>> for Rope<T> {
    type Output = [T];

    fn index(&self, i: ops::RangeTo<usize>) -> &[T] {
        unimplemented!()
    }
}

impl<T> ops::Index<ops::RangeFrom<usize>> for Rope<T> {
    type Output = [T];

    fn index(&self, i: ops::RangeFrom<usize>) -> &[T] {
        unimplemented!()
    }
}

impl<T> ops::IndexMut<ops::Range<usize>> for Rope<T> {
    fn index_mut(&mut self, i: ops::Range<usize>) -> &mut [T] {
        unimplemented!()
    }
}

impl<T> ops::IndexMut<ops::RangeTo<usize>> for Rope<T> {
    fn index_mut(&mut self, i: ops::RangeTo<usize>) -> &mut [T] {
        unimplemented!()
    }
}

impl<T> ops::IndexMut<ops::RangeFrom<usize>> for Rope<T> {
    fn index_mut(&mut self, i: ops::RangeFrom<usize>) -> &mut [T] {
        unimplemented!()
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
