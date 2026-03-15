use core::ops::{Add, Mul, Neg, Sub};

use subtle::{ConditionallySelectable, ConstantTimeEq};

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
}
