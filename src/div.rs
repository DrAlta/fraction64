use std::{ops::Div, num::NonZeroU64};
use super::{Sign, Fraction};

impl Div for &Fraction {
    type Output = Fraction;

    fn div(self, other: Self) -> Self::Output {
        let new_sign = match (self.sign.clone(), other.sign.clone()) {
            (Sign::Positive, Sign::Positive) | (Sign::Negative, Sign::Negative) => Sign::Positive,
            _ => Sign::Negative,
        };
        match (
            self.numer.checked_mul(other.denom.get()), 
            self.denom.get().checked_mul(other.numer)
        ) {
            (Some(numer), Some(denom)) => {
                return Fraction {
                    sign: new_sign,
                    numer,
                    denom: NonZeroU64::new(denom).unwrap(),
                }.reduce_consuming();
            },
            (_,_) => {
                let (numer, denom) = super::convert_fraction(
                    self.numer as u128 * other.denom.get() as u128,
                    self.denom.get() as u128 * other.numer as u128
                );
                return Fraction {
                    sign: new_sign,
                    numer,
                    denom: NonZeroU64::new(denom).unwrap(),
                }.reduce_consuming();
            }
        }
    }
}

impl Div for Fraction {
    type Output = Fraction;
    
    fn div(self, rhs: Self) -> Self::Output {
         &self / &rhs
    }
}
impl Div<&Fraction> for Fraction {
    type Output = Fraction;
    
    fn div(self, rhs: &Fraction) -> Self::Output {
         &self / rhs
    }
}
impl Div<Fraction> for &Fraction {
    type Output = Fraction;
    
    fn div(self, rhs: Fraction) -> Self::Output {
         self / &rhs
    }
}