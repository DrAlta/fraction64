use std::num::NonZeroU64;

use num_traits::Float;
//use qol::OkOr;
use crate::{convert_fraction, Fraction, Sign};


impl TryFrom<f32> for Fraction {
    type Error = String;

    fn try_from(value: f32) -> Result<Self, Self::Error> {        
        if value.is_nan() { return Err("Fraction doesn't have NaN".into()) };
        if value.is_infinite()  { return Err("Fraction doesn't have infinities".into()) };

        Ok(float_to_fraction_direct_from_mantissa_and_exponent(value).ok_or("Float is out of range".to_string())?)
    }
}


impl TryFrom<f64> for Fraction {
    type Error = String;

    fn try_from(value: f64) -> Result<Self, Self::Error> {        
        if value.is_nan() { return Err("Fraction doesn't have NaN".into()) };
        if value.is_infinite()  { return Err("Fraction doesn't have infinities".into()) };

        Ok(float_to_fraction_direct_from_mantissa_and_exponent(value).ok_or("Float is out of range".to_string())?)
    }
}



fn float_to_fraction_direct_from_mantissa_and_exponent<F: Float>(value: F) -> Option<Fraction> {
    let (mantissa, exponent, sign) = Float::integer_decode(value);
    let new_sign = if sign < 0 {
        Sign::Negative
    } else {
        Sign::Positive
    };

    let(radix_numer, radix_denom) = if exponent > 0 {
        (1, 2_u128.checked_pow(exponent.unsigned_abs() as u32)?)
    } else {
        (2_u128.checked_pow(exponent.unsigned_abs() as u32)?, 1)
    };
    let mantissa = mantissa as u128;
    let (numer, denom) = convert_fraction(mantissa * radix_denom, radix_numer);
    let ret = Fraction {
        sign: new_sign,
        numer,
        denom: NonZeroU64::new(denom)?,
    }.reduce_consuming();
    Some(ret)
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU64;

    use crate::Fraction;
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