use super::{Fraction, Sign};

impl Fraction {
    pub fn abs(self) -> Self {
        Self { sign: Sign::Positive, numer: self.numer, denom: self.denom }
    }
}