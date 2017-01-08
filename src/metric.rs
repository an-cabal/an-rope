//! Metrics for indexing `Rope`s.
//!
//! A [`Metric`] represents a measurement with which indices into a [`Rope`]
//! may be calculated.
//!
//! All [`Rope`] methods are optionally parameterised with [`Metric`]s. This
//! means that you can [`split`], [`insert`], or [`delete`] Ropes on character,
//! line, or grapheme indices, without necessitating the addition of a whole
//! bunch of new, wordy method names like `split_on_grapheme_index` and so on.
//!
//! # Examples
//!
//! If I wanted to delete characters 10 to 15 from rope _r_, I could say:
//!
//! ```
//! # use an_rope::Rope;
//! let mut r = Rope::from("this is a long rope");
//! r.delete(10..15);
//! assert_eq!(&r, "this is a rope");
//! ```
//!
//! Suppose my `Rope` contained some strange Unicode characters, and I realised
//! that I actually wanted to delete _graphemes_ 10 to 13. In that case, I
//! could say:
//!
//! ```
//! # use an_rope::Rope;
//! use an_rope::metric::Grapheme;
//! let mut r = Rope::from("this is a 🆒🆕 rope, 🆗!");
//! r.delete(Grapheme(10)..Grapheme(13));
//! assert_eq!(&r, "this is a rope, 🆗!");
//! ```
//!
//! Or, suppose my `Rope` spanned multiple lines:
//!
//! ```
//! # use an_rope::Rope;
//! use an_rope::metric::Line;
//! let mut r = Rope::from("this is\n\
//!                         a\n\
//!                         multi\n\
//!                         line\n\
//!                         rope");
//! r.delete(Line(2)..Line(3));
//! assert_eq!(&r, "this is\na\nrope");
//! ```
//!
//! [`Metric`]: trait.Metric.html
//! [`Rope`]: ../struct.Rope.html
//! [`split`]: ../struct.Rope.html#method.split
//! [`insert`]: ../struct.Rope.html#method.insert
//! [`delete`]: ../struct.Rope.html#method.delete

use std::convert;
use std::ops::{Add, Sub};
use std::default::Default;
use std::fmt;

/// The class of monoids
///
/// [Monoid]s are types with an accumulative binary operation that has
/// an identity.
///
/// Technically, `Add<Self, Output=Self>` is standing in for "semigroup" here,
/// while [`Default`] is standing in for "identity"[^id].
///
/// An instance _M_ should satisfy the following laws:
///
///  + _x_`.add(`_M_`::default())` = _x_
///  + _M_`::default().add(`_x_`)` = _x_
///  + _x_`.add(`_y_`.add(`_z_`))` = _z_`.add(`_x_`.add(`_y_`))`
///  + _M_`::accumulate(`_a_`)` = _a_`.fold(`_M_`::default,`_M_`::sum)`
///
/// [^id]: A mathematician might point out that it might be more correct to
///        represent the "identity" operation using the [`Zero`] trait rather
///        than [`Default`], as the documentation for `Zero` notes that "[t]his
///        trait is intended for use in conjunction with `Add`, as an identity".
///        However, the `Zero` trait is marked as unstable, so it would only be
///        useable on nightly Rust, and its use is deprecated. Thus, `Default`.
/// [`Default`]: https://doc.rust-lang.org/std/default/trait.Default.html)
/// [`Zero`]: https://doc.rust-lang.org/std/num/trait.Zero.html
/// [Monoid]: http://mathworld.wolfram.com/Monoid.html
pub trait Monoid: Add<Self, Output=Self> + Default + Sized {
    #[inline]
    fn accumulate<F>(xs: F) -> Self
    where F: Iterator<Item=Self>
        , Self: Sized {
        xs.fold(Self::default(), Self::add)
    }
}

/// Trait indicating that a type may be measured with [`Metric`] _M_.
///
/// [`Metric`]: trait.Metric.html
pub trait Measured<M: Metric> {
    /// Apply `Metric` to `Self`
    ///
    /// Although we aren't currently enforcing this, `measure`ing a `Node` with
    /// two children should produce the same result as `measure`ing both
    /// children and `Monoid::sum`ming the result. That is to say, `measure`
    /// should be a [_monoid homomorphism_].
    ///
    /// [_monoid homomorphism_]:
    /// https://en.wikipedia.org/wiki/Monoid#Monoid_homomorphisms
    fn measure(&self) -> M;

    /// Measure the `weight` of `Node` by this `metric`.
    fn measure_weight(&self) -> M;

    /// Convert the `Metric` into a byte index into the given `Node`
    ///
    /// # Returns
    /// - `Some` with the byte index of the beginning of the `n`th  element
    ///    in `node` measured by this `Metric`, if there is an `n`th element
    /// - `None` if there is no `n`th element in `node`
    fn to_byte_index(&self, index: M) -> Option<usize>;
}

/// A [monoid] that can be applied to a type as a measurement.
///
/// [monoid]: trait.Monoid.html
pub trait Metric: Monoid + Eq + Add<usize, Output=Self>
                         + Sub<usize, Output=Self>
                         + Sub<Self, Output=Self>
                         + Eq + Ord
                         + Sized
                         + convert::Into<usize>
                         + Copy
                         + fmt::Debug {

    /// Returns whether text may be split into new leaf nodes using this metric.
    fn is_splittable() -> bool;

    /// Returns the byte index of the next element of this metric in `Node`
    #[inline]
    fn next<M: Measured<Self> >(self, node: &M)-> Option<usize> {
        node.to_byte_index(self + 1)
    }

    /// Returns the byte index of the previous element of this metric in `Node`
    #[inline]
    fn back<M: Measured<Self>>(self, node: &M) -> Option<usize> {
        node.to_byte_index(self - 1)
    }

    /// Returns true if index `i` in `node` is a boundary along this `Metric`
    fn is_boundary<M: Measured<Self>>(node: &M, i: usize) -> bool;
}

/// A metric for calculating indices in `Rope`s based on Unicode graphemes.
#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct Grapheme(pub usize);
impl Default for Grapheme {
    #[inline] fn default() -> Self { Grapheme(0) }
}
impl convert::From<usize> for Grapheme {
    #[inline] fn from(u: usize) -> Self { Grapheme(u) }
}
impl convert::Into<usize> for Grapheme {
    #[inline] fn into(self) -> usize { self.0 }
}
impl Add<usize> for Grapheme {
    type Output = Self;
    #[inline] fn add(self, rhs: usize) -> Self { Grapheme(self.0 + rhs) }
}
impl Add<Grapheme> for Grapheme {
    type Output = Self;
    #[inline] fn add(self, rhs: Self) -> Self { Grapheme(self.0 + rhs.0) }
}
impl Monoid for Grapheme { }
impl Sub<usize> for Grapheme {
    type Output = Self;
    #[inline] fn sub(self, rhs: usize) -> Self { Grapheme(self.0 - rhs) }
}
impl Sub<Grapheme> for Grapheme {
    type Output = Self;
    #[inline] fn sub(self, rhs: Self) -> Self { Grapheme(self.0 - rhs.0) }
}

/// A metric for calculating indices in `Rope`s based on line numbering.
#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct Line(pub usize);
impl Default for Line {
    #[inline] fn default() -> Self { Line(0) }
}
impl Add<Line> for Line {
    type Output = Self;
    #[inline] fn add(self, rhs: Self) -> Self { Line(self.0 + rhs.0) }
}
impl Sub<Line> for Line {
    type Output = Self;
    #[inline] fn sub(self, rhs: Self) -> Self { Line(self.0 - rhs.0) }
}
impl convert::From<usize> for Line {
    #[inline] fn from(u: usize) -> Self { Line(u) }
}
impl convert::Into<usize> for Line {
    #[inline] fn into(self) -> usize { self.0 }
}
impl Monoid for Line { }
impl Add<usize> for Line {
    type Output = Self;
    #[inline] fn add(self, rhs: usize) -> Self { Line(self.0 + rhs) }
}
impl Sub<usize> for Line {
    type Output = Self;
    #[inline] fn sub(self, rhs: usize) -> Self { Line(self.0 - rhs) }
}

impl Monoid for usize { }
