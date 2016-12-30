//! # An rope.
//!
//! A rope is an efficient data structure for large mutable strings. It's
//! essentially a binary tree whose leaves are strings.
//!
//! For more information, see the following resources:
//!
//! + http://scienceblogs.com/goodmath/2009/01/26/ropes-twining-together-strings/
//! + https://www.ibm.com/developerworks/library/j-ropes/
//! + http://citeseer.ist.psu.edu/viewdoc/download?doi=10.1.1.14.9450&rep=rep1&type=pdf

#![feature(const_fn)]
#![feature(box_syntax, box_patterns)]
#![feature(conservative_impl_trait)]
#![cfg_attr(test, feature(test))]
#![cfg_attr(test, feature(insert_str))]
#![feature(collections)]
#![feature(collections_range)]
#![feature(inclusive_range_syntax)]

extern crate collections;

use collections::range::RangeArgument;
use collections::borrow::Borrow;

use std::cmp;
use std::ops;
use std::convert;
use std::fmt;
use std::string;
use std::iter;

#[cfg(feature = "with_tendrils")] extern crate tendril;
#[cfg(feature = "with_tendrils")] use tendril::StrTendril;

#[cfg(test)] mod tests;
#[cfg(test)] extern crate test;

mod slice;


use self::internals::Node;
pub use self::slice::{RopeSlice, RopeSliceMut};

/// A Rope
///
/// This Rope implementation aims to eventually function as a superset of
/// [`String`](https://doc.rust-lang.org/1.3.0/std/string/struct.String.html),
/// providing the same API plus additional methods. Therefore, code which uses
/// `String` can easily be ported to use `Rope`.
///
/// `Rope` provides two APIs for editing a `Rope`: a destructive,
/// append-in-place API whose methods match those of `String`, and a
/// non-destructive, persistant API. The persistant API's methods have names
/// prefixed with `with_`, such as `with_push()` and `with_append()`.
///
#[derive(Clone)]
pub struct Rope {
    // can we get away with having these be of &str or will they need
    // to be string?
    root: Node
}

impl fmt::Debug for Rope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rope[\"{}\"] {:?}", self.root, self.root)
    }
}


macro_rules! str_iters {
    ( $($(#[$attr:meta])* impl $name: ident<$ty: ty> for Node {})+ ) => { $(
        $(#[$attr])*
        pub fn $name<'a>(&'a self) -> impl Iterator<Item=$ty> + 'a {
            self.strings().flat_map(str::$name)
        }
    )+ };

    ( $($(#[$attr:meta])* impl $name: ident<$ty: ty> for Rope {})+ )=> { $(
        $(#[$attr])*
        pub fn $name<'a>(&'a self) -> impl Iterator<Item=$ty> + 'a {
            self.root.$name()
        }
    )+ }

}

mod internals;



impl Rope {

    /// Converts a vector of bytes to a `Rope`.
    ///
    /// If you are sure that the byte slice is valid UTF-8, and you don't want
    /// to incur the overhead of the validity check, there is an unsafe version
    /// of this function, `from_utf8_unchecked(),`` which has the same behavior
    /// but skips the check.
    ///
    /// This method will take care to not copy the vector, for efficiency's
    /// sake.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the slice is not UTF-8 with a description as to why the
    /// provided bytes are not UTF-8. The vector you moved in is also included.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use an_rope::Rope;
    ///
    /// // some bytes, in a vector
    /// let sparkle_heart = vec![240, 159, 146, 150];
    ///
    /// // We know these bytes are valid, so we'll use `unwrap()`.
    /// let sparkle_heart = Rope::from_utf8(sparkle_heart).unwrap();
    ///
    /// assert_eq!(&sparkle_heart, "💖");
    /// ```
    ///
    /// Incorrect bytes:
    ///
    /// ```
    /// use an_rope::Rope;
    ///
    /// // some invalid bytes, in a vector
    /// let sparkle_heart = vec![0, 159, 146, 150];
    ///
    /// assert!(Rope::from_utf8(sparkle_heart).is_err());
    /// ```
    ///
    #[inline]
    pub fn from_utf8(vec: Vec<u8>) -> Result<Rope, string::FromUtf8Error> {
        String::from_utf8(vec).map(Rope::from)
    }

    /// Decode a UTF-16 encoded vector `v` into a `Rope`,
    /// returning `Err` if `v` contains any invalid data.
    #[inline]
    pub fn from_utf16(v: &[u16]) -> Result<Rope, string::FromUtf16Error> {
        String::from_utf16(v).map(Rope::from)
    }

    /// Converts a vector of bytes to a `Rope` without checking that the
    /// vector contains valid UTF-8.
    ///
    /// See the safe version, [`from_utf8()`], for more details.
    ///
    /// [`from_utf8()`]: struct.Rope.html#method.from_utf8
    ///
    /// # Safety
    ///
    /// This function is unsafe because it does not check that the bytes passed
    /// to it are valid UTF-8. If this constraint is violated, it may cause
    /// memory unsafety issues with future users of the `Rope`, as the rest of
    /// the standard library assumes that `Rope`s are valid UTF-8.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use an_rope::Rope;
    ///
    /// // some bytes, in a vector
    /// let sparkle_heart = vec![240, 159, 146, 150];
    ///
    /// let sparkle_heart = unsafe {
    ///     Rope::from_utf8_unchecked(sparkle_heart)
    /// };
    ///
    /// assert_eq!(&sparkle_heart, "💖");
    /// ```
    #[inline]
    pub unsafe fn from_utf8_unchecked(bytes: Vec<u8>) -> Rope {
        Rope::from(String::from_utf8_unchecked(bytes))
    }

    /// Take this `Rope`s root node, leaving an empty node in its place
    #[inline]
    fn take_root(&mut self) -> Node {
        use std::mem;
        mem::replace(&mut self.root, Node::empty())
    }

    /// Returns a new empty Rope
    ///
    /// # Examples
    /// ```
    /// use an_rope::Rope;
    /// let mut an_rope = Rope::new();
    /// assert_eq!(an_rope.len(), 0);
    /// ```
    pub fn new() -> Rope {
        Rope { root: Node::empty() }
    }

    /// Returns the length of this Rope
    ///
    /// # Examples
    ///
    /// An empty `Rope` should have length 0.
    ///
    /// ```
    /// use an_rope::Rope;
    /// let mut an_empty_rope = Rope::new();
    /// assert_eq!(an_empty_rope.len(), 0);
    /// ```
    ///
    /// ```
    /// use an_rope::Rope;
    /// let mut an_empty_rope = Rope::from(String::from(""));
    /// assert_eq!(an_empty_rope.len(), 0);
    /// ```
    ///
    /// A `Rope` with text should have length equal to the number of
    /// characters in the `Rope`.
    ///
    /// ```
    /// use an_rope::Rope;
    /// let mut an_rope = Rope::from(String::from("a string"));
    /// assert_eq!(an_rope.len(), "a string".len());
    /// ```
    pub fn len(&self) -> usize { self.root.len() }

    /// Insert `char` into `index` in this `Rope`,
    ///
    /// # Panics
    /// * If `index` is greater than the length of this `Rope`
    ///
    /// # Time Complexity
    /// O(log _n_)
    ///
    /// # Examples
    ///
    /// Inserting at index 0 prepends `char` to this `Rope`:
    ///
    /// ```
    /// use an_rope::Rope;
    /// let mut an_rope = Rope::from("bcd");
    /// an_rope.insert(0, 'a');
    /// assert_eq!(an_rope, Rope::from("abcd"));
    /// ```
    ///
    /// Inserting at index `len` prepends `char` to this `Rope`:
    ///
    /// ```
    /// use an_rope::Rope;
    /// let mut an_rope = Rope::from("abc");
    /// an_rope.insert(3, 'd');
    /// assert_eq!(an_rope, Rope::from("abcd"));
    /// ```
    ///
    /// Inserting at an index in the middle inserts `char` at that index:
    ///
    /// ```
    /// use an_rope::Rope;
    /// let mut an_rope = Rope::from("acd");
    /// an_rope.insert(1, 'b');
    /// assert_eq!(an_rope, Rope::from("abcd"));
    /// ```
    #[inline]
    pub fn insert(&mut self, index: usize, ch: char) {
        assert!( index <= self.len()
               , "Rope::insert: index {} was > length {}"
               , index, self.len());
        // TODO: this is gross...
        let mut s = String::new();
        s.push(ch);
        self.insert_rope(index, Rope::from(s))
    }

    /// Delete the range `range` from this `Rope`,
    ///
    /// # Panics
    /// * If the start or end of `range` are indices outside of the `Rope`
    /// * If the end index of `range` is greater than the start index
    ///
    /// # Time Complexity
    /// O(log _n_)
    ///
    /// # Examples
    ///
    /// Deleting "not" from this `Rope`:
    ///
    /// ```
    /// use an_rope::Rope;
    /// let mut an_rope = Rope::from("this is not fine".to_string());
    /// an_rope.delete((8..12));
    /// assert_eq!(&an_rope, "this is fine");
    /// ```
    #[inline]
    pub fn delete<R>(&mut self, range: R)
    where R: RangeArgument<usize> {
        let start = *range.start().unwrap_or(&0);
        let end = *range.end().unwrap_or(&self.len());
        let (l, r) = self.take_root().split(start);
        let (_, r) = r.split(end - start);
        self.root = Node::new_branch(l, r);
    }

    /// Insert `ch` into `index` in this `Rope`, returning a new `Rope`.
    ///
    ///
    /// # Returns
    /// * A new `Rope` with `ch` inserted at `index`
    ///
    /// # Time Complexity
    /// O(log _n_)
    ///
    /// # Panics
    /// * If `index` is greater than the length of this `Rope`
    ///
    /// # Examples
    ///
    /// Inserting at index 0 prepends `rope` to this `Rope`:
    ///
    /// ```
    /// use an_rope::Rope;
    /// let an_rope = Rope::from("bcd");
    /// let new_rope = an_rope.with_insert(0, 'a');
    /// assert_eq!(new_rope, Rope::from("abcd"));
    /// assert_eq!(an_rope, Rope::from("bcd"));
    /// ```
    ///
    /// Inserting at index `len` prepends `char` to this `Rope`:
    ///
    /// ```
    /// use an_rope::Rope;
    /// let an_rope = Rope::from("abc");
    /// let new_rope = an_rope.with_insert(an_rope.len(), 'd');
    /// assert_eq!(new_rope, Rope::from("abcd"));
    /// assert_eq!(an_rope, Rope::from("abc"));
    /// ```
    ///
    /// Inserting at an index in the middle inserts `char` at that index:
    ///
    /// ```
    /// use an_rope::Rope;
    /// let an_rope = Rope::from("acd");
    /// let new_rope = an_rope.with_insert(1, 'b');
    /// assert_eq!(new_rope, Rope::from("abcd"));
    /// assert_eq!(an_rope, Rope::from("acd"));
    /// ```
    #[inline]
    pub fn with_insert(&self, index: usize, ch: char) -> Rope {
        assert!( index <= self.len()
               , "Rope::with_insert: index {} was > length {}"
               , index, self.len());
       // TODO: this is gross...
       let mut s = String::new();
       s.push(ch);
       self.with_insert_rope(index, Rope::from(s))
    }

    /// Insert `rope` into `index` in this `Rope`,
    ///
    /// Consumes `rope`.
    ///
    /// # Panics
    /// * If `index` is greater than the length of this `Rope`
    ///
    /// # Time Complexity
    /// O(log _n_)
    ///
    /// # Examples
    ///
    /// Inserting at index 0 prepends `rope` to this `Rope`:
    ///
    /// ```
    /// use an_rope::Rope;
    /// let mut an_rope = Rope::from("cd");
    /// an_rope.insert_rope(0, Rope::from("ab"));
    /// assert_eq!(an_rope, Rope::from("abcd"));
    /// ```
    ///
    /// Inserting at index `len` prepends `rope` to this `Rope`:
    ///
    /// ```
    /// use an_rope::Rope;
    /// let mut an_rope = Rope::from("ab");
    /// an_rope.insert_rope(2, Rope::from("cd"));
    /// assert_eq!(an_rope, Rope::from("abcd"));
    /// ```
    ///
    /// Inserting at an index in the middle inserts `rope` at that index:
    ///
    /// ```
    /// use an_rope::Rope;
    /// let mut an_rope = Rope::from("ad");
    /// an_rope.insert_rope(1, Rope::from("bc"));
    /// assert_eq!(an_rope, Rope::from("abcd"));
    /// ```
    pub fn insert_rope(&mut self, index: usize, rope: Rope) {
        if rope.len() > 0 {
            let len = self.len();
            if index == 0 {
                // if the rope is being inserted at index 0, just prepend it
                self.prepend(rope)
            } else if index == len {
                // if the rope is being inserted at index len, append it
                self.append(rope)
            } else {
                // split the rope at the given index
                let (left, right) = self.take_root().split(index);

                // construct the new root node with `Rope` inserted
                self.root = left + rope.root + right;
            }
            // rebalance the new rope
            self.rebalance();
        }
    }

    /// Insert `rope` into `index` in this `Rope`, returning a new `Rope`.
    ///
    /// Consumes `rope`.
    ///
    ///
    /// # Returns
    /// * A new `Rope` with `rope` inserted at `index`
    ///
    /// # Time Complexity
    /// O(log _n_)
    ///
    /// # Panics
    /// * If `index` is greater than the length of this `Rope`
    ///
    /// # Examples
    ///
    /// Inserting at index 0 prepends `rope` to this `Rope`:
    ///
    /// ```
    /// use an_rope::Rope;
    /// let an_rope = Rope::from("cd");
    /// let new_rope = an_rope.with_insert_rope(0, Rope::from("ab"));
    /// assert_eq!(new_rope, Rope::from("abcd"));
    /// assert_eq!(an_rope, Rope::from("cd"));
    /// ```
    ///
    /// Inserting at index `len` prepends `rope` to this `Rope`:
    ///
    /// ```
    /// use an_rope::Rope;
    /// let an_rope = Rope::from("ab");
    /// let new_rope = an_rope.with_insert_rope(an_rope.len(), Rope::from("cd"));
    /// assert_eq!(new_rope, Rope::from("abcd"));
    /// assert_eq!(an_rope, Rope::from("ab"));
    /// ```
    ///
    /// Inserting at an index in the middle inserts `rope` at that index:
    ///
    /// ```
    /// use an_rope::Rope;
    /// let an_rope = Rope::from("ad");
    /// let new_rope = an_rope.with_insert_rope(1, Rope::from("bc"));
    /// assert_eq!(new_rope, Rope::from("abcd"));
    /// assert_eq!(an_rope, Rope::from("ad"))
    /// ```
    pub fn with_insert_rope(&self, index: usize, rope: Rope) -> Rope {
        assert!( index <= self.len()
               , "Rope::with_:insert_rope: index {} was > length {}"
               , index, self.len());
        let mut new_rope = self.clone();
        new_rope.insert_rope(index, rope);
        new_rope
    }

    /// Insert string slice `s` into `index` in this `Rope`,
    ///
    /// # Panics
    /// * If `index` is greater than the length of this `Rope`
    ///
    /// # Time Complexity
    /// O(log _n_)
    ///
    /// # Examples
    ///
    /// Inserting at index 0 prepends `s` to this `Rope`:
    ///
    /// ```
    /// use an_rope::Rope;
    /// let mut an_rope = Rope::from("cd");
    /// an_rope.insert_str(0, "ab");
    /// assert_eq!(an_rope, Rope::from("abcd"));
    /// ```
    ///
    /// Inserting at index `len` prepends `s` to this `Rope`:
    ///
    /// ```
    /// use an_rope::Rope;
    /// let mut an_rope = Rope::from("ab");
    /// an_rope.insert_str(2, "cd");
    /// assert_eq!(an_rope, Rope::from("abcd"));
    /// ```
    ///
    /// Inserting at an index in the middle inserts `s` at that index:
    ///
    /// ```
    /// use an_rope::Rope;
    /// let mut an_rope = Rope::from("ad");
    /// an_rope.insert_str(1, "bc");
    /// assert_eq!(an_rope, Rope::from("abcd"));
    /// ```
    #[inline]
    pub fn insert_str(&mut self, index: usize, s: &str) {
        assert!( index <= self.len()
               , "Rope::insert_str: index {} was > length {}"
               , index, self.len());
        self.insert_rope(index, s.into())
    }

    /// Insert `s` into `index` in this `Rope`, returning a new `Rope`.
    ///
    /// # Returns
    /// * A new `Rope` with `s` inserted at `index`
    ///
    /// # Panics
    /// *  If `index` is greater than the length of this `Rope`
    ///
    /// # Time Complexity
    /// O(log _n_)
    ///
    /// # Examples
    ///
    /// Inserting at index 0 prepends `s` to this `Rope`:
    ///
    /// ```
    /// use an_rope::Rope;
    /// let an_rope = Rope::from("cd");
    /// let an_rope = an_rope.with_insert_str(0, "ab");
    /// assert_eq!(an_rope, Rope::from("abcd"));
    /// ```
    ///
    /// Inserting at index `len` prepends `s` to this `Rope`:
    ///
    /// ```
    /// use an_rope::Rope;
    /// let an_rope = Rope::from("ab");
    /// let new_rope = an_rope.with_insert_str(an_rope.len(), "cd");
    /// assert_eq!(new_rope, Rope::from("abcd"));
    /// assert_eq!(an_rope, Rope::from("ab"));
    /// ```
    ///
    /// Inserting at an index in the middle inserts `s` at that index:
    ///
    /// ```
    /// use an_rope::Rope;
    /// let an_rope = Rope::from("ad");
    /// let new_rope = an_rope.with_insert_str(1, "bc");
    /// assert_eq!(an_rope, Rope::from("ad"));
    /// assert_eq!(new_rope, Rope::from("abcd"));
    /// ```
    #[inline]
    pub fn with_insert_str(&self, index: usize, s: &str) -> Rope {
        assert!( index <= self.len()
               , "Rope::with_insert_str: index {} was > length {}"
               , index, self.len());
        self.with_insert_rope(index, s.into())
    }

    /// Appends a `Rope` to the end of this `Rope`, updating it in place.
    ///
    /// Note that this is equivalent to using the `+=` operator.
    ///
    /// # Examples
    ///
    /// ```
    /// use an_rope::Rope;
    /// let mut an_rope = Rope::from(String::from("abcd"));
    /// an_rope.append(Rope::from(String::from("efgh")));
    /// assert_eq!(an_rope, Rope::from(String::from("abcdefgh")) );
    /// ```

    pub fn append(&mut self, other: Rope) {
        if other.len() > 0 {
            self.root += other.root;
            self.rebalance();
        }
    }

    /// Appends a `Rope` to the end of this `Rope`, returning a new `Rope`
    ///
    /// Consumes `other`.
    ///
    /// Note that this is equivalent to using the `+` operator.
    ///
    /// # Examples
    ///
    /// ```
    /// use an_rope::Rope;
    /// let an_rope = Rope::from("abcd");
    /// let another_rope = an_rope.with_append(Rope::from("efgh"));
    /// assert_eq!(&another_rope, "abcdefgh");
    /// assert_eq!(&an_rope, "abcd");
    /// ```
    pub fn with_append(&self, other: Rope) -> Rope {
        if other.len() == 0 {
            self.clone()
        } else {
            // let mut rope = Rope {
            //     root: Node::new_branch(self.root.clone(), other.root)
            // };
            // rope.rebalance();
            // rope
            let mut rope = self.clone();
            rope.append(other);
            rope
        }
    }

    /// Prepends a `Rope` to the front of this `Rope`, modifying it in place.
    ///
    /// Consumes `other`.
    ///
    /// # Examples
    /// ```
    /// use an_rope::Rope;
    /// let mut an_rope = Rope::from(String::from("efgh"));
    /// an_rope.prepend(Rope::from(String::from("abcd")));
    /// assert_eq!(&an_rope, "abcdefgh");
    /// ```
    pub fn prepend(&mut self, other: Rope) {
        if other.len() > 0 {
            self.root = other.root + self.take_root();
            self.rebalance();
        }
    }

    /// Prepends a `Rope` to the end of this `Rope`, returning a new `Rope`
    ///
    /// Consumes `other`.
    ///
    /// # Examples
    ///
    /// ```
    /// use an_rope::Rope;
    /// let an_rope = Rope::from("efgh");
    /// let another_rope = an_rope.with_prepend(Rope::from("abcd"));
    /// assert_eq!(&an_rope, "efgh");
    /// assert_eq!(&another_rope, "abcdefgh");
    /// ```
    ///
    /// ```
    /// use an_rope::Rope;
    /// let an_rope = Rope::from("");
    /// let another_rope = an_rope.with_prepend(Rope::from("abcd"));
    /// assert_eq!(&an_rope, "");
    /// assert_eq!(&another_rope, "abcd");
    /// ```
    ///
    /// ```
    /// use an_rope::Rope;
    /// let an_rope = Rope::from("abcd");
    /// let another_rope = an_rope.with_prepend(Rope::from(""));
    /// assert_eq!(&an_rope, "abcd");
    /// assert_eq!(&another_rope, &an_rope);
    /// assert_eq!(&another_rope, "abcd");
    /// ```
    pub fn with_prepend(&self, other: Rope) -> Rope {
        if other.len() == 0 {
            self.clone()
        } else {
            // let mut rope = Rope {
            //     root: Node::new_branch(self.root.clone(), other.root)
            // };
            // rope.rebalance();
            // rope
            let mut rope = self.clone();
            rope.prepend(other);
            rope
        }
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
        assert!(index <= self.len());
        let (l, r) = self.root.split(index);
        (Rope { root: l }, Rope { root: r })
    }

    /// Rebalances this entire `Rope`, returning a balanced `Rope`.
    #[inline]
    fn rebalance(&mut self) {
        if self.is_balanced() {
            // the rope is already balanced, do nothing
        } else {
            // rebalance the rope
            self.root = self.take_root().rebalance();
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

    /// Returns an iterator over all the strings in this `Rope`
    #[inline]
    pub fn strings<'a>(&'a self) -> impl Iterator<Item=&'a str> {
        // TODO: since all the iterator methods on `Rope` just call the
        //       methods on `Node`, do we wanna just make `Node` pub
        //       and add a deref conversion from a `Rope` handle to its'
        //       root `Node`?
        //          - eliza, 12/18/2016
        self.root.strings()
    }

    /// Returns a move iterator over all the strings in this `Rope`
    ///
    /// Consumes `self`.
    #[inline]
    pub fn into_strings<'a>(self) -> impl Iterator<Item=String> + 'a {
        self.root.into_strings()
    }

    str_iters! {
        #[doc="Returns an iterator over all the bytes in this `Rope`.\n\
               \nAs a Rope consists of a sequence of bytes, we can iterate \
               through a rope by byte. This method returns such an iterator."]
        #[inline]
        impl bytes<u8> for Rope {}
        #[doc="Returns an iterator over all the characters in this `Rope`.\n\
               \nAs a `Rope` consists of valid UTF-8, we can iterate through a \
               `Rope` by `char`. This method returns such an iterator. \n\
               \nIt's important to remember that `char` represents a Unicode \
               Scalar Value, and may not match your idea of what a \
               'character' is. Iteration over grapheme clusters may be what \
               you actually want."]
        #[inline]
        impl chars<char> for Rope {}
        #[inline]
        impl char_indices<(usize, char)> for Rope {}
        #[inline]
        impl split_whitespace<&'a str> for Rope {}
        #[inline]
        impl lines<&'a str> for Rope {}
    }

    /// Returns an iterator over the grapheme clusters of this `Rope`
    ///
    /// This is the iterator returned by `Node::into_iter`.
    #[inline]
    pub fn graphemes<'a>(&'a self) -> impl Iterator<Item=&'a str> {
        self.root.graphemes()
    }

    /// Returns true if the bytes in `self` equal the bytes in `other`
    #[inline]
    fn bytes_eq<I>(&self, other: I) -> bool
    where I: Iterator<Item=u8> {
        self.bytes().zip(other).all(|(a, b)| a == b)
    }

    /// Returns an immutable slice of this `Rope` between the given indices.
    ///
    /// # Arguments
    /// + `range`: A [`RangeArgument`](https://doc.rust-lang.org/nightly/collections/range/trait.RangeArgument.html)
    /// specifying the range to slice. This can be produced by range syntax
    /// like `..`, `a..`, `..b` or `c..d`.
    ///
    /// # Panics
    /// If the start or end indices of the range to slice exceed the length of
    /// this `Rope`.
    ///
    /// # Examples
    /// ```ignore
    //  this doctest fails to link on my macbook for Secret Reasons.
    //  i'd really like to know why...
    //      - eliza, 12/23/2016
    /// #![feature(collections)]
    /// #![feature(collections_range)]
    ///
    /// extern crate collections;
    /// extern crate an_rope;
    /// # fn main() {
    /// use collections::range::RangeArgument;
    /// use an_rope::Rope;
    ///
    /// let rope = Rope::from("this is an example string");
    /// assert_eq!(&rope.slice(4..6), "is");
    /// # }
    /// ```
    //  TODO: this uses the unstable `collections::Range::RangeArgument` type
    //        to be generic over different types of ranges (inclusive & //        non-inclusive). however, since `RangeArgument` is feature-gated,
    //        this won't work on stable Rust. We could easily add a feature flag
    //        for `RangeArgument`, and provide an alternate implementation of
    //        rope slicing as well, if we wanted to support stable Rust
    //          -- eliza, 12/23/2016
    #[inline]
    pub fn slice<'a, R>(&'a self, range: R) -> RopeSlice<'a>
    where R: RangeArgument<usize> {
        RopeSlice::new(&self.root, range)
    }

    /// Returns an mutable slice of this `Rope` between the given indices.
    ///
    ///
    /// # Arguments
    /// + `range`: A [`RangeArgument`](https://doc.rust-lang.org/nightly/collections/range/trait.RangeArgument.html)
    /// specifying the range to slice. This can be produced by range syntax
    /// like `..`, `a..`, `..b` or `c..d`.
    ///
    ///
    /// # Panics
    /// If the start or end indices of the range to slice exceed the length of
    /// this `Rope`.
    ///
    /// # Examples
    /// ```ignore
    //  this doctest fails to link on my macbook for Secret Reasons.
    //  i'd really like to know why...
    //      - eliza, 12/23/2016
    /// #![feature(collections)]
    /// #![feature(collections_range)]
    ///
    /// extern crate collections;
    /// extern crate an_rope;
    /// # fn main() {
    /// use collections::range::RangeArgument;
    /// use an_rope::Rope;
    ///
    /// let mut rope = Rope::from("this is an example string");
    /// assert_eq!(&mut rope.slice_mut(4..6), "is");
    /// # }
    /// ```
    //  TODO: this uses the unstable `collections::Range::RangeArgument` type
    //        to be generic over different types of ranges (inclusive & //        non-inclusive). however, since `RangeArgument` is feature-gated,
    //        this won't work on stable Rust. We could easily add a feature flag
    //        for `RangeArgument`, and provide an alternate implementation of
    //        rope slicing as well, if we wanted to support stable Rust
    //          -- eliza, 12/23/2016
    #[inline]
    pub fn slice_mut<'a, R>(&'a mut self, range: R) -> RopeSliceMut<'a>
    where R: RangeArgument<usize> {
        RopeSliceMut::new(&mut self.root, range)
    }
}

impl convert::Into<Vec<u8>> for Rope {
    fn into(self) -> Vec<u8> {
        unimplemented!()
    }

}

#[cfg(feature = "with_tendrils")]
impl convert::From<StrTendril> for Rope {
    fn from(tendril: StrTendril) -> Rope {
        Rope { root: Node::new_leaf(tendril) }
    }
}

impl convert::From<String> for Rope {


    #[cfg(feature = "with_tendrils")]
    #[inline]
    fn from(string: String) -> Rope {
        Rope {
            root: if string.len() == 0 { Node::empty() }
                  else { Node::new_leaf(StrTendril::from(string)) }
        }
    }


    #[cfg(not(feature = "with_tendrils"))]
    #[inline]
    fn from(string: String) -> Rope {
        Rope {
            root: if string.len() == 0 { Node::empty() }
                  else { Node::new_leaf(string) }
        }
    }
}


impl<'a> convert::From<&'a str> for Rope {
    #[cfg(feature = "with_tendrils")]
    #[inline]
    fn from(string: &'a str) -> Rope {
         Rope::from(StrTendril::from_slice(string))
     }

    #[cfg(not(feature = "with_tendrils"))]
    #[inline]
    fn from(string: &'a str) -> Rope { Rope::from(String::from(string)) }
}


//-- comparisons ----------------------------------------------------
impl cmp::Eq for Rope {}
impl cmp::PartialEq for Rope {
    /// A rope equals another rope if all the bytes in both are equal.
    ///
    /// # Examples
    /// ```
    /// use an_rope::Rope;
    /// assert!(Rope::from("abcd") == Rope::from("abcd"));
    /// ```
    /// ```
    /// use an_rope::Rope;
    /// assert!(Rope::from("abcd") != Rope::from("ab"));
    /// ```
    /// ```
    /// use an_rope::Rope;
    /// assert!(Rope::from("abcd") != Rope::from("dcab"))
    /// ```
    #[inline]
    fn eq(&self, other: &Rope) -> bool {
        if self.len() == other.len() {
            self.bytes_eq(other.bytes())
        } else {
            false
        }
    }
}

impl cmp::PartialEq<str> for Rope {
    /// A rope equals a string if all the bytes in the string equal the rope's.
    ///
    /// # Examples
    /// ```
    /// use an_rope::Rope;
    /// assert!(&Rope::from("abcd") == "abcd");
    /// ```
    /// ```
    /// use an_rope::Rope;
    /// assert!(&Rope::from("abcd") != "ab");
    /// ```
    /// ```
    /// use an_rope::Rope;
    /// assert!(&Rope::from("abcd") != "dcab");
    /// ```
    #[inline]
    fn eq(&self, other: &str) -> bool {
        if self.len() == other.len() {
            self.bytes_eq(other.bytes())
        } else {
            false
        }
    }
}

//-- concatenation --------------------------------------------------
impl<'a> ops::Add for &'a Rope {
    type Output = Rope;
    /// Non-destructively concatenate two `Rope`s, returning a new `Rope`.
    ///
    /// # Examples
    /// ```
    /// use an_rope::Rope;
    /// let rope = Rope::from(String::from("ab"));
    /// assert_eq!( &rope + &Rope::from(String::from("cd"))
    ///           , Rope::from(String::from("abcd")) );
    /// ```
    #[inline] fn add(self, other: Self) -> Rope { self.with_append((*other).clone()) }

}

impl ops::Add for Rope {
    type Output = Rope;
    /// Non-destructively concatenate two `Rope`s, returning a new `Rope`.
    ///
    /// # Examples
    /// ```
    /// use an_rope::Rope;
    /// let rope = Rope::from(String::from("ab"));
    /// assert_eq!( rope + Rope::from(String::from("cd"))
    ///           , Rope::from(String::from("abcd")) );
    /// ```
    #[inline] fn add(self, other: Self) -> Rope { self.with_append(other) }
}

impl ops::Add<String> for Rope {
    type Output = Rope;
    /// Non-destructively concatenate a `Rope` and a `String`.
    ///
    /// Returns a new `Rope`
    ///
    /// # Examples
    /// ```
    /// use an_rope::Rope;
    /// let rope = Rope::from(String::from("ab"));
    /// assert_eq!( rope + String::from("cd")
    ///           , Rope::from(String::from("abcd")));
    /// ```
    #[inline] fn add(self, other: String) -> Rope {
         self.with_append(Rope::from(other))
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
    /// use an_rope::Rope;
    /// let rope = Rope::from(String::from("ab"));
    /// assert_eq!( &rope + "cd"
    ///           , Rope::from(String::from("abcd")));
    /// ```
    #[inline] fn add(self, other: &'b str) -> Rope {
         self.with_append(Rope::from(other.to_owned()))
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
    /// use an_rope::Rope;
    /// let rope = Rope::from(String::from("ab"));
    /// assert_eq!( rope + "cd"
    ///           , Rope::from(String::from("abcd")));
    /// ```
    #[inline] fn add(self, other: &'a str) -> Rope {
         self.with_append(Rope::from(other.to_owned()))
     }

}


impl ops::AddAssign for Rope {

    /// Concatenate two `Rope`s mutably.
    ///
    /// # Examples
    /// ```
    /// use an_rope::Rope;
    /// let mut rope = Rope::from(String::from("ab"));
    /// rope += Rope::from(String::from("cd"));
    /// assert_eq!(rope, Rope::from(String::from("abcd")));
    /// ```
    #[inline]
    fn add_assign(&mut self, other: Rope) {
        self.append(other)
    }
}

impl ops::AddAssign<String> for Rope {

    /// Concatenate a `String` onto a `Rope` mutably.
    ///
    /// # Examples
    /// ```
    /// use an_rope::Rope;
    /// let mut rope = Rope::from(String::from("ab"));
    /// rope += String::from("cd");
    /// assert_eq!(rope, Rope::from(String::from("abcd")));
    /// ```
    #[inline]
    fn add_assign(&mut self, string: String) {
        self.append(Rope::from(string))
    }
}

impl<'a> ops::AddAssign<&'a str> for Rope {

    /// Concatenate an `&str` onto a `Rope` mutably.
    ///
    /// # Examples
    /// ```
    /// use an_rope::Rope;
    /// let mut rope = Rope::from(String::from("ab"));
    /// rope += String::from("cd");
    /// assert_eq!(rope, Rope::from(String::from("abcd")));
    /// ```
    #[inline]
    fn add_assign(&mut self, string: &'a str) {
        self.append(Rope::from(string.to_owned()))
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

impl<'a> Borrow<RopeSlice<'a>> for &'a Rope {
    fn borrow(&self) -> &RopeSlice<'a> {
        unimplemented!()
    }
}

impl iter::Extend<char> for Rope{

    fn extend<T>(&mut self, iter: T)
    where T: IntoIterator<Item=char> {
        let s: String = iter.into_iter().collect();
        let r: Rope = Rope::from(s);
        self.append(r);
    }

}

impl iter::Extend<String> for Rope {

    fn extend<T>(&mut self, iter: T)
    where T: IntoIterator<Item=String> {
        for s in iter {self.append(Rope::from(s));}
    }

}

impl<'a> iter::Extend<&'a str> for Rope {

    fn extend<T>(&mut self, iter: T)
    where T: IntoIterator<Item=&'a str> {
        for s in iter {self.append(Rope::from(s));}
    }

}

impl<'a> iter::Extend<&'a char> for Rope {

    fn extend<T>(&mut self, iter: T)
    where T: IntoIterator<Item=&'a char> {
        let s: String = iter.into_iter().fold(String::new(), |mut acc, x| {acc.push(*x); acc});
        let r: Rope = Rope::from(s);
        self.append(r);
    }

}

impl iter::Extend<Rope> for Rope {

    fn extend<T>(&mut self, iter: T)
    where T: IntoIterator<Item=Rope> {
        for r in iter {self.append(r);}
    }

}
