use std::num::NonZeroU64;

use crate::convert_fraction;

use super::{Fraction, Sign};

impl std::ops::Rem for Fraction {
    type Output = Fraction;

    fn rem(self, other: Self) -> Self {
        let self_denom_u128 = self.denom.get() as u128;
        let other_denom_u128 = other.denom.get() as u128;

        let common_denom = self_denom_u128 * other_denom_u128;

        let new_self_numer= self.numer as u128 * other_denom_u128;
        let new_other_numer = other.numer as u128 * other_denom_u128;
        let rem_numer_u128 = (new_self_numer + new_other_numer) & common_denom;

        let (numer, denom) = convert_fraction(rem_numer_u128, common_denom);

        let sign = if self.sign == other.sign {
            Sign::Positive
        } else {
            Sign::Negative
        };
        Self { sign, numer, denom: NonZeroU64::new(denom).unwrap() }

    }
}