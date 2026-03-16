use std::ops::{Add, Div, Mul, Neg, Sub};

use starky_ff::FiniteField;
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct KoalaBearField(u32);

const PRIME: u32 = 0x7f000001;

impl Add for KoalaBearField {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(((self.0 as u64).wrapping_add(rhs.0 as u64) % PRIME as u64) as u32)
    }
}

impl Sub for KoalaBearField {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(((PRIME as u64 + self.0 as u64 - rhs.0 as u64) % PRIME as u64) as u32)
    }
}

impl Mul for KoalaBearField {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(((self.0 as u64 * rhs.0 as u64) % PRIME as u64) as u32)
    }
}

impl Neg for KoalaBearField {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(((PRIME as u64 - self.0 as u64) % PRIME as u64) as u32)
    }
}

impl ConditionallySelectable for KoalaBearField {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Self(u32::conditional_select(&a.0, &b.0, choice))
    }
}

impl ConstantTimeEq for KoalaBearField {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.0.ct_eq(&other.0)
    }
}

impl PartialEq for KoalaBearField {
    fn eq(&self, other: &Self) -> bool {
        self.ct_eq(other).into()
    }
}

impl Eq for KoalaBearField {}

fn xgcd(x: u32, y: u32) -> (u32, u32, u32) {
    let (mut old_r, mut r) = (x, y);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);

    while r != 0 {
        let quotient = old_r / r;
        (old_r, r) = (r, old_r - quotient * r);
        (old_s, s) = (s, old_s - quotient * s);
        (old_t, t) = (t, old_t - quotient * t);
    }

    (old_s, old_t, old_r)
}

impl FiniteField for KoalaBearField {
    const ONE: Self = Self(1);
    const ZERO: Self = Self(0);

    fn inverse(&self) -> CtOption<Self> {
        let (a, _, _) = xgcd(self.0, PRIME);

        CtOption::new(KoalaBearField(a), self.ct_ne(&Self::ZERO))
    }
}
