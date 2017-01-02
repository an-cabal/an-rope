use std::convert;
use std::ops::{Add, Sub};
use internals::Node;

/// The class of monoids
///
/// The class of monoids (types with an sumiative binary operation that has
/// an identity).
///
/// An instance _M_ should satisfy the following laws:
///
///  + _x_`.sum(`_M_`::identity())` = _x_
///  + _M_`::identity().sum(`_x_`)` = _x_
///  + _x_`.sum(`_y_`.sum(`_z_`))` = _z_`.sum(`_x_`.sum(`_y_`))`
///  + _M_`::concat(`_a_`)` = _a_`.fold(`_M_`::identity,`_M_`::sum)`
///
pub trait Monoid: Sized {
    /// The identity operation of the monoid
    //  TODO: can this be represented by std::default::Default instead?
    fn identity() -> Self;

    /// The sum operation of the monoid
    //  TODO: can this be represented by std::ops::Add instead?
    fn sum(self, other: Self) -> Self;

    #[inline]
    fn concat<F>(xs: F) -> Self
    where F: Iterator<Item=Self>
        , Self: Sized {
        xs.fold(Self::identity(), Self::sum)
    }
}

/// A monoid that can be applied to a `Node` as a measurement
pub trait Metric: Monoid + Eq + Add<usize, Output=Self>
                              + Sub<usize, Output=Self>
                              + Sized {
    /// Apply this `Metric` to the given `Node`
    fn measure(node: &Node) -> Self;

    /// Convert the `Metric` into a byte index into the given `Node`
    ///
    /// # Returns
    /// - `Some` with the byte index of the beginning of the `n`th  element
    ///    in `node` measured by this `Metric`, if there is an `n`th element
    /// - `None` if there is no `n`th element in `node`
    fn to_byte_index(&self, node: &Node) -> Option<usize>;

    /// Returns the byte index of the next element of this metric in `Node`
    #[inline] fn next(self, node: &Node) -> Option<usize> {
        (self + 1).to_byte_index(node)
    }

    /// Returns the byte index of the previous element of this metric in `Node`
    #[inline] fn back(self, node: &Node) -> Option<usize> {
        (self - 1).to_byte_index(node)
    }

    /// Returns true if index `i` in `node` is a boundary along this `Metric`
    // TODO: should this be a method on `Node`s instead?
    fn is_boundary(node: &Node, i: usize) -> bool;
}

#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct Grapheme(usize);
impl Monoid for Grapheme {
    #[inline] fn identity() -> Self { Grapheme(0) }
    #[inline] fn sum(self, other: Self) -> Self { Grapheme(self.0 + other.0) }
}

impl convert::From<usize> for Grapheme {
    #[inline] fn from(u: usize) -> Self { Grapheme(u) }
}

impl Add<usize> for Grapheme {
    type Output = Self;
    #[inline] fn add(self, rhs: usize) -> Self { Grapheme(self.0 + rhs) }
}
impl Sub<usize> for Grapheme {
    type Output = Self;
    #[inline] fn sub(self, rhs: usize) -> Self { Grapheme(self.0 - rhs) }
}

#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct Line(usize);
impl Monoid for Line {
    #[inline] fn identity() -> Self { Line(0) }
    #[inline] fn sum(self, other: Self) -> Self { Line(self.0 + other.0) }
}

impl convert::From<usize> for Line {
    #[inline] fn from(u: usize) -> Self { Line(u) }
}
impl Add<usize> for Line {
    type Output = Self;
    #[inline] fn add(self, rhs: usize) -> Self { Line(self.0 + rhs) }
}
impl Sub<usize> for Line {
    type Output = Self;
    #[inline] fn sub(self, rhs: usize) -> Self { Line(self.0 - rhs) }
}
