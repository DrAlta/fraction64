use super::{Fraction, Sign};
impl std::ops::Neg for Fraction {
    type Output = Fraction;

    fn neg(self) -> Self::Output {
        Self {
            sign: match self.sign {
                Sign::Positive => Sign::Negative,
                Sign::Negative => Sign::Positive,
            },
            numer: self.numer,
            denom: self.denom,
        }
    }
}