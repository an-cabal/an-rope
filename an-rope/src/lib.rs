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

use std::cmp;
use std::ops;
use std::convert;

#[cfg(test)]
mod test;

use self::internals::Node;

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
#[derive(Debug, Clone)]
pub struct Rope {
    // can we get away with having these be of &str or will they need
    // to be string?
    root: Node
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
    /// An empty `Rope` should have length 0.
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
    /// A `Rope` with text should have length equal to the number of
    /// characters in the `Rope`.
    /// ```
    /// use an_rope::Rope;
    /// let mut an_rope = Rope::from(String::from("a string"));
    /// assert_eq!(an_rope.len(), "a string".len());
    /// ```
    pub fn len(&self) -> usize { self.root.len() }

    /// Insert `char` into `index` in this `Rope`,
    ///
    /// # Panics
    /// + If `index` is greater than the length of this `Rope`
    ///
    /// # Time Complexity
    /// O(log _n_)
    ///
    /// # Examples
    ///
    /// Inserting at index 0 prepends `char` to this `Rope`:
    /// ```
    /// let mut an_rope = Rope::from("bcd");
    /// an_rope.insert(0, 'a');
    /// assert_eq!(an_rope, Rope::from("abcd"));
    /// ```
    ///
    /// Inserting at index `len` prepends `char` to this `Rope`:
    /// ```
    /// let mut an_rope = Rope::from("abc");
    /// an_rope.insert(an_rope.len(), 'd');
    /// assert_eq!(an_rope, Rope::from("abcd"));
    /// ```
    ///
    ///Inserting at an index in the middle inserts `char` at that index:
    /// ```
    /// let mut an_rope = Rope::from("acd");
    /// an_rope = an_rope.insert(1, 'b');
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

    /// Insert `ch` into `index` in this `Rope`, returning a new `Rope`.
    ///
    ///
    /// # Returns
    /// + A new `Rope` with `ch` inserted at `index`
    ///
    /// # Time Complexity
    /// O(log _n_)
    ///
    /// # Panics
    /// + If `index` is greater than the length of this `Rope`
    ///
    /// # Examples
    ///
    /// Inserting at index 0 prepends `rope` to this `Rope`:
    /// ```
    /// let mut an_rope = Rope::from("bcd");
    /// an_rope = an_rope.with_insert(0, 'a'));
    /// assert_eq!(an_rope, Rope::from("abcd"));
    /// ```
    ///
    /// Inserting at index `len` prepends `char` to this `Rope`:
    /// ```
    /// let mut an_rope = Rope::from("abc");
    /// an_rope = an_rope.with_insert(an_rope.len(), 'd');
    /// assert_eq!(an_rope, Rope::from("abcd"));
    /// ```
    ///
    ///Inserting at an index in the middle inserts `char` at that index:
    /// ```
    /// let mut an_rope = Rope::from("acd");
    /// an_rope = an_rope.with_insert(1, 'b'));
    /// assert_eq!(an_rope, Rope::from("abcd"));
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
    /// + If `index` is greater than the length of this `Rope`
    ///
    /// # Time Complexity
    /// O(log _n_)
    ///
    /// # Examples
    ///
    /// Inserting at index 0 prepends `rope` to this `Rope`:
    /// ```
    /// let mut an_rope = Rope::from("cd");
    /// an_rope.insert_rope(0, Rope::from("ab"));
    /// assert_eq!(an_rope, Rope::from("abcd"));
    /// ```
    ///
    /// Inserting at index `len` prepends `rope` to this `Rope`:
    /// ```
    /// let mut an_rope = Rope::from("ab");
    /// an_rope.insert_rope(an_rope.len(), Rope::from("cd"));
    /// assert_eq!(an_rope, Rope::from("abcd"));
    /// ```
    ///
    ///Inserting at an index in the middle inserts `rope` at that index:
    /// ```
    /// let mut an_rope = Rope::from("ad");
    /// an_rope = an_rope.insert_rope(1, Rope::from("bd"));
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
    /// # Returns
    /// + A new `Rope` with `rope` inserted at `index`
    ///
    /// # Time Complexity
    /// O(log _n_)
    ///
    /// # Panics
    /// + If `index` is greater than the length of this `Rope`
    ///
    /// # Examples
    ///
    /// Inserting at index 0 prepends `rope` to this `Rope`:
    /// ```
    /// let mut an_rope = Rope::from("cd");
    /// an_rope = an_rope.with_insert_rope(0, Rope::from("ab"));
    /// assert_eq!(an_rope, Rope::from("abcd"));
    /// ```
    ///
    /// Inserting at index `len` prepends `rope` to this `Rope`:
    /// ```
    /// let mut an_rope = Rope::from("ab");
    /// an_rope = an_rope.with_insert_rope(an_rope.len(), Rope::from("cd"));
    /// assert_eq!(an_rope, Rope::from("abcd"));
    /// ```
    ///
    ///Inserting at an index in the middle inserts `rope` at that index:
    /// ```
    /// let mut an_rope = Rope::from("ad");
    /// an_rope = an_rope.with_insert_rope(1, Rope::from("bd"));
    /// assert_eq!(an_rope, Rope::from("abcd"));
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
    /// + If `index` is greater than the length of this `Rope`
    ///
    /// # Time Complexity
    /// O(log _n_)
    ///
    /// # Examples
    ///
    /// Inserting at index 0 prepends `s` to this `Rope`:
    /// ```
    /// let mut an_rope = Rope::from("cd");
    /// an_rope.insert_str(0, "ab");
    /// assert_eq!(an_rope, Rope::from("abcd"));
    /// ```
    ///
    /// Inserting at index `len` prepends `s` to this `Rope`:
    /// ```
    /// let mut an_rope = Rope::from("ab");
    /// an_rope.insert_str(an_rope.len(), "cd");
    /// assert_eq!(an_rope, Rope::from("abcd"));
    /// ```
    ///
    ///Inserting at an index in the middle inserts `s` at that index:
    /// ```
    /// let mut an_rope = Rope::from("ad");
    /// an_rope = an_rope.insert_str(1, "bd");
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
    /// + A new `Rope` with `s` inserted at `index`
    ///
    /// # Panics
    /// + If `index` is greater than the length of this `Rope`
    ///
    /// # Time Complexity
    /// O(log _n_)
    ///
    /// # Examples
    ///
    /// Inserting at index 0 prepends `s` to this `Rope`:
    /// ```
    /// let mut an_rope = Rope::from("cd");
    /// an_rope = an_rope.with_insert_str(0, "ab");
    /// assert_eq!(an_rope, Rope::from("abcd"));
    /// ```
    ///
    /// Inserting at index `len` prepends `s` to this `Rope`:
    /// ```
    /// let mut an_rope = Rope::from("ab");
    /// an_rope = an_rope.with_insert_str(an_rope.len(), "cd");
    /// assert_eq!(an_rope, Rope::from("abcd"));
    /// ```
    ///
    ///Inserting at an index in the middle inserts `s` at that index:
    /// ```
    /// let mut an_rope = Rope::from("ad");
    /// an_rope = an_rope.with_insert_str(1, "bd");
    /// assert_eq!(an_rope, Rope::from("abcd"));
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
    /// ```
    /// use an_rope::Rope;
    /// let an_rope = Rope::from("efgh");
    /// let another_rope = an_rope.with_prepend(Rope::from("abcd"));
    /// assert_eq!(&an_rope, "efgh");
    /// assert_eq!(&another_rope, "abcdefgh");
    /// ```
    /// ```
    /// use an_rope::Rope;
    /// let an_rope = Rope::from("");
    /// let another_rope = an_rope.with_prepend(Rope::from("abcd"));
    /// assert_eq!(&an_rope, "");
    /// assert_eq!(&another_rope, "abcd");
    /// ```
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
        // #[inline]
        // impl char_indices<(usize, char)> for Rope {}
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
}

impl convert::Into<Vec<u8>> for Rope {
    fn into(self) -> Vec<u8> {
        unimplemented!()
    }

}

impl convert::From<String> for Rope {
    #[inline]
    fn from(string: String) -> Rope {
        Rope {
            root: if string.len() == 0 { Node::empty() }
                  else { Node::new_leaf(string) }
        }
    }
}

impl<'a> convert::From<&'a str> for Rope {
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
