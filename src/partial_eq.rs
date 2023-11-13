use super::Fraction;

impl PartialEq for Fraction {
    fn eq(&self, other: &Self) -> bool {
        if self.sign == other.sign {
            // To compare fractions, cross-multiply and compare the results
            match(self.numer.checked_mul(other.denom.get()), other.numer.checked_mul(self.denom.get())){
                (Some(self_cross), Some(other_cross)) => self_cross == other_cross,
                (_,_) => {
                    self.numer as u128 * other.denom.get() as u128 == other.numer as u128 * self.denom.get() as u128
                }
            }
        } else {
            false
        }
    }
}