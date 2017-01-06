//! Rope slices
//!
//! While we could just have the slicing methods on `Rope`s return `&str`s,
//! this would have some serious drawbacks: to slice a rope as `&str`, we'd have
//! to collect all the `Rope`'s characters into a `String` and then slice that
//! `String`. Creating an intermediate `String` would be slow, would cause
//! unnecessary allocations, and, in the case of `Rope::slice_mut()`, would
//! return a mutable slice of a _new `String`_ –- mutating the slice would _not_
//! mutate the underlying `Rope`.
// TODO: implement Borrow<RopeSlice> for Rope?

use std::fmt;
use std::cmp;
use std::convert;

#[cfg(feature = "unstable")]
use collections::range::RangeArgument;
#[cfg(not(feature = "unstable"))]
use std::ops::Range;

use super::Rope;
use super::internals::Node;


/// An immutable borrowed slice of a `Rope`.
///
/// A `RopeSlice` represents an immutable borrowed slice of some or all the
/// characters in a `Rope`.
pub struct RopeSlice<'a> { node: &'a Node
                         , offset: usize
                         , len: usize
                         }

impl<'a> fmt::Display for RopeSlice<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: don't create an intermediate string?
        write!(f, "{}", self.chars().collect::<String>())
    }
}

impl<'a> fmt::Debug for RopeSlice<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: don't create an intermediate string?
        write!( f, "RopeSlice {{ offset: {}, len {} }} [{:?}]"
              , self.offset, self.len, self.chars().collect::<String>())
    }
}

/// An mutable borrowed slice of a `Rope`.
///
/// A `RopeSliceMut` represents a mutable borrowed slice of some or all the
/// characters in a `Rope`.
#[derive(Debug)]
pub struct RopeSliceMut<'a> { node: &'a mut Node
                            , offset: usize
                            , len: usize
                            }

impl<'a> RopeSliceMut<'a>  {

    // TODO: share duplicate functionality with non-mutable RopeSlice in a less
    //       ugly way. Maybe an added generic?
    //          - eliza, 12/23/16

    // TODO: add mutable iterators

    #[cfg(feature = "unstable")]
    pub fn new<R>(node: &'a mut Node, range: R) -> Self
    where R: RangeArgument<usize> {
        let len = node.len();

        // if the RangeArgument doesn't have a defined start index,
        // the slice begins at the 0th index.
        let start = *range.start().unwrap_or(&0);
        // similarly, if there's no defined end, then the end index
        // is the last index in the Rope.
        let end = *range.end().unwrap_or(&node.len());

        let slice_len = end - start;

        // find the lowest node that contains both the slice start index and
        // the end index
        let (node, offset) = if start == 0 && end == len {
            // if the slice contains the entire rope, then the spanning node
            // is the root node
            (node, 0)
        } else {
            node.spanning_mut(start, slice_len)
        };

        RopeSliceMut { node: node
                     , offset: offset
                     , len: slice_len }
    }
    #[cfg(not(feature = "unstable"))]
    pub fn new(node: &'a mut Node, range: Range<usize>) -> Self {
        let len = node.len();
        let slice_len = range.end - range.start;

        // find the lowest node that contains both the slice start index and
        // the end index
        let (node, offset) = if range.start == 0 && range.end == len {
            // if the slice contains the entire rope, then the spanning node
            // is the root node
            (node, 0)
        } else {
            node.spanning_mut(range.start, slice_len)
        };

        RopeSliceMut { node: node
                     , offset: offset
                     , len: slice_len }
    }

    #[cfg(feature = "unstable")]
    #[inline]
    fn slice_char_iter<I, T>(&'a self, i: I) -> impl Iterator<Item=T> + 'a
    where I: Iterator<Item=T>
        , I: 'a
        , T: Copy {
            i.skip(self.offset).take(self.len)
    }

    #[cfg(feature = "unstable")]
    fn slice_strings_iter<I>(&'a self, i: I) -> impl Iterator<Item=&'a str> +'a
    where I: Iterator<Item=&'a str>
        , I: 'a {
        i.scan((self.offset, self.len), |curr, s| {
            match *curr {
                (0, 0) => None
              , (0, ref mut remaining) if *remaining < s.len() => {
                    let r = *remaining;
                    *remaining = 0;
                    Some(&s[..r])
                }
              , (0, ref mut remaining) => {
                    *remaining -= s.len();
                    Some(s)
                }
              , (ref mut offset, _) if *offset > s.len() => {
                    *offset -= s.len();
                    Some("")
                }
              , (ref mut offset, _) => {
                    let c = *offset;
                    *offset -= s.len();
                    Some(&s[c..])
                }
            }
        })
         .skip_while(|&s| s == "")
    }
    #[cfg(not(feature = "unstable"))]
    #[inline]
    fn slice_char_iter<I, T>(&'a self, i: I) -> Box<Iterator<Item=T> + 'a>
    where I: Iterator<Item=T>
        , I: 'a
        , T: Copy {
            Box::new(i.skip(self.offset)
                      .take(self.len))
    }

    #[cfg(not(feature = "unstable"))]
    fn slice_strings_iter<I>(&'a self, i: I) -> Box<Iterator<Item=&'a str> + 'a>
    where I: Iterator<Item=&'a str>
        , I: 'a {
        Box::new(i.scan((self.offset, self.len), |curr, s| {
            match *curr {
                (0, 0) => None
              , (0, ref mut remaining) if *remaining < s.len() => {
                    let r = *remaining;
                    *remaining = 0;
                    Some(&s[..r])
                }
              , (0, ref mut remaining) => {
                    *remaining -= s.len();
                    Some(s)
                }
              , (ref mut offset, _) if *offset > s.len() => {
                    *offset -= s.len();
                    Some("")
                }
              , (ref mut offset, _) => {
                    let c = *offset;
                    *offset -= s.len();
                    Some(&s[c..])
                }
            }
        })
         .skip_while(|&s| s == ""))
    }

    unstable_iters! {
        #[inline]
        pub fn chars(&'a self) -> impl Iterator<Item=char> + 'a  {
            self.slice_char_iter(self.node.chars())
        }
        #[inline]
        pub fn char_indices(&'a self) -> impl Iterator<Item=(usize, char)> + 'a {
            self.chars().enumerate()
        }
        #[inline]
        pub fn bytes(&'a self) -> impl Iterator<Item=u8> + 'a  {
            self.slice_char_iter(self.node.bytes())
        }
        #[inline]
        pub fn split_whitespace(&'a self) -> impl Iterator<Item=&'a str> + 'a  {
            self.slice_strings_iter(self.node.split_whitespace())
        }
    }

    /// Returns true if the bytes in `self` equal the bytes in `other`
    #[inline]
    fn bytes_eq<I>(&self, other: I) -> bool
    where I: Iterator<Item=u8> {
        self.bytes().zip(other).all(|(a, b)| a == b)
    }

    /// Returns the length of `self.`
    ///
    /// This length is in bytes, not chars or graphemes. In other words, it may
    /// not be what a human considers the length of the string.
    #[inline]
    pub fn len(&self) -> usize { self.len }

    #[inline]
    fn take_node(&mut self) -> Node {
        use std::mem::replace;
        replace(self.node, Node::empty())
    }

    /// Insert `rope` into `index` in this mutable `RopeSlice`.
    ///
    /// Note that the index to insert into is relative to the beginning of this
    /// _slice_, not to the beginning of the sliced `Rope`.
    ///
    /// Consumes `rope`.
    ///
    /// # Panics
    /// * If `index` is greater than the length of this `RopeSlice`
    ///
    /// # Time Complexity
    /// O(log _n_)
    ///
    /// # Examples
    /// If built with `--features unstable`:
    /// ```
    /// #![feature(collections)]
    /// #![feature(collections_range)]
    /// ##[cfg(feature = "unstable")]
    /// extern crate collections;
    /// extern crate an_rope;
    /// ##[cfg(feature = "unstable")]
    /// # fn main() {
    ///
    /// use collections::range::RangeArgument;
    /// use an_rope::Rope;
    ///
    /// let mut rope = Rope::from("this is a string");
    /// { // we have to create a new block here so that the mutable borrow
    ///   // on `Rope` will end
    ///    let mut slice = rope.slice_mut(8..);
    ///    slice.insert_rope(1, Rope::from("n example"));
    /// }
    /// assert_eq!(&rope, "this is an example string");
    /// # }
    /// ```
    pub fn insert_rope(&mut self, index: usize, rope: Rope) {
        use metric::Grapheme;
        assert!( index <= self.len()
               , "RopeSliceMut::insert_rope: index {} was > length {}"
               , index, self.len());
        if rope.len() > 0 {
            // split the rope at the given index
            let (left, right) = self.take_node()
                                    .split(Grapheme::from(self.offset + index));

            // construct the new root node with `Rope` inserted
            *self.node = (left + rope.root + right).rebalance();
        }
    }

    /// Insert `ch` into `index` in this mutable `RopeSlice`.
    ///
    /// Note that the index to insert into is relative to the beginning of this
    /// _slice_, not to the beginning of the sliced `Rope`.
    ///
    /// Consumes `ch`.
    ///
    /// # Panics
    /// * If `index` is greater than the length of this `RopeSlice`
    ///
    /// # Time Complexity
    /// O(log _n_)
    ///
    /// # Examples
    /// If built with `--features unstable`:
    /// ```
    /// #![feature(collections)]
    /// #![feature(collections_range)]
    /// ##[cfg(feature = "unstable")]
    /// extern crate collections;
    /// extern crate an_rope;
        /// ##[cfg(feature = "unstable")]
    /// # fn main() {
    ///
    /// use collections::range::RangeArgument;
    /// use an_rope::Rope;
    ///
    /// let mut rope = Rope::from("this is a string");
    /// { // we have to create a new block here so that the mutable borrow
    ///   // on `Rope` will end
    ///    let mut slice = rope.slice_mut(8..);
    ///    slice.insert(1, 'n');
    /// }
    /// assert_eq!(&rope, "this is an string");
    /// # }
    /// ```
    pub fn insert(&mut self, index: usize, ch: char) {
        assert!( index <= self.len()
               , "RopeSliceMut::insert: index {} was > length {}"
               , index, self.len());
        // TODO: this is gross...
        let mut s = String::new();
        s.push(ch);
        self.insert_rope(index, Rope::from(s))
    }


    /// Insert `s` into `index` in this mutable `RopeSlice`.
    ///
    /// Note that the index to insert into is relative to the beginning of this
    /// _slice_, not to the beginning of the sliced `Rope`.
    ///
    /// # Panics
    /// * If `index` is greater than the length of this `RopeSlice`
    ///
    /// # Time Complexity
    /// O(log _n_)
    ///
    /// # Examples
    /// If built with `--features unstable`:
    /// ```
    /// #![feature(collections)]
    /// #![feature(collections_range)]
    /// ##[cfg(feature = "unstable")]
    /// extern crate collections;
    /// extern crate an_rope;
    /// ##[cfg(feature = "unstable")]
    /// # fn main() {
    ///
    /// use collections::range::RangeArgument;
    /// use an_rope::Rope;
    ///
    /// let mut rope = Rope::from("this is a string");
    /// { // we have to create a new block here so that the mutable borrow
    ///   // on `Rope` will end
    ///    let mut slice = rope.slice_mut(8..);
    ///    slice.insert_str(1, "n example");
    /// }
    /// assert_eq!(&rope, "this is an example string");
    /// # }
    /// ```
    pub fn insert_str(&mut self, index: usize, s: &str) {
        assert!( index <= self.len()
               , "RopeSliceMut::insert_str: index {} was > length {}"
               , index, self.len());
        self.insert_rope(index, Rope::from(s))
    }

}

impl<'a> RopeSlice<'a> {
    unstable_iters! {
        #[inline]
        pub fn chars(&'a self) -> impl Iterator<Item=char> + 'a  {
            self.slice_char_iter(self.node.chars())
        }
        #[inline]
        pub fn char_indices(&'a self) -> impl Iterator<Item=(usize, char)> + 'a {
            self.chars().enumerate()
        }
        #[inline]
        pub fn bytes(&'a self) -> impl Iterator<Item=u8> + 'a  {
            self.slice_char_iter(self.node.bytes())
        }
        #[inline]
        pub fn split_whitespace(&'a self) -> impl Iterator<Item=&'a str> + 'a  {
            self.slice_strings_iter(self.node.split_whitespace())
        }
    }


    #[cfg(feature = "unstable")]
    pub fn new<R>(node: &'a Node, range: R) -> Self
    where R: RangeArgument<usize> {
        let len = node.len();

        // if the RangeArgument doesn't have a defined start index,
        // the slice begins at the 0th index.
        let start = *range.start().unwrap_or(&0);
        // similarly, if there's no defined end, then the end index
        // is the last index in the Rope.
        let end = *range.end().unwrap_or(&node.len());

        let slice_len = end - start;

        // find the lowest node that contains both the slice start index and
        // the end index
        let (node, offset) = if start == 0 && end == len {
            // if the slice contains the entire rope, then the spanning node
            // is the root node
            (node, 0)
        } else {
            node.spanning(start, slice_len)
        };

        RopeSlice { node: node
                  , offset: offset
                  , len: slice_len }
    }

    #[cfg(not(feature = "unstable"))]
    pub fn new(node: &'a Node, range: Range<usize>) -> Self {
        let len = node.len();
        let slice_len = range.end - range.start;

        // find the lowest node that contains both the slice start index and
        // the end index
        let (node, offset) = if range.start == 0 && range.end == len {
            // if the slice contains the entire rope, then the spanning node
            // is the root node
            (node, 0)
        } else {
            node.spanning(range.start, slice_len)
        };

        RopeSlice { node: node
                  , offset: offset
                  , len: slice_len }
    }


    #[cfg(feature = "unstable")]
    #[inline]
    fn slice_char_iter<I, T>(&'a self, i: I) -> impl Iterator<Item=T> + 'a
    where I: Iterator<Item=T>
        , I: 'a
        , T: Copy {
            i.skip(self.offset).take(self.len)
    }

    #[cfg(feature = "unstable")]
    fn slice_strings_iter<I>(&'a self, i: I) -> impl Iterator<Item=&'a str> + 'a
    where I: Iterator<Item=&'a str>
        , I: 'a {
        i.scan((self.offset, self.len), |curr, s| {
            match *curr {
                (0, 0) => None
              , (0, ref mut remaining) if *remaining < s.len() => {
                    let r = *remaining;
                    *remaining = 0;
                    Some(&s[..r])
                }
              , (0, ref mut remaining) => {
                    *remaining -= s.len();
                    Some(s)
                }
              , (ref mut offset, _) if *offset > s.len() => {
                    *offset -= s.len();
                    Some("")
                }
              , (ref mut offset, _) => {
                    let c = *offset;
                    *offset -= s.len();
                    Some(&s[c..])
                }
            }
        })
         .skip_while(|&s| s == "")
    }
    #[cfg(not(feature = "unstable"))]
    #[inline]
    fn slice_char_iter<I, T>(&'a self, i: I) -> Box<Iterator<Item=T> + 'a>
    where I: Iterator<Item=T>
        , I: 'a
        , T: Copy {
            Box::new(i.skip(self.offset)
                      .take(self.len))
    }

    #[cfg(not(feature = "unstable"))]
    fn slice_strings_iter<I>(&'a self, i: I) -> Box<Iterator<Item=&'a str> + 'a>
    where I: Iterator<Item=&'a str>
        , I: 'a {
        Box::new(i.scan((self.offset, self.len), |curr, s| {
            match *curr {
                (0, 0) => None
              , (0, ref mut remaining) if *remaining < s.len() => {
                    let r = *remaining;
                    *remaining = 0;
                    Some(&s[..r])
                }
              , (0, ref mut remaining) => {
                    *remaining -= s.len();
                    Some(s)
                }
              , (ref mut offset, _) if *offset > s.len() => {
                    *offset -= s.len();
                    Some("")
                }
              , (ref mut offset, _) => {
                    let c = *offset;
                    *offset -= s.len();
                    Some(&s[c..])
                }
            }
        })
         .skip_while(|&s| s == ""))
    }

    /// Returns true if the bytes in `self` equal the bytes in `other`
    #[inline]
    fn bytes_eq<I>(&self, other: I) -> bool
    where I: Iterator<Item=u8> {
        self.bytes().zip(other).all(|(a, b)| a == b)
    }


    #[inline]
    pub fn len(&self) -> usize { self.len }
}

//-- comparisons ----------------------------------------------------
impl<'a> cmp::Eq for RopeSlice<'a> {}
impl<'a> cmp::PartialEq for RopeSlice<'a> {
    /// A rope equals another rope if all the bytes in both are equal.
    #[inline]
    fn eq(&self, other: &RopeSlice<'a>) -> bool {
        if self.len() == other.len() {
            self.bytes_eq(other.bytes())
        } else {
            false
        }
    }
}

impl<'a> cmp::PartialEq<str> for RopeSlice<'a> {
    /// A rope equals another rope if all the bytes in both are equal.
    #[inline]
    fn eq(&self, other: &str) -> bool {
        if self.len() == other.len() {
            self.bytes_eq(other.bytes())
        } else {
            false
        }
    }
}


impl<'a> cmp::PartialEq<&'a str> for RopeSlice<'a>  {
    /// A rope equals another rope if all the bytes in both are equal.
    #[inline]
    fn eq(&self, other: &&'a str) -> bool {
        if self.len() == other.len() {
            self.bytes_eq((*other).bytes())
        } else {
            false
        }
    }
}

impl<'a> cmp::Eq for RopeSliceMut<'a> {}
impl<'a> cmp::PartialEq for RopeSliceMut<'a> {
    /// A rope equals another rope if all the bytes in both are equal.
    #[inline]
    fn eq(&self, other: &RopeSliceMut<'a>) -> bool {
        if self.len() == other.len() {
            self.bytes_eq(other.bytes())
        } else {
            false
        }
    }
}

impl<'a, 'b> cmp::PartialEq<RopeSlice<'b>> for RopeSliceMut<'a> {
    /// A rope equals another rope if all the bytes in both are equal.

    #[inline]
    fn eq(&self, other: &RopeSlice<'b>) -> bool {
        if self.len() == other.len() {
            self.bytes_eq(other.bytes())
        } else {
            false
        }
    }
}

impl<'a> cmp::PartialEq<str> for RopeSliceMut<'a> {
    /// A rope equals another rope if all the bytes in both are equal.

    #[inline]
    fn eq(&self, other: &str) -> bool {
        if self.len() == other.len() {
            self.bytes_eq(other.bytes())
        } else {
            false
        }
    }
}

impl<'a> convert::Into<Rope> for RopeSlice<'a> {
    /// Converts this `RopeSlice` into a new `Rope`
    fn into(self) -> Rope {
        Rope::from(self.chars().collect::<String>())
    }
}

impl<'a> convert::Into<String> for RopeSlice<'a> {
    /// Converts this `RopeSlice` into a new `String`
    fn into(self) -> String {
        self.chars().collect::<String>()
    }
}

impl<'a> convert::Into<Rope> for RopeSliceMut<'a> {
    /// Converts this `RopeSliceMut` into a new `Rope`
    fn into(self) -> Rope {
        Rope::from(self.chars().collect::<String>())
    }
}

impl<'a> convert::Into<String> for RopeSliceMut<'a> {
    /// Converts this `RopeSliceMut` into a new `String`
    fn into(self) -> String {
        self.chars().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::Rope;

    #[test]
    fn char_indices() {
        let string = "aaaaabbbbbbccccccccccccdefgdefgaabababab";
        let rope = Rope::from(string);
        let rope_slice = rope.slice(4..8);
        let string_slice = &string[4..8];
        let indices = rope_slice.char_indices().zip(string_slice.char_indices());
        for ((ridx, rch), (sidx, sch)) in indices {
            assert_eq!(rch, sch);
            assert_eq!(ridx, sidx);
        }
    }

    #[test]
    fn to() {
        let string = "aaaaabbbbbbccccccccccccdefgdefgaabababab";
        let rope = Rope::from(string);
        let rope_slice = rope.slice(1..10);
        let string_slice = &string[1..10];
        assert_eq!(&rope_slice, string_slice)
    }

    // #[test]
    // fn between() {
    //     let string = "aaaaabbbbbbccccccccccccdefgdefgaabababab";
    //     let rope = Rope::from(string);
    //     let rope_slice = rope.slice(1...10usize);
    //     let string_slice = &string[1...10];
    //     assert_eq!(&rope_slice, string_slice)
    // }
    #[cfg(feature = "unstable")]
    #[test]
    fn until() {
        let string = "aaaaabbbbbbccccccccccdefgdefgaabababab";
        let rope = Rope::from(string);
        let rope_slice = rope.slice(..10);
        let string_slice = &string[..10];
        assert_eq!(&rope_slice, string_slice)
    }

    #[cfg(feature = "unstable")]
    #[test]
    fn from() {
        let mut string = "aaaaabbbbbbccccccccccccdefgdefgaabababab";
        let mut rope = Rope::from(string);
        let rope_slice = rope.slice(5..);
        let string_slice = &string[5..];
        assert_eq!(&rope_slice, string_slice)
    }
    #[cfg(feature = "unstable")]
    #[test]
    fn full() {
        let string = "aaaaabbbbbbccccccccccccdefgdefgaabababab";
        let rope = Rope::from(string);
        let rope_slice = rope.slice(..);
        let string_slice = &string[..];
        assert_eq!(&rope_slice, string_slice)
    }

    #[test]
    fn mut_char_indices() {
        let mut string =
            String::from("aaaaabbbbbbccccccccccccdefgdefgaabababab");
        let mut rope = Rope::from(string.clone());
        let rope_slice = rope.slice_mut(4..8);
        let string_slice = &mut string[4..8];
        let indices = rope_slice.char_indices().zip(string_slice.char_indices());
        for ((ridx, rch), (sidx, sch)) in indices {
            assert_eq!(rch, sch);
            assert_eq!(ridx, sidx);
        }
    }

    #[test]
    fn mut_to() {
        let mut string =
            String::from("aaaaabbbbbbccccccccccccdefgdefgaabababab");
        let mut rope = Rope::from(string.clone());
        let rope_slice = rope.slice_mut(1..10);
        let string_slice = &mut string[1..10];
        assert_eq!(&rope_slice, string_slice)
    }
    #[cfg(feature = "unstable")]
    #[test]
    fn mut_until() {
        let mut string =
            String::from("aaaaabbbbbbccccccccccccdefgdefgaabababab");
        let mut rope = Rope::from(string.clone());
        let rope_slice = rope.slice_mut(..10);
        let string_slice = &mut string[..10];
        assert_eq!(&rope_slice, string_slice)
    }
    #[cfg(feature = "unstable")]
    #[test]
    fn mut_from() {
        let mut string =
            String::from("aaaaabbbbbbccccccccccccdefgdefgaabababab");
        let mut rope = Rope::from(string.clone());
        let rope_slice = rope.slice_mut(5..);
        let string_slice = &mut string[5..];
        assert_eq!(&rope_slice, string_slice)
    }

    #[cfg(feature = "unstable")]
    #[test]
    fn mut_full() {
        let mut string =
            String::from("aaaaabbbbbbccccccccccccdefgdefgaabababab");
        let mut rope = Rope::from(string.clone());
        let rope_slice = rope.slice_mut(..);
        let string_slice = &mut string[..];
        assert_eq!(&rope_slice, string_slice)
    }
    #[cfg(feature = "unstable")]
    #[test]
    fn mut_insert_rope() {
        let mut rope = Rope::from("this is a string");
         {
             let slice = rope.slice_mut(8..);
             assert_eq!(&slice, "a string");
         }
         {
             let mut slice = rope.slice_mut(8..);
             slice.insert_rope(1, Rope::from("n example"));
         }
        assert_eq!(&rope, "this is an example string");
    }
    #[cfg(feature = "unstable")]
    #[test]
    fn mut_insert_str() {
        let mut rope = Rope::from("this is a string");
         {
             let slice = rope.slice_mut(8..);
             assert_eq!(&slice, "a string");
         }
         {
             let mut slice = rope.slice_mut(8..);
             slice.insert_str(1, "n example");
         }
        assert_eq!(&rope, "this is an example string");
    }
    #[cfg(feature = "unstable")]
    #[test]
    fn mut_insert_char() {
        let mut rope = Rope::from("this is a string");
         {
             let slice = rope.slice_mut(8..);
             assert_eq!(&slice, "a string");
         }
         {
             let mut slice = rope.slice_mut(8..);
             slice.insert(1, 'n');
         }
        assert_eq!(&rope, "this is an string");
    }
}
