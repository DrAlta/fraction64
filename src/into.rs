use crate::Fraction;

impl Into<f32> for &Fraction {
    fn into(self) -> f32 {
        let x = self.numer as f32 / self.denom.get() as f32;
        match self.sign {
            crate::Sign::Positive => x,
            crate::Sign::Negative => -x,
        }
    }
}
impl Into<f32> for Fraction {
    fn into(self) -> f32 {
        (&self).into()
    }
}


impl Into<f64> for &Fraction {
    fn into(self) -> f64 {
        let x = self.numer as f64 / self.denom.get() as f64;
        match self.sign {
            crate::Sign::Positive => x,
            crate::Sign::Negative => -x,
        }
    }
}
impl Into<f64> for Fraction {
    fn into(self) -> f64 {
    (&self).into()
    }
}