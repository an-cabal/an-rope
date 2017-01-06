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

#![cfg_attr( feature = "unstable"
           , feature( const_fn
                    , box_syntax, box_patterns
                    , conservative_impl_trait
                    , collections, collections_range
                    , inclusive_range_syntax
                    ))]
#![cfg_attr( all( test, feature = "unstable")
           , feature( test, insert_str) )]
#![cfg_attr( feature = "clippy", feature(plugin) )]
#![cfg_attr( feature = "clippy", plugin(clippy) )]
#![cfg_attr( feature = "clippy", allow(unused_variables, dead_code))]

#[cfg(feature = "unstable")]
extern crate collections;
#[cfg(feature = "unstable")]
use collections::range::RangeArgument;

extern crate unicode_segmentation;


use std::borrow::Borrow;
use std::cmp;
use std::ops;
use std::convert;
use std::fmt;
use std::string;
use std::iter;

#[cfg(feature = "tendril")] extern crate tendril;
#[cfg(feature = "tendril")] use tendril::StrTendril;

#[cfg(test)] #[macro_use] extern crate quickcheck;
#[cfg(test)] mod test;
#[cfg(all( test, feature = "unstable"))] mod bench;

mod unicode;
pub mod metric;

use metric::{Measured, Metric};

use self::internals::{Node, BranchNode};
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
#[derive(Clone, Default)]
pub struct Rope {
    // can we get away with having these be of &str or will they need
    // to be string?
    root: Node
}

impl<M> Measured<M> for Rope
where M: Metric
    , Node: Measured<M>
    , BranchNode: Measured<M>
    , String: Measured<M>
    {

    #[inline] fn to_byte_index(&self, index: M) -> Option<usize> {
        self.root.to_byte_index(index)
    }

    #[inline] fn measure(&self) -> M { self.root.measure() }

    #[inline] fn measure_weight(&self) -> M { self.root.measure_weight() }

}

impl fmt::Debug for Rope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rope[\"{}\"] {:?}", self.root, self.root)
    }
}
 #[cfg(feature = "unstable")]
macro_rules! unstable_iters {
    ( $($(#[$attr:meta])*
     pub fn $name:ident$(<$lf:tt>)*(&'a $sel:ident) -> $ty:ty {
         $body:expr
     })+ ) => { $(
         $(#[$attr])*
         #[cfg_attr(feature = "clippy", allow(needless_lifetimes))]
         pub fn $name$(<$lf>)*(&'a $sel) -> $ty {
             $body
         }
    )+ };
    ( $($(#[$attr:meta])*
    pub fn $name:ident$(<$lf:tt>)*(&'a mut $sel:ident) -> $ty:ty {
         $body:expr
     })+ ) => { $(
         $(#[$attr])*
         #[cfg(feature = "unstable")]
         #[cfg_attr(feature = "clippy", allow(needless_lifetimes))]
         pub fn $name$(<$lf>)*(&'a mut $sel) -> $ty {
             $body
         }
    )+ };
}

#[cfg(not(feature = "unstable"))]
macro_rules! unstable_iters {
    ( $($(#[$attr:meta])*
    pub fn $name:ident$(<$lf:tt>)*(&'a $sel:ident) -> impl Iterator<Item=$ty:ty> + 'a {
         $body:expr
     })+ ) => ($(
         $(#[$attr])*
         #[cfg(not(feature = "unstable"))]
         #[cfg_attr(feature = "clippy", allow(needless_lifetimes))]
         pub fn $name$(<$lf>)*(&'a $sel) -> Box<Iterator<Item=$ty> + 'a> {
             Box::new($body)
         }
     )+);
    ( $( $(#[$attr:meta])*
    pub fn $name:ident$(<$lf:tt>)*(&'a mut $sel:ident) - impl Iterator<Item=$ty:ty> + 'a {
         $body:expr
     })+ ) => { $({
         $(#[$attr])*
         #[cfg(not(feature = "unstable"))]
         #[cfg_attr(feature = "clippy", allow(needless_lifetimes))]
         pub fn $name$(<$lf>)*(&'a mut $sel) -> Box<Iterator<Item=$ty> + 'a> {
             Box::new($body)
         }
     })+
    };
}
macro_rules! str_iters {
    ( $($(#[$attr:meta])* impl $name: ident<$ty: ty> for Node {})+ ) => { $(
        unstable_iters! {
            $(#[$attr])*
            pub fn $name<'a>(&'a self) -> impl Iterator<Item=$ty> + 'a{
                self.strings().flat_map(str::$name)
            }
        }
    )+ };

    ( $($(#[$attr:meta])* impl $name: ident<$ty: ty> for Rope {})+ )=> { $(
        unstable_iters! {
            $(#[$attr])*
            pub fn $name<'a>(&'a self) -> impl Iterator<Item=$ty>  + 'a{
                self.root.$name()
            }
        }
    )+ }

}


macro_rules! unicode_seg_iters {
    ( $($(#[$attr:meta])* impl $name: ident for Node { extend })+ ) => { $(

        unstable_iters! {
            $(#[$attr])*
            pub fn $name<'a>(&'a self) -> impl Iterator<Item=&'a str> + 'a {
                { // this block is required so that the macro will bind the
                  // `use` statement
                    use unicode_segmentation::UnicodeSegmentation;
                    self.strings()
                        .flat_map(|s| UnicodeSegmentation::$name(s, true))
                }
            }
        }
    )+ };
    ( $($(#[$attr:meta])* impl $name: ident for Node {} )+ ) => { $(
        unstable_iters!{
            $(#[$attr])*
            pub fn $name<'a>(&'a self) -> impl Iterator<Item=&'a str> + 'a {
                { // this block is required so that the macro will bind the
                  // `use` statement
                    use unicode_segmentation::UnicodeSegmentation;
                    self.strings().flat_map(UnicodeSegmentation::$name)
                }
            }
        }
    )+ };
    ( $($(#[$attr:meta])* impl $name: ident<$ty: ty> for Rope {})+ )=> { $(
        unstable_iters! {
            $(#[$attr])*
            pub fn $name<'a>(&'a self) -> impl Iterator<Item=$ty> + 'a {
                self.root.$name()
            }
        }
    )+ }

}

mod internals;
mod slice;

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

    /// Returns `true` if this `Rope` is empty.
    ///
    /// # Examples
    ///
    /// A `Rope` with no characters should be empty:
    ///
    /// ```
    /// use an_rope::Rope;
    /// let an_empty_rope = Rope::new();
    /// assert!(an_empty_rope.is_empty());
    /// ```
    ///
    /// ```
    /// use an_rope::Rope;
    /// let an_empty_rope = Rope::from(String::from(""));
    /// assert!(an_empty_rope.is_empty());
    /// ```
    ///
    /// A `Rope` with characters should not be empty:
    ///
    /// ```
    /// use an_rope::Rope;
    /// let an_rope = Rope::from("a string");
    /// assert!(!an_rope.is_empty());
    /// ```
    #[inline] pub fn is_empty(&self) -> bool { self.len() == 0 }

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
    pub fn insert<M>(&mut self, index: M, ch: char)
    where M: Metric
        , Self: Measured<M>
        , Node: Measured<M>
        , BranchNode: Measured<M>
        , String: Measured<M>
        {
        assert!( index <= self.measure()
               , "Rope::insert: index {:?} was > length {:?}"
               , index, self.measure());
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
    #[cfg(feature = "unstable")]
    pub fn delete<R, M>(&mut self, range: R)
    where R: RangeArgument<M>
        , M: Metric
        , Rope: Measured<M>
        , Node: Measured<M>
        , BranchNode: Measured<M>
        , String: Measured<M>
        {
        let start = range.start().map(|s| *s)
                         .unwrap_or_else(|| { M::default() });
        let end = range.end().map(|e| *e)
                       .unwrap_or_else(|| { self.measure() });

        assert!( start <= end
               , "invalid index! start {:?} > end {:?}", end, start);
        let (l, r) = self.take_root().split(start);
        let (_, r) = r.split(end - start);
        self.root = Node::new_branch(l, r);
    }

    #[inline]
    #[cfg(not(feature = "unstable"))]
    pub fn delete<M: Metric>(&mut self, range: ops::Range<M>)
    where Node: Measured<M>
        , internals::BranchNode: Measured<M>
        , String: Measured<M>
        {
        let (l, r) = self.take_root().split(range.start);
        let (_, r) = r.split(range.end - range.start);
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
    pub fn with_insert<M>(&self, index: M, ch: char) -> Rope
    where M: Metric
        , Self: Measured<M>
        , Node: Measured<M>
        , BranchNode: Measured<M>
        , String: Measured<M>
        {
        assert!( index <= self.measure()
               , "Rope::with_insert: index {:?} was > length {:?}"
               , index, self.measure());
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
    #[inline]
    pub fn insert_rope<M>(&mut self, index: M, rope: Rope)
    where M: Metric
        , Self: Measured<M>
        , Node: Measured<M>
        , BranchNode: Measured<M>
        , String: Measured<M>
        {
        if !rope.is_empty() {
            let len = self.measure();
            if index.into() == 0 {
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
    pub fn with_insert_rope<M>(&self, index: M, rope: Rope) -> Rope
    where M: Metric
        , Self: Measured<M>
        , Node: Measured<M>
        , BranchNode: Measured<M>
        , String: Measured<M>
        {
        assert!( index <= self.measure()
               , "Rope::with_:insert_rope: index {:?} was > length {:?}"
               , index, self.measure());
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
    pub fn insert_str<M>(&mut self, index: M, s: &str)
    where M: Metric
        , Self: Measured<M>
        , Node: Measured<M>
        , BranchNode: Measured<M>
        , String: Measured<M>
        {
        assert!( index <= self.measure()
               , "Rope::insert_str: index {:?} was > length {:?}"
               , index, self.measure());
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
    pub fn with_insert_str<M>(&self, index: M, s: &str) -> Rope
    where M: Metric
        , Self: Measured<M>
        , Node: Measured<M>
        , BranchNode: Measured<M>
        , String: Measured<M>
        {
        assert!( index <= self.measure()
               , "Rope::with_insert_str: index {:?} was > length {:?}"
               , index, self.measure());
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
        if !other.is_empty() {
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
        if other.is_empty() {
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
        if !other.is_empty() {
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
        if other.is_empty() {
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
    pub fn split<M: Metric>(self, index: M) -> (Rope, Rope)
    where Self: Measured<M>
        , Node: Measured<M>
        , internals::BranchNode: Measured<M>
        , String: Measured<M>
        {
        assert!(index <= self.measure());
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

    unstable_iters! {
        #[doc="Returns an iterator over all the strings in this `Rope`"]
        #[inline]
        pub fn strings<'a>(&'a self) -> impl Iterator<Item=&'a str> + 'a {
            self.root.strings()
        }

        #[doc="Returns an iterator over all the lines of text in this `Rope`."]
        pub fn lines<'a>(&'a self) -> impl Iterator<Item=RopeSlice<'a>> +'a  {
            {   // create a new block here so the macro will bind the `use` stmt
                use internals::IsLineEnding;
                let last_idx = self.len() - 1;
                Box::new(self.char_indices()
                             .filter_map(move |(i, c)|
                                if c.is_line_ending() { Some(i) }
                                // special case: slice to the end of the rope
                                // even if it doesn't end in a newline character
                                else if i == last_idx { Some(i + 1) }
                                else { None })
                              .scan(0, move |mut l, i|  {
                                    let last = *l;
                                    *l = i + 1;
                                    Some(self.slice(last..i))
                                }))
            }
        }
    }


    /// Returns a move iterator over all the strings in this `Rope`
    ///
    /// Consumes `self`.
    #[cfg(feature = "unstable")]
    #[inline]
    pub fn into_strings<'a>(self) -> impl Iterator<Item=String> + 'a {
        self.root.into_strings()
    }

    /// Returns a move iterator over all the strings in this `Rope`
    ///
    /// Consumes `self`.
    #[cfg(not(feature = "unstable"))]
    #[inline]
    pub fn into_strings<'a>(self) -> Box<Iterator<Item=String> + 'a> {
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
        // #[inline]
        // impl lines<&'a str> for Rope {}
    }

    unicode_seg_iters! {
        #[doc=
            "Returns an iterator over the [grapheme clusters][graphemes] of \
             `self`.\n\

             [graphemes]: \
             http://www.unicode.org/reports/tr29/#Grapheme_Cluster_Boundaries\
             \n\
             The iterator is over the  *extended grapheme clusters*; as \
             [UAX#29]\
             (http://www.unicode.org/reports/tr29/#Grapheme_Cluster_Boundaries)\
             recommends extended grapheme cluster boundaries for general \
             processing."]
        #[inline]
        impl graphemes<&'a str> for Rope {}
        #[doc=
            "Returns an iterator over the words of `self`, separated on \
            [UAX#29 word boundaries]\
            (http://www.unicode.org/reports/tr29/#Word_Boundaries).\n\n\

            Here, \"words\" are just those substrings which, after splitting on\
            UAX#29 word boundaries, contain any alphanumeric characters. That \
            is, the substring must contain at least one character with the \
            [Alphabetic](http://unicode.org/reports/tr44/#Alphabetic) \
            property, or with [General_Category=Number]\
            (http://unicode.org/reports/tr44/#General_Category_Values)."]
        #[inline]
        impl unicode_words<&'a str> for Rope {}
        #[doc=
            "Returns an iterator over substrings of `self` separated on \
            [UAX#29 word boundaries]\
            (http://www.unicode.org/reports/tr29/#Word_Boundaries). \n\n\
            The concatenation of the substrings returned by this function is \
            just the original string."]
        #[inline]
        impl split_word_bounds<&'a str> for Rope {}
        // #[inline]
        // impl grapheme_indices<(usize, &'a str)> for Rope {}
        // #[inline]
        // impl split_word_bound_indices<(usize, &'a str)> for Rope {}
    }

    /// Returns an iterator over the grapheme clusters of `self` and their
    /// byte offsets. See `graphemes()` for more information.
    ///
    /// # Examples
    ///
    /// ```
    /// # use an_rope::Rope;
    /// let rope = Rope::from("a̐éö̲\r\n");
    /// let gr_inds = rope.grapheme_indices()
    ///                   .collect::<Vec<(usize, &str)>>();
    /// let b: &[_] = &[(0, "a̐"), (3, "é"), (6, "ö̲"), (11, "\r\n")];
    ///
    /// assert_eq!(&gr_inds[..], b);
    /// ```
    #[inline]
    pub fn grapheme_indices(&self) -> internals::GraphemeIndices {
        self.root.grapheme_indices()
    }

    /// Returns an iterator over substrings of `self`, split on UAX#29 word
    /// boundaries, and their offsets. See `split_word_bounds()` for more
    /// information.
    ///
    /// # Example
    ///
    /// ```
    /// # use an_rope::Rope;
    /// let rope = Rope::from("Brr, it's 29.3°F!");
    /// let swi1 = rope.split_word_bound_indices()
    ///                .collect::<Vec<(usize, &str)>>();
    /// let b: &[_] = &[ (0, "Brr"), (3, ","), (4, " "), (5, "it's")
    ///                , (9, " "), (10, "29.3"),  (14, "°"), (16, "F")
    ///                , (17, "!")];
    ///
    /// assert_eq!(&swi1[..], b);
    /// ```
    #[inline]
    pub fn split_word_bound_indices(&self) -> internals::UWordBoundIndices {
        self.root.split_word_bound_indices()
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
    #[inline]
    #[cfg(feature = "unstable")]
    pub fn slice<R>(&self, range: R) -> RopeSlice
    where R: RangeArgument<usize> {
        RopeSlice::new(&self.root, range)
    }
    #[cfg(not(feature = "unstable"))]
    pub fn slice(&self, range: ops::Range<usize>) -> RopeSlice {
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
    #[inline]
    #[cfg(feature = "unstable")]
    pub fn slice_mut<R>(&mut self, range: R) -> RopeSliceMut
    where R: RangeArgument<usize> {
        RopeSliceMut::new(&mut self.root, range)
    }
    #[cfg(not(feature = "unstable"))]
    pub fn slice_mut(&mut self, range: ops::Range<usize>) -> RopeSliceMut {
        RopeSliceMut::new(&mut self.root, range)
    }

}

impl convert::Into<Vec<u8>> for Rope {
    fn into(self) -> Vec<u8> {
        unimplemented!()
    }

}

fn str_to_tree(string: String) -> Node {
    assert!(!string.is_empty());
    let mut strings = string.rsplit('\n');
    let last: Node = Node::new_leaf(String::from(strings.next().unwrap()));
    let leaves = strings.map(|s| Node::new_leaf(String::from(s) + "\n"));
    leaves.fold(last, |r, l| Node::new_branch(l, r))
}

#[cfg(feature = "tendril")]
impl convert::From<StrTendril> for Rope {
    fn from(tendril: StrTendril) -> Rope {
        Rope { root: str_to_tree(tendril) }
    }
}

impl convert::From<String> for Rope {


    #[cfg(feature = "tendril")]
    #[inline]
    fn from(string: String) -> Rope {
        Rope {
            root: if string.is_empty() { Node::empty() }
                  else { str_to_tree(StrTendril::from(string)) }
        }
    }


    #[cfg(not(feature = "tendril"))]
    #[inline]
    fn from(string: String) -> Rope {
        Rope {
            root: if string.is_empty() { Node::empty() }
                  else { str_to_tree(string) }
        }
    }
}


impl<'a> convert::From<&'a str> for Rope {
    #[cfg(feature = "tendril")]
    #[inline]
    fn from(string: &'a str) -> Rope {
         Rope::from(StrTendril::from_slice(string))
     }

    #[cfg(not(feature = "tendril"))]
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


impl cmp::PartialEq<String> for Rope {
    /// A rope equals a string if all the bytes in the string equal the rope's.
    ///
    /// # Examples
    /// ```
    /// use an_rope::Rope;
    /// assert!(Rope::from("abcd") == String::from("abcd"));
    /// ```
    /// ```
    /// use an_rope::Rope;
    /// assert!(Rope::from("abcd") != String::from("ab"));
    /// ```
    /// ```
    /// use an_rope::Rope;
    /// assert!(Rope::from("abcd") != String::from("dcab"));
    /// ```
    #[inline]
    fn eq(&self, other: &String) -> bool {
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

impl<A> iter::Extend<A> for Rope
where Rope: iter::FromIterator<A>{

    fn extend<B>(&mut self, iter: B)
    where B: IntoIterator<Item=A> {

        self.append(iter.into_iter().collect());

    }

}

impl iter::FromIterator<char> for Rope {

    fn from_iter<I>(iter: I) -> Rope
    where I: IntoIterator<Item=char> {
        let s: String = iter.into_iter().collect();
        Rope::from(s)

    }

}

impl iter::FromIterator<String> for Rope {

    fn from_iter<I>(iter: I) -> Rope
    where I: IntoIterator<Item=String> {
        iter.into_iter().fold(Rope::new(), |mut acc, x| {acc.append(Rope::from(x)); acc})
    }

}

impl iter::FromIterator<Rope> for Rope {

    fn from_iter<I>(iter: I) -> Rope
    where I: IntoIterator<Item=Rope> {
        iter.into_iter().fold(Rope::new(), |mut acc, x| {acc.append(x); acc})
    }

}

impl<'a> iter::FromIterator<&'a char> for Rope {

    fn from_iter<I>(iter: I) -> Rope
    where I: IntoIterator<Item=&'a char> {
        let s: String = iter.into_iter().fold(String::new(), |mut acc, x| {acc.push(*x); acc});
        Rope::from(s)
    }

}

impl<'a> iter::FromIterator<&'a str> for Rope {

    fn from_iter<I>(iter: I) -> Rope
    where I: IntoIterator<Item=&'a str> {
        iter.into_iter().fold(Rope::new(), |mut acc, x| {acc.append(Rope::from(x)); acc})
    }

}
