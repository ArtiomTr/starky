pub mod koala_bear;

use core::ops::{Add, Mul, Neg, Sub};

use subtle::{ConditionallySelectable, ConstantTimeEq, CtOption};

pub trait FiniteField:
    Sized
    + Clone
    + Eq
    + Neg<Output = Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Send
    + Sync
    + ConstantTimeEq
    + ConditionallySelectable
{
    /// Additive identity.
    const ZERO: Self;
    /// Multiplicative identity.
    const ONE: Self;
    /// Multiplicative generator.
    const GENERATOR: Self;

    /// Compute multiplicative inverse of this element.
    ///
    /// Returns None if it is zero (additive identity).
    fn inverse(&self) -> CtOption<Self>;
}
