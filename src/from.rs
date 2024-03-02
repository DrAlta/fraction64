use std::num::NonZeroU64;

use super::{Fraction, Sign};
///////
/// unsigned intergers
impl From<u8> for Fraction {
    fn from(item: u8) -> Self {
        Self {sign: Sign::Positive, numer: item.into(), denom: NonZeroU64::new(1_u64).unwrap()}
    }
}
impl From<u16> for Fraction {
    fn from(item: u16) -> Self {
        Self {sign: Sign::Positive, numer: item.into(), denom: NonZeroU64::new(1_u64).unwrap()}
    }
}
impl From<u32> for Fraction {
    fn from(item: u32) -> Self {
        Self {sign: Sign::Positive, numer: item.into(), denom: NonZeroU64::new(1_u64).unwrap()}
    }
}
impl From<u64> for Fraction {
    fn from(item: u64) -> Self {
        Self {sign: Sign::Positive, numer: item.into(), denom: NonZeroU64::new(1_u64).unwrap()}
    }
}
impl From<(i64, u64)> for Fraction {
    fn from(item: (i64, u64)) -> Self {
        let Some(denom) = NonZeroU64::new(item.1) else {
            // any amount of zeroths in zero;
            return Self::ZERO
        };
        let sign = if item.0.is_positive(){
            Sign::Positive
        } else {
            Sign::Negative
        };

        let numer = item.0.unsigned_abs();
        Self {sign, numer, denom}
    }
}


////////
/// Signed intergers
impl From<i8> for Fraction {
    fn from(item: i8) -> Self {
        let sign = if item < 0 {
            Sign::Negative
        } else {
            Sign::Positive
        };

        Self {sign, numer: item.abs() as u64, denom: NonZeroU64::new(1_u64).unwrap()}
    }
}
impl From<i16> for Fraction {
    fn from(item: i16) -> Self {
        let sign = if item < 0 {
            Sign::Negative
        } else {
            Sign::Positive
        };

        Self {sign, numer: item.abs() as u64, denom: NonZeroU64::new(1_u64).unwrap()}
    }
}
impl From<i32> for Fraction {
    fn from(item: i32) -> Self {
        let sign = if item < 0 {
            Sign::Negative
        } else {
            Sign::Positive
        };

        Self {sign, numer: item.abs() as u64, denom: NonZeroU64::new(1_u64).unwrap()}
    }
}
impl From<i64> for Fraction {
    fn from(item: i64) -> Self {
        let sign = if item.is_positive(){
            Sign::Positive
        } else {
            Sign::Negative
        };

        Self {sign, numer: item.unsigned_abs(), denom: NonZeroU64::new(1_u64).unwrap()}
    }
}

