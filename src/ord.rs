use std::cmp::Ordering;

use super::{Fraction, Sign};

impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> Ordering {
        // if it's zero lets say it's positive
        let a = if self.numer == 0 {
            &Sign::Positive
        } else {
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
        match (
            a == &Sign::Negative,
            // To compare fractions, cross-multiply and compare the results
            match (
                self.numer.checked_mul(other.denom.get()),
                other.numer.checked_mul(self.denom.get()),
            ) {
                (Some(self_cross), Some(other_cross)) => self_cross.cmp(&other_cross),
                (_, _) => (self.numer as u128 * other.denom.get() as u128)
                    .cmp(&(other.numer as u128 * self.denom.get() as u128)),
            },
        ) {
            (true, Ordering::Less) => Ordering::Greater,
            (true, Ordering::Equal) => Ordering::Equal,
            (true, Ordering::Greater) => Ordering::Less,
            (false, Ordering::Less) => Ordering::Less,
            (false, Ordering::Equal) => Ordering::Equal,
            (false, Ordering::Greater) => Ordering::Greater,
        }
    }
}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use crate::Fraction;
    #[test]
    fn one_cmp_two() {
        assert_eq!(Fraction::ONE.cmp(&Fraction::TWO), Ordering::Less)
    }
    #[test]
    fn two_cmp_one() {
        assert_eq!(Fraction::TWO.cmp(&Fraction::ONE), Ordering::Greater)
    }
    #[test]
    fn neg_one_cmp_one() {
        assert_eq!(Fraction::NEG_ONE.cmp(&Fraction::ONE), Ordering::Less)
    }
    #[test]
    fn one_cmp_neg_one() {
        assert_eq!(Fraction::ONE.cmp(&Fraction::NEG_ONE), Ordering::Greater)
    }
    #[test]
    fn one_cmp_neg_third() {
        assert_eq!(Fraction::ONE.cmp(&Fraction::NEG_THIRD), Ordering::Greater)
    }
    #[test]
    fn neg_third_cmp_one() {
        assert_eq!(Fraction::NEG_THIRD.cmp(&Fraction::ONE), Ordering::Less)
    }
    #[test]
    fn neg_one_cmp_neg_third() {
        assert_eq!(Fraction::NEG_ONE.cmp(&Fraction::NEG_THIRD), Ordering::Less)
    }
    #[test]
    fn neg_third_cmp_neg_one() {
        assert_eq!(
            Fraction::NEG_THIRD.cmp(&Fraction::NEG_ONE),
            Ordering::Greater
        )
    }
}
