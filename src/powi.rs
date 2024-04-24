use crate::{u128_to_fraction, Fraction};

impl Fraction{
    pub fn powi(&self, p: i32) -> Self {
        if p == 0 {
            return Fraction::ONE;
        }

        let n = (self.numer as u128).checked_pow(p.unsigned_abs()).expect("the numen over flowed");
        let d = (self.denom.get() as u128).checked_pow(p.unsigned_abs()).expect("the demon over flowed");

        let (numer, denom) = if p < 0 {
            ( d, n, )
        } else {
            ( n, d,)
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