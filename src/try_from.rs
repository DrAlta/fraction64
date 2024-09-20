use num_traits::Float;

use crate::{u128_to_fraction, Fraction, Sign};


impl TryFrom<f32> for Fraction {
    type Error = String;

    fn try_from(value: f32) -> Result<Self, Self::Error> {        
        if value.is_nan() { return Err("Fraction doesn't have NaN".into()) };
        if value.is_infinite()  { return Err("Fraction doesn't have infinities".into()) };

        // -0.0 + 0.0 == +0.0 under IEEE754 roundTiesToEven rounding mode,
        // which Rust guarantees. Thus by adding a positive zero we
        // canonicalize signed zero without any branches in one instruction.
        // doing it on this side of the fuction call so that we can get zero with `0.0`
        Ok(float_to_fraction_direct_from_mantissa_and_exponent(value + 0.0).ok_or("Float is out of range".to_string())?)
    }
}


impl TryFrom<f64> for Fraction {
    type Error = String;

    fn try_from(value: f64) -> Result<Self, Self::Error> {        
        if value.is_nan() { return Err("Fraction doesn't have NaN".into()) };
        if value.is_infinite()  { return Err("Fraction doesn't have infinities".into()) };

        // -0.0 + 0.0 == +0.0 under IEEE754 roundTiesToEven rounding mode,
        // which Rust guarantees. Thus by adding a positive zero we
        // canonicalize signed zero without any branches in one instruction.
        // doing it on this side of the fuction call so that we can get zero with `0.0`
        Ok(float_to_fraction_direct_from_mantissa_and_exponent(value + 0.0).ok_or("Float is out of range".to_string())?)
    }
}



fn float_to_fraction_direct_from_mantissa_and_exponent<F: Float>(value: F) -> Option<Fraction> {
    if value  == F::zero() {
        return Some(Fraction::ZERO)
    }
    
    let (mantissa, exponent, sign) = Float::integer_decode(value);
    let new_sign = if sign < 0 {
        Sign::Negative
    } else {
        Sign::Positive
    };

    let Some(x) = 2_u128.checked_pow(exponent.unsigned_abs() as u32) else {
        return Some(if exponent > 0 {
            Fraction{ sign: new_sign, numer: std::u64::MAX, denom: std::num::NonZeroU64::new(1).unwrap() }
        } else {
            Fraction::ZERO
        })
    };
    
    let(radix_numer, radix_denom) = if exponent > 0 {
        (1, x)
    } else {
        (x, 1)
    };
    let mantissa = mantissa as u128;
    Some(u128_to_fraction(new_sign, mantissa * radix_denom, radix_numer))
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU64;

    use crate::Fraction;
    #[test]
    fn zero_f32() {
        let value: Fraction = 0.0_f32.try_into().unwrap(); 
        assert_eq!(
            value,
            Fraction::ZERO
        );
    }
    #[test]
    fn point25_f32() {
        let value: Fraction = 0.25_f32.try_into().unwrap(); 
        assert_eq!(
            value,
            Fraction::new(1, NonZeroU64::new(4_u64).unwrap())
        );
    }
    #[test]
    fn point25_f64() {
        let value: Fraction = 0.25_f64.try_into().unwrap(); 
        assert_eq!(
            value,
            Fraction::new(1, NonZeroU64::new(4_u64).unwrap()),
        );
    }
    #[test]
    fn neg_point25_f64() {
        let value = TryInto::<Fraction>::try_into(-0.25_f64).unwrap(); 
        assert_eq!(
            value,
            Fraction::new_neg(1, NonZeroU64::new(4_u64).unwrap()),
        );
    }
}