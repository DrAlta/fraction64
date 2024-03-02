//! based on https://yal.cc/fast-atan2/  which is based on 
//! https://nghiaho.com/?p=997 which is based on 
//! “Efficient approximations for the arctangent function”, 
//! Rajan, S. Sichun Wang Inkol, R. Joyal, A., May 2006
use std::num::NonZeroU64;

use super::Fraction;

impl Fraction {
    pub fn atan(&self) -> Self {
        let x = self;
        let frac_pi_4: Fraction = Fraction::new(286602_u64, NonZeroU64::new(364913_u64).unwrap());
        frac_pi_4 * x.clone() - x.clone() * (x.clone().abs() - Self::ONE) * (Self::new(2447_u64, NonZeroU64::new(10000_u64).unwrap()) + Self::new(663_u64, NonZeroU64::new(10000_u64).unwrap()) * x.abs())
    }
}