//! An rope.
//!
//! A rope is an efficient data structure for large mutable strings. It's
//! essentially a binary tree whose leaves are strings.
//!
//! For more information, see the following resources:
//! + http://scienceblogs.com/goodmath/2009/01/26/ropes-twining-together-strings/
//! + https://www.ibm.com/developerworks/library/j-ropes/

use std::ops;

#[derive(Debug)]
pub struct Rope{
    // can we get away with having these be of &str or will they need
    // to be string?
    root: Node
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

// slicing operators -----------------------------------------
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
