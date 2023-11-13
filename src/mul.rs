use std::{ops::Mul, num::NonZeroU64};

use super::{Sign, Fraction};

impl Mul for Fraction {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let new_sign = match (self.sign, other.sign) {
            (Sign::Positive, Sign::Positive) | (Sign::Negative, Sign::Negative) => Sign::Positive,
            _ => Sign::Negative,
        };

        match (
            self.numer.checked_mul(other.numer), 
            self.denom.checked_mul(other.denom)
        ) {
            (Some(numer), Some(denom)) => {
                Self {
                    sign: new_sign,
                    numer,
                    denom,
                }.simplify_consuming()
            },
            (_,_) => {
                let (numer, denom) = super::convert_fraction(self.numer as u128 * other.numer as u128, self.denom.get() as u128 * other.denom.get() as u128);
                Self {
                    sign: new_sign,
                    numer,
                    denom: NonZeroU64::new(denom).unwrap(),
                }.simplify_consuming()
            },
        }

    }
}
