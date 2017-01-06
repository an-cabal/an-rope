use std::convert;
use std::ops::{Add, Sub};
use std::default::Default;


/// The class of monoids
///
/// The class of monoids (types with an accumulative binary operation that has
/// an identity).
///
/// Technically, `Add<Self, Output=Self>` is standing in for "semigroup" here,
/// while `Default` is standing in for "identity".
///
/// An instance _M_ should satisfy the following laws:
///
///  + _x_`.add(`_M_`::default())` = _x_
///  + _M_`::default().add(`_x_`)` = _x_
///  + _x_`.add(`_y_`.add(`_z_`))` = _z_`.add(`_x_`.add(`_y_`))`
///  + _M_`::accumulate(`_a_`)` = _a_`.fold(`_M_`::default,`_M_`::sum)`
///
pub trait Monoid: Add<Self, Output=Self> + Default + Sized {
    #[inline]
    fn accumulate<F>(xs: F) -> Self
    where F: Iterator<Item=Self>
        , Self: Sized {
        xs.fold(Self::default(), Self::add)
    }
}

pub trait Measured<M: Metric> {
    /// Apply `Metric` to `Self`
    ///
    /// Although we aren't currently enforcing this, `measure`ing a `Node` with
    /// two children should produce the same result as `measure`ing both
    /// children and `Monoid::sum`ming the result. That is to say, `measure`
    /// should be a [_monoid homomorphism_]
    /// (https://en.wikipedia.org/wiki/Monoid#Monoid_homomorphisms).
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

/// A monoid that can be applied to a `Node` as a measurement
pub trait Metric: Monoid + Eq + Add<usize, Output=Self>
                         + Sub<usize, Output=Self>
                         + Sub<Self, Output=Self>
                         + Eq + Ord
                         + Sized
                         + convert::Into<usize>
                         + Copy{

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

#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct Grapheme(usize);
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

#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct Line(usize);
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
