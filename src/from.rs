use std::num::NonZeroU64;

use crate::{Fraction, Sign};

impl From<&Fraction> for Fraction {
    fn from(value: &Fraction) -> Self {
        value.clone()
    }
}

///////
/// unsigned intergers
impl From<u8> for Fraction {
    fn from(item: u8) -> Self {
        Self {
            sign: Sign::Positive,
            numer: item.into(),
            denom: NonZeroU64::new(1_u64).unwrap(),
        }
    }
}
impl From<u16> for Fraction {
    fn from(item: u16) -> Self {
        Self {
            sign: Sign::Positive,
            numer: item.into(),
            denom: NonZeroU64::new(1_u64).unwrap(),
        }
    }
}
impl From<u32> for Fraction {
    fn from(item: u32) -> Self {
        Self {
            sign: Sign::Positive,
            numer: item.into(),
            denom: NonZeroU64::new(1_u64).unwrap(),
        }
    }
}
impl From<u64> for Fraction {
    fn from(item: u64) -> Self {
        Self {
            sign: Sign::Positive,
            numer: item.into(),
            denom: NonZeroU64::new(1_u64).unwrap(),
        }
    }
}
impl From<(i64, u64)> for Fraction {
    fn from(item: (i64, u64)) -> Self {
        let Some(denom) = NonZeroU64::new(item.1) else {
            // any amount of zeroths in zero;
            return Self::ZERO;
        };
        let sign = if item.0.is_positive() {
            Sign::Positive
        } else {
            Sign::Negative
        };

        let numer = item.0.unsigned_abs();
        Self { sign, numer, denom }
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

        Self {
            sign,
            numer: item.abs() as u64,
            denom: NonZeroU64::new(1_u64).unwrap(),
        }
    }
}
impl From<i16> for Fraction {
    fn from(item: i16) -> Self {
        let sign = if item < 0 {
            Sign::Negative
        } else {
            Sign::Positive
        };

        Self {
            sign,
            numer: item.abs() as u64,
            denom: NonZeroU64::new(1_u64).unwrap(),
        }
    }
}
impl From<i32> for Fraction {
    fn from(item: i32) -> Self {
        let sign = if item < 0 {
            Sign::Negative
        } else {
            Sign::Positive
        };

        Self {
            sign,
            numer: item.abs() as u64,
            denom: NonZeroU64::new(1_u64).unwrap(),
        }
    }
}
impl From<i64> for Fraction {
    fn from(item: i64) -> Self {
        let sign = if item.is_positive() {
            Sign::Positive
        } else {
            Sign::Negative
        };

        Self {
            sign,
            numer: item.unsigned_abs(),
            denom: NonZeroU64::new(1_u64).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU64;

    use crate::Fraction;
    #[test]
    fn three_u8_test() {
        assert_eq!(
            Into::<Fraction>::into(3_u8),
            Fraction::new(3, NonZeroU64::new(1_u64).unwrap())
        )
    }
    #[test]
    fn three_u16_test() {
        assert_eq!(
            Into::<Fraction>::into(3_u16),
            Fraction::new(3, NonZeroU64::new(1_u64).unwrap())
        )
    }
    #[test]
    fn three_u32_test() {
        assert_eq!(
            Into::<Fraction>::into(3_u32),
            Fraction::new(3, NonZeroU64::new(1_u64).unwrap())
        )
    }
    #[test]
    fn three_u64_test() {
        assert_eq!(
            Into::<Fraction>::into(3_u64),
            Fraction::new(3, NonZeroU64::new(1_u64).unwrap())
        )
    }

    #[test]
    fn three_i8_test() {
        assert_eq!(
            Into::<Fraction>::into(3_i8),
            Fraction::new(3, NonZeroU64::new(1_u64).unwrap())
        )
    }
    #[test]
    fn three_i16_test() {
        assert_eq!(
            Into::<Fraction>::into(3_i16),
            Fraction::new(3, NonZeroU64::new(1_u64).unwrap())
        )
    }
    #[test]
    fn three_i32_test() {
        assert_eq!(
            Into::<Fraction>::into(3_i32),
            Fraction::new(3, NonZeroU64::new(1_u64).unwrap())
        )
    }
    #[test]
    fn three_i64_test() {
        assert_eq!(
            Into::<Fraction>::into(3_i64),
            Fraction::new(3, NonZeroU64::new(1_u64).unwrap())
        )
    }

    #[test]
    fn neg_three_i8_test() {
        assert_eq!(
            Into::<Fraction>::into(-3_i8),
            Fraction::new_neg(3, NonZeroU64::new(1_u64).unwrap())
        )
    }
    #[test]
    fn neg_three_i16_test() {
        assert_eq!(
            Into::<Fraction>::into(-3_i16),
            Fraction::new_neg(3, NonZeroU64::new(1_u64).unwrap())
        )
    }
    #[test]
    fn neg_three_i32_test() {
        assert_eq!(
            Into::<Fraction>::into(-3_i32),
            Fraction::new_neg(3, NonZeroU64::new(1_u64).unwrap())
        )
    }
    #[test]
    fn neg_three_i64_test() {
        assert_eq!(
            Into::<Fraction>::into(-3_i64),
            Fraction::new_neg(3, NonZeroU64::new(1_u64).unwrap())
        )
    }
}
