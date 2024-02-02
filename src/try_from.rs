use qol::OkOr;
use crate::{Fraction, Sign};


impl TryFrom<f32> for Fraction {
    type Error = String;

    fn try_from(value: f32) -> Result<Self, Self::Error> {        
        if value.is_nan() { return Err("Fraction doesn't have NaN".into()) };
        if value.is_infinite()  { return Err("Fraction doesn't have infinities".into()) };

        let sign = if value < 0.0 { Sign::Negative } else { Sign::Positive };

        let mut p: i32 = 0;
        let mut new_val = value;
        let ten: f32 = 10.0;

        loop {
            if (new_val.floor() - new_val).abs() < f32::EPSILON {
                // the precision of this number has been found
                break;
            }
            p += 1;
            new_val = value * ten.powi(p);
            if new_val.is_infinite() {
                todo!( "fallback_to_string_conversion()");
            }
        }

        if new_val < 0.0 {
            new_val = -new_val;
        }

        let (numer, denom) = super::reduce(
            new_val as u64, 
            ten.powi(p) as u64
        );
        Ok(Self{ sign, numer, denom: denom.try_into().ok_or("denom can't be zero".to_string())? })
    }
}


impl TryFrom<f64> for Fraction {
    type Error = String;

    fn try_from(value: f64) -> Result<Self, Self::Error> {        
        if value.is_nan() { return Err("Fraction doesn't have NaN".into()) };
        if value.is_infinite()  { return Err("Fraction doesn't have infinities".into()) };

        let sign = if value < 0.0 { Sign::Negative } else { Sign::Positive };

        let mut p: i32 = 0;
        let mut new_val = value;
        let ten: f64 = 10.0;

        loop {
            if (new_val.floor() - new_val).abs() < f64::EPSILON {
                // the precision of this number has been found
                break;
            }
            p += 1;
            new_val = value * ten.powi(p);
            if new_val.is_infinite() {
                todo!( "fallback_to_string_conversion()");
            }
        }

        if new_val < 0.0 {
            new_val = -new_val;
        }

        let (numer, denom) = super::reduce(
            new_val as u64, 
            ten.powi(p) as u64
        );
        Ok(Self{ sign, numer, denom: denom.try_into().ok_or("denom can't be zero".to_string())? })
    }
}