use std::ops::Add;

use starky_ff::FiniteField;

pub struct KoalaBearField(u32);

const PRIME: u32 = 0x7f000001;

impl Add for KoalaBearField {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0.wrapping_add(rhs.0) % PRIME)
    }
}

impl FiniteField for KoalaBearField {
    const ONE: Self = Self(1);
    const ZERO: Self = Self(0);
}
