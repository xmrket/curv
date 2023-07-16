use std::marker::PhantomData;
use std::ops::Deref;

use crate::elliptic::curves::traits::*;

use super::Point;

/// Elliptic curve generator
///
/// Holds internally a static reference on curve generator. Can be used in arithmetic interchangeably
/// as [&Point\<E\>](Point<E>).
///
/// You can convert the generator into `Point<E>` and `&'static Point<E>` using
/// [`to_point`](Self::to_point) and [`as_point`](Self::as_point)
/// methods respectively.
///
/// ## Example
///
/// ```rust
/// # use curv::elliptic::curves::{Point, Scalar, Ed25519};
/// let s = Scalar::<Ed25519>::random();
/// let g = Point::<Ed25519>::generator();
/// let result: Point<Ed25519> = s * g;
/// ```
///
/// ## Performance
///
/// Generator multiplication is often more efficient than regular point multiplication, so avoid
/// converting generator into the `Point<E>` as long as it's possible:
///
/// ```rust
/// # use curv::elliptic::curves::{Point, Scalar, Ed25519, Generator};
/// let s: Scalar<Ed25519> = Scalar::random();
/// // Generator multiplication:
/// let g: Generator<Ed25519> = Point::generator();
/// let p1: Point<Ed25519> = g * &s;
/// // Point multiplication:
/// let g: Point<Ed25519> = g.to_point();
/// let p2: Point<Ed25519> = g * &s;
/// // Result will be the same, but generator multiplication is usually faster
/// assert_eq!(p1, p2);
/// ```
pub struct Generator<E: Curve> {
    _ph: PhantomData<&'static E::Point>,
}

impl<E: Curve> Default for Generator<E> {
    fn default() -> Self {
        Self { _ph: PhantomData }
    }
}

impl<E: Curve> Generator<E> {
    /// Clones generator point, returns `Point<E>`
    pub fn to_point(self) -> Point<E> {
        Point::from(self)
    }

    /// Converts generator into `&'static Point<E>`
    pub fn as_point(self) -> &'static Point<E> {
        // Safety: generator point expected to have correct order
        unsafe { Point::from_raw_ref_unchecked(self.as_raw()) }
    }

    /// Returns a reference to low-level point implementation
    pub fn as_raw(self) -> &'static E::Point {
        E::Point::generator()
    }
}

impl<E: Curve> Clone for Generator<E> {
    fn clone(&self) -> Self {
        Self { _ph: PhantomData }
    }
}

impl<E: Curve> Copy for Generator<E> {}

impl<E: Curve> Deref for Generator<E> {
    type Target = Point<E>;
    fn deref(&self) -> &Point<E> {
        self.as_point()
    }
}
