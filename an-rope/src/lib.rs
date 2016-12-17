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
#![feature(box_patterns)]

use std::cmp;
use std::ops;
use std::convert;


pub mod bintree;
use bintree::Node;

#[derive(Debug)]
pub struct Rope<T> {
    // can we get away with having these be of &str or will they need
    // to be string?
    root: RopeNode<T>
}

type RopeNode<T> = Node<Vec<T>>;

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
        Rope { root: Node::None }
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

    /// Appends a rope to the end of this Rope
    ///
    /// # Examples
    /// ```
    /// use an_rope::Rope;
    /// let mut an_rope = Rope::from(String::from("abcd"));
    /// an_rope.append(Rope::from(String::from("efgh")))
    /// assert_eq!(an_rope, Rope::from(String::from("abcdefgh")));
    /// ```
    pub fn append(&mut self, other: Rope<T>) {
        unimplemented!()
    }

    /// Prepends a rope to the front of this Rope
    ///
    /// # Examples
    /// ```
    /// use an_rope::Rope;
    /// let mut an_rope = Rope::from(String::from("efgh"));
    /// an_rope.preend(Rope::from(String::from("abcd")))
    /// assert_eq!(an_rope, Rope::from(String::from("abcdefgh")));
    /// ```
    pub fn prepend(&mut self, other: Rope<T>) {
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


impl<T> ops::Index<usize> for RopeNode<T> {
    type Output = T;

    fn index(&self, i: usize) -> &T {
        let len = self.len();
        match self { &Node::Leaf(ref vec) => { &vec[i] }
                    , &Node::Branch { box ref r, .. } if len < i => &r[i - len]
                    , &Node::Branch { box ref l, .. } => &l[i]
                    , &Node::None => panic!("Index out of bounds!")
                    }
    }
}

impl<T> RopeNode<T> {

    /// Returns the length of a node
    //  TODO: do we want to cache this?
    fn len(&self) -> usize {
        match *self { Node::Leaf(ref v) => v.len()
                    , Node::Branch { box ref l, box ref r} => l.len() + r.len()
                    , Node::None => 0
                    }
    }

    /// Returns the weight of a node
    fn weight (&self) -> usize {
        match *self { Node::Leaf(ref v) => v.len()
                    , Node::Branch { box ref l, .. } => l.weight()
                    , Node::None => 0
                    }
    }


}


impl convert::From<String> for Rope<u8> {
    fn from(string: String) -> Rope<u8> {
        Rope {
            root: if string.len() == 0 { Node::None }
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
impl ops::AddAssign for Rope<u8> {

    /// Concatenate two `Rope`s.
    ///
    /// # Examples
    /// ```
    /// let mut rope = Rope::from(String::from("ab"));
    /// rope += Rope::from(String::from("cd"));
    /// assert_eq!(rope, Rope::from(String::from("abcd")));
    /// ````
    #[inline]
    fn add_assign(&mut self, other: Rope<u8>) {
        self.append(other)
    }
}

impl ops::AddAssign<String> for Rope<u8> {

    /// Concatenate a `String` onto a `Rope`
    ///
    /// # Examples
    /// ```
    /// let mut rope = Rope::from(String::from("ab"));
    /// rope += String::from("cd");
    /// assert_eq!(rope, Rope::from(String::from("abcd")));
    /// ````
    #[inline]
    fn add_assign(&mut self, string: String) {
        self.append(Rope::from(string))
    }
}

impl<'a> ops::AddAssign<&'a str> for Rope<u8> {

    /// Concatenate an `&str` onto a `Rope`
    ///
    /// # Examples
    /// ```
    /// let mut rope = Rope::from(String::from("ab"));
    /// rope += String::from("cd");
    /// assert_eq!(rope, Rope::from(String::from("abcd")));
    /// ````
    #[inline]
    fn add_assign(&mut self, string: &'a str) {
        self.append(Rope::from(string.to_owned()))
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
