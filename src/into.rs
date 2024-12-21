use crate::Fraction;

impl Into<u8> for &Fraction {
    fn into(self) -> u8 {
        assert_eq!(self.sign, crate::Sign::Positive);
        (self.numer / self.denom.get()) as u8
    }
}

impl Into<u16> for &Fraction {
    fn into(self) -> u16 {
        assert_eq!(self.sign, crate::Sign::Positive);
        (self.numer / self.denom.get()) as u16
    }
}
impl Into<u32> for &Fraction {
    fn into(self) -> u32 {
        assert_eq!(self.sign, crate::Sign::Positive);
        (self.numer / self.denom.get()) as u32
    }
}

impl Into<u64> for &Fraction {
    fn into(self) -> u64 {
        assert_eq!(self.sign, crate::Sign::Positive);
        (self.numer / self.denom.get()) as u64
    }
}

impl Into<u128> for &Fraction {
    fn into(self) -> u128 {
        assert_eq!(self.sign, crate::Sign::Positive);
        (self.numer / self.denom.get()) as u128
    }
}

impl Into<i8> for &Fraction {
    fn into(self) -> i8 {
        let x = (self.numer / self.denom.get()) as i8;
        match self.sign {
            crate::Sign::Positive => x,
            crate::Sign::Negative => -x,
        }
    }
}
impl Into<i16> for &Fraction {
    fn into(self) -> i16 {
        let x = (self.numer / self.denom.get()) as i16;
        match self.sign {
            crate::Sign::Positive => x,
            crate::Sign::Negative => -x,
        }
    }
}
impl Into<i32> for &Fraction {
    fn into(self) -> i32 {
        let x = (self.numer / self.denom.get()) as i32;
        match self.sign {
            crate::Sign::Positive => x,
            crate::Sign::Negative => -x,
        }
    }
}

impl Into<i64> for &Fraction {
    fn into(self) -> i64 {
        let x = (self.numer / self.denom.get()) as i64;
        match self.sign {
            crate::Sign::Positive => x,
            crate::Sign::Negative => -x,
        }
    }
}
impl Into<i128> for &Fraction {
    fn into(self) -> i128 {
        let x = (self.numer / self.denom.get()) as i128;
        match self.sign {
            crate::Sign::Positive => x,
            crate::Sign::Negative => -x,
        }
    }
}

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
