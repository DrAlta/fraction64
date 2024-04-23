use num_integer::Roots;

use crate::{u128_to_fraction, Fraction, Sign};

impl Fraction {
    pub fn pow(&self, n: &Self) -> Self {
        //nth_root is losses information so we do it after raiseing to the power.
        let nu = (self.numer as u128)
            .pow(n.numer as u32)
            .nth_root(n.denom.get() as u32);
        let de = (self.denom.get() as u128)
            .pow(n.numer as u32)
            .nth_root(n.denom.get() as u32);

        let (numer, denom) = if n.sign == Sign::Positive {
            (nu, de)
        } else {
            (de, nu)
        };
        u128_to_fraction( self.sign.clone(), numer, denom)
    }
}


#[cfg(test)]
mod tests {
    use std::num::NonZeroU64;

    use crate::Fraction;
    #[test]
    fn three_to_the_three() {
        let value: Fraction = Fraction::THREE.pow(&Fraction::THREE); 
        assert_eq!(
            value,
            Fraction::new(27, NonZeroU64::new(1).unwrap())
        );
    }
    #[test]
    fn three_to_the_neg_two() {
        let value: Fraction = Fraction::THREE.pow(&Fraction::NEG_TWO); 
        assert_eq!(
            value,
            Fraction::new(1, NonZeroU64::new(9).unwrap())
        );
    }
}