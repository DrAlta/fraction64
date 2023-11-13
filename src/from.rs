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
        let sign = if item < 0 {
            Sign::Negative
        } else {
            Sign::Positive
        };

        Self {sign, numer: item.abs() as u64, denom: NonZeroU64::new(1_u64).unwrap()}
    }
}