use std::cmp::Ordering;

use super::{Fraction, Sign};

impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> Ordering {
        // if it's zero lets say it's positive
        let a = if self.numer == 0 {
            &Sign::Positive
        } else{
            &self.sign
        };
        let b = if other.numer == 0 {
            &Sign::Positive
        } else {
            &other.sign
        };

        if a != b {
            return match (a, b) {
                (Sign::Positive, Sign::Negative) => Ordering::Greater,
                (Sign::Negative, Sign::Positive) => Ordering::Less,
                _ => unreachable!(), // Should not happen if fractions are reduced
            };
        }
        // To compare fractions, cross-multiply and compare the results
        match(self.numer.checked_mul(other.denom.get()), other.numer.checked_mul(self.denom.get())){
            (Some(self_cross), Some(other_cross)) => self_cross.cmp(&other_cross),
            (_,_) => {
                (self.numer as u128 * other.denom.get() as u128).cmp(&(other.numer as u128 * self.denom.get() as u128))
            }
        }
   }
}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
