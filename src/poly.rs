use std::{
    iter,
    ops::{Add, Neg, Sub},
};

use crate::ff::FiniteField;

pub struct Poly<T: FiniteField> {
    coefficients: Vec<T>,
}

impl<T: FiniteField> Neg for Poly<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output {
            coefficients: self
                .coefficients
                .into_iter()
                .map(|coeff| coeff.neg())
                .collect(),
        }
    }
}

impl<T: FiniteField> Add for Poly<T> {
    type Output = Poly<T>;

    fn add(self, rhs: Self) -> Self::Output {
        let len = self.coefficients.len().max(rhs.coefficients.len());
        let l = self
            .coefficients
            .into_iter()
            .chain(iter::repeat(T::ZERO))
            .take(len);
        let r = rhs
            .coefficients
            .into_iter()
            .chain(iter::repeat(T::ZERO))
            .take(len);

        Self::Output {
            coefficients: l.zip(r).map(|(a, b)| a + b).collect(),
        }
    }
}

impl<T: FiniteField> Sub for Poly<T> {
    type Output = Poly<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        let len = self.coefficients.len().max(rhs.coefficients.len());
        let l = self
            .coefficients
            .into_iter()
            .chain(iter::repeat(T::ZERO))
            .take(len);
        let r = rhs
            .coefficients
            .into_iter()
            .chain(iter::repeat(T::ZERO))
            .take(len);

        Self::Output {
            coefficients: l.zip(r).map(|(a, b)| a - b).collect(),
        }
    }
}

impl<T: FiniteField> Poly<T> {
    pub fn degree(&self) -> usize {
        let mut max_index = usize::MAX;

        for (i, coeff) in self.coefficients.iter().enumerate() {
            if coeff != &T::ZERO {
                max_index = i;
            }
        }

        max_index
    }
}
