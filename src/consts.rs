use std::num::NonZeroU64;

use super::{Fraction, Sign};
#[allow(dead_code)]// Keep
impl Fraction {// 18_446_744_073_709_551_615
    pub const ONE: Self = unsafe {Fraction{sign: Sign::Positive, numer: 1_u64, denom: NonZeroU64::new_unchecked(1_u64)}};
    pub const ZERO: Self = unsafe {Fraction{sign: Sign::Positive, numer: 0_u64, denom: NonZeroU64::new_unchecked(1_u64)}};
    
    pub const E: Self = unsafe {Fraction{sign: Sign::Positive, numer: 5_739_439_214_861_417_731_u64, denom: NonZeroU64::new_unchecked(2_111_421_691_000_680_031_u64)}};
    pub const FRAC_2_PI: Self = Self::TAU;
    pub const FRAC_PI_2: Self = unsafe {Fraction{sign: Sign::Positive, numer: 573204, denom:  NonZeroU64::new_unchecked(364913)}};
    pub const FRAC_PI_4: Self = unsafe {Fraction{sign: Sign::Positive, numer: 286602, denom:  NonZeroU64::new_unchecked(364913)}};
    pub const FRAC_PI_6: Self = unsafe {Fraction{sign: Sign::Positive, numer: 573204, denom:  NonZeroU64::new_unchecked(1094739)}};
    pub const FRAC_PI_8: Self = unsafe {Fraction{sign: Sign::Positive, numer: 1393301, denom:  NonZeroU64::new_unchecked(364913)}};
    pub const PI: Self = unsafe {Fraction{sign: Sign::Positive, numer: 11_146_408, denom:  NonZeroU64::new_unchecked(364913)}};
    pub const SQRT_2: Self = unsafe {Fraction{sign: Sign::Positive, numer: 367_296_043_199, denom:  NonZeroU64::new_unchecked(259_717_522_849)}};
    pub const TAU: Self = unsafe {Fraction{sign: Sign::Positive, numer: 2292816, denom: NonZeroU64::new_unchecked(364913)}};
}