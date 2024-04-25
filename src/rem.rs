use std::num::NonZeroU64;

use crate::convert_fraction;

use super::{Fraction, Sign};

impl std::ops::Rem for &Fraction {
    type Output = Fraction;

    fn rem(self, other: Self) -> Fraction {
        let self_denom_u128 = self.denom.get() as u128;
        let other_denom_u128 = other.denom.get() as u128;

        let common_denom = self_denom_u128 * other_denom_u128;

        let new_self_numer= self.numer as u128 * other_denom_u128;
        let new_other_numer = other.numer as u128 * self_denom_u128;
        let rem_numer_u128 = new_self_numer % new_other_numer;

        let (numer, denom) = convert_fraction(rem_numer_u128, common_denom);

        let sign = if self.sign == other.sign {
            Sign::Positive
        } else {
            Sign::Negative
        };
        Fraction { sign, numer, denom: NonZeroU64::new(denom).unwrap() }

    }
}

impl std::ops::Rem for Fraction {
    type Output = Fraction;

    fn rem(self, other: Self) -> Self {
        &self % &other
    }
}
impl std::ops::Rem<&Fraction> for Fraction {
    type Output = Fraction;

    fn rem(self, other: &Fraction) -> Self {
        &self % other
    }
}
impl std::ops::Rem<Fraction> for &Fraction {
    type Output = Fraction;

    fn rem(self, other: Fraction) -> Fraction {
        self % &other
    }
}

#[cfg(test)]
mod tests {

    use crate::Fraction;
    #[test]
    fn seven_mod_four(){
        assert_eq!(
            Fraction::from((7,1)) % Fraction::FOUR,
            Fraction::THREE
        )
    }
}