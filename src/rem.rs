use super::{Fraction, Sign};

impl Rem for Fraction {
    type Output = Fraction64;

    fn rem(self, other: Self) -> Self {
        // Calculate the remainder by finding the remainder of the division of numerators
        let remainder = self.numer % other.numer;

        Fraction64 {
            sign: self.sign,
            numer: remainder,
            denom: self.denom,
        }
        .reduce()
    }
}