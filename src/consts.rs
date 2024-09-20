use std::num::NonZeroU64;

use super::{Fraction, Sign};
impl Fraction {// 18_446_744_073_709_551_615
    pub const NEG_TWO: Self = unsafe {Fraction{sign: Sign::Negative, numer: 2_u64, denom: NonZeroU64::new_unchecked(1_u64)}};
    pub const NEG_FOUR_THIRDS: Fraction = unsafe {Fraction::new_neg(4, std::num::NonZeroU64::new_unchecked( 3))};
    pub const NEG_ONE: Self = unsafe {Fraction{sign: Sign::Negative, numer: 1_u64, denom: NonZeroU64::new_unchecked(1_u64)}};
    pub const NEG_TWO_THIRDS: Fraction = unsafe {Fraction::new_neg(2, std::num::NonZeroU64::new_unchecked( 3))};
    pub const NEG_THIRD: Fraction = unsafe {Fraction::new_neg(1, std::num::NonZeroU64::new_unchecked( 3))};
    pub const ZERO: Self = unsafe {Fraction{sign: Sign::Positive, numer: 0_u64, denom: NonZeroU64::new_unchecked(1_u64)}};
    pub const FOURTH: Fraction = unsafe {Fraction::new(1, std::num::NonZeroU64::new_unchecked( 4))};
    pub const THIRD: Fraction = unsafe {Fraction::new(1, std::num::NonZeroU64::new_unchecked( 3))};
    pub const HALF: Self = unsafe {Fraction{sign: Sign::Positive, numer: 1_u64, denom: NonZeroU64::new_unchecked(2_u64)}};
    pub const TWO_THIRDS: Fraction = unsafe {Fraction::new(2, std::num::NonZeroU64::new_unchecked( 3))};
    pub const ONE: Self = unsafe {Fraction{sign: Sign::Positive, numer: 1_u64, denom: NonZeroU64::new_unchecked(1_u64)}};
    pub const TWO: Self = unsafe {Fraction{sign: Sign::Positive, numer: 2_u64, denom: NonZeroU64::new_unchecked(1_u64)}};
    pub const FOUR_THIRDS: Fraction = unsafe {Fraction::new(4, std::num::NonZeroU64::new_unchecked( 3))};
    pub const THREE: Self = unsafe {Fraction{sign: Sign::Positive, numer: 3_u64, denom: NonZeroU64::new_unchecked(1_u64)}};
    pub const FOUR: Self = unsafe {Fraction{sign: Sign::Positive, numer: 4_u64, denom: NonZeroU64::new_unchecked(1_u64)}};
    pub const SIX: Self = unsafe {Fraction{sign: Sign::Positive, numer: 6_u64, denom: NonZeroU64::new_unchecked(1_u64)}};
    
    pub const E: Self = unsafe {Fraction{sign: Sign::Positive, numer: 5_739_439_214_861_417_731_u64, denom: NonZeroU64::new_unchecked(2_111_421_691_000_680_031_u64)}};
    pub const FRAC_2_PI: Self = Self::TAU;
    pub const FRAC_PI_2: Self = unsafe {Fraction{sign: Sign::Positive, numer: 573204, denom:  NonZeroU64::new_unchecked(364913)}};
    pub const FRAC_PI_4: Self = unsafe {Fraction{sign: Sign::Positive, numer: 286602, denom:  NonZeroU64::new_unchecked(364913)}};
    pub const FRAC_PI_6: Self = unsafe {Fraction{sign: Sign::Positive, numer: 573204, denom:  NonZeroU64::new_unchecked(1094739)}};
    pub const FRAC_PI_8: Self = unsafe {Fraction{sign: Sign::Positive, numer: 1393301, denom:  NonZeroU64::new_unchecked(364913)}};
    pub const PI: Self = unsafe {Fraction{sign: Sign::Positive, numer: 11_146_408, denom:  NonZeroU64::new_unchecked(364913)}};
    pub const SQRT_2: Self = unsafe {Fraction{sign: Sign::Positive, numer: 367_296_043_199, denom:  NonZeroU64::new_unchecked(259_717_522_849)}};
    pub const TAU: Self = unsafe {Fraction{sign: Sign::Positive, numer: 2292816, denom: NonZeroU64::new_unchecked(364913)}};

    pub const MAX: Self = unsafe {Fraction{sign: Sign::Positive, numer: std::u64::MAX, denom: NonZeroU64::new_unchecked(1)}};
    pub const MIN: Self = unsafe {Fraction{sign: Sign::Negative, numer: std::u64::MAX, denom: NonZeroU64::new_unchecked(1)}};

}