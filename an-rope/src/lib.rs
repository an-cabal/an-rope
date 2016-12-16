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

use std::cmp;
use std::ops;
use std::convert;

#[derive(Debug)]
pub struct Rope {
    // can we get away with having these be of &str or will they need
    // to be string?
    root: Node
}

impl Rope {

    /// Returns a new empty Rope
    ///
    /// # Examples
    /// ```
    /// use an_rope::Rope;
    /// let mut an_rope = Rope::new();
    /// assert_eq!(an_rope.length(), 0);
    /// ```
    pub const fn new() -> Rope {
        Rope { root: Node::None }
    }

    /// Returns the length of this Rope
    ///
    /// # Examples
    /// ```
    /// use an_rope::Rope;
    /// let mut an_empty_rope = Rope::new();
    /// assert_eq!(an_empty_rope.length(), 0);
    /// ```
    /// ```
    /// use an_rope::Rope;
    /// let mut an_empty_rope = Rope::from(String::from(""));
    /// assert_eq!(an_empty_rope.length(), 0);
    /// ```
    /// ```
    /// use an_rope::Rope;
    /// let mut an_rope = Rope::from(String::from("a string"));
    /// assert_eq!(an_rope.length(), "a string".len());
    /// ```
    pub fn length(&self) -> usize {
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
    pub fn append(&mut self, other: Rope) {
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
    pub fn prepend(&mut self, other: Rope) {
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
}

#[derive(Debug)]
enum Node { /// A leaf node
            // todo: is box<str> the right choice?
            Leaf(Box<str>)
          , /// A branch node
            Branch { l: Box<Node>, r: Box<Node> }
          , /// Nothing
            None
}

impl Node {

    /// Computes the weight of a node
    fn weight(&self) -> usize {
        unimplemented!()
    }
}

impl convert::From<String> for Rope {
    fn from(string: String) -> Rope {
        unimplemented!()
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
impl ops::Add for Rope {
    type Output = Rope;

    // This is the concat operation.
    fn add(self, other: Rope) -> Rope {
        unimplemented!()
    }
}

impl ops::Index<usize> for Rope {
    type Output = char;

    // Index a character
    fn index(&self, i: usize) -> &char {
        unimplemented!()
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
