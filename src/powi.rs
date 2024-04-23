use crate::{u128_to_fraction, Fraction};

impl Fraction{
    pub fn powi(&self, n: i32) -> Self {
        if n == 0 {
            return Fraction::ONE;
        }

        let (numer, denom) = if n < 0 {
            (
                (self.denom.get() as u128).checked_pow(n.unsigned_abs()).expect("the new numer over flowed"),
                (self.numer as u128).checked_pow(n.unsigned_abs()).expect("the new denom over flowed"),
            )
        } else {
            (
                (self.numer as u128).checked_pow(n.unsigned_abs()).expect("the new numer over flowed"),
                (self.denom.get() as u128).checked_pow(n.unsigned_abs()).expect("the new denom over flowed"),
            )
        };
        u128_to_fraction(crate::Sign::Positive, numer, denom)
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU64;

    use crate::Fraction;
    #[test]
    fn three_to_the_three() {
        let value: Fraction = Fraction::THREE.powi(3); 
        assert_eq!(
            value,
            Fraction::new(27, NonZeroU64::new(1).unwrap())
        );
    }
    #[test]
    fn three_to_the_neg_two() {
        let value: Fraction = Fraction::THREE.powi(-2); 
        assert_eq!(
            value,
            Fraction::new(1, NonZeroU64::new(9).unwrap())
        );
    }
}