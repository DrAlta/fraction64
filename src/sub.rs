use std::ops::Sub;

use super::{Fraction, Sign};

impl Sub for Fraction {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        // To subtract fractions, negate the second fraction and add it
        let neg_other = Fraction {
            sign: match other.sign {
                Sign::Positive => Sign::Negative,
                Sign::Negative => Sign::Positive,
            },
            numer: other.numer,
            denom: other.denom,
        };

        self + neg_other
    }
}