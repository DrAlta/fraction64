use std::{ops::Mul, num::NonZeroU64};

use super::{Sign, Fraction};

impl Mul for &Fraction {
    type Output = Fraction;

    fn mul(self, other: Self) -> Self::Output {
        let new_sign = match (self.sign.clone(), other.sign.clone()) {
            (Sign::Positive, Sign::Positive) | (Sign::Negative, Sign::Negative) => Sign::Positive,
            _ => Sign::Negative,
        };

        match (
            self.numer.checked_mul(other.numer), 
            self.denom.checked_mul(other.denom)
        ) {
            (Some(numer), Some(denom)) => {
                Fraction {
                    sign: new_sign,
                    numer,
                    denom,
                }.reduce_consuming()
            },
            (_,_) => {
                let (numer, denom) = super::convert_fraction(self.numer as u128 * other.numer as u128, self.denom.get() as u128 * other.denom.get() as u128);
                Fraction {
                    sign: new_sign,
                    numer,
                    denom: NonZeroU64::new(denom).unwrap(),
                }.reduce_consuming()
            },
        }

    }
}

impl Mul for Fraction {
    type Output = Fraction;

    fn mul(self, other: Self) -> Self::Output {
        &self * &other
    }
}
impl Mul<&Fraction> for Fraction {
    type Output = Fraction;

    fn mul(self, other: &Fraction) -> Self::Output {
        &self * other
    }
}
impl Mul<Fraction> for &Fraction {
    type Output = Fraction;

    fn mul(self, other: Fraction) -> Self::Output {
        self * &other
    }
}