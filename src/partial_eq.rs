use super::Fraction;

impl PartialEq for Fraction {
    fn eq(&self, other: &Self) -> bool {
        if self.sign == other.sign {
            // To compare fractions, cross-multiply and compare the results
            match (
                self.numer.checked_mul(other.denom.get()),
                other.numer.checked_mul(self.denom.get()),
            ) {
                (Some(self_cross), Some(other_cross)) => self_cross == other_cross,
                (_, _) => {
                    self.numer as u128 * other.denom.get() as u128
                        == other.numer as u128 * self.denom.get() as u128
                }
            }
        } else {
            self.numer == 0 && other.numer == 0
        }
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU64;

    use crate::Fraction;
    #[test]
    fn zeros() {
        let pos = Fraction {
            sign: crate::Sign::Positive,
            numer: 0,
            denom: NonZeroU64::new(1).unwrap(),
        };
        let neg = Fraction {
            sign: crate::Sign::Negative,
            numer: 0,
            denom: NonZeroU64::new(1).unwrap(),
        };
        assert_eq!(pos, neg)
    }
}
