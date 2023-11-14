use std::fmt;

use super::{Fraction, Sign};

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.sign == Sign::Negative {
            write!(f, "-")?;
        };
    // Handle the case where the fraction is a whole number
        if self.numer % self.denom.get() == 0 {
            write!(f, "{}", self.numer / self.denom.get())
        } else {
            // Initialize the result as a string with the sign
            let mut result = String::new();

            // Take the absolute values for simplicity
            let (numerator, denominator) = (self.numer, self.denom.get());
            if let Some(precision) = f.precision() {
                // Add the integer part to the result
                write!(f, "{}.", numerator / denominator)?;
    
                // Initialize a vector to store remainder positions for repeating decimals
                let mut remainder_positions = Vec::new();
                remainder_positions.reserve_exact(10);
    
                // Perform long division to find the decimal part
                let mut remainder = numerator % denominator;
                
                let mut idx = 0;
                while remainder != 0 {
                    
                    if idx == precision {
                        return write!(f, "{}...", result)
                    };
    
                    // If the remainder repeats, insert a parenthesis and break
                    if let Some(position) = remainder_positions.iter().position(|&r| r == remainder) {
                    let repeat = result.split_off(position );
                        return write!(f, "{result}({repeat})")
                    }
    
                    // Update the remainder position
                    remainder_positions.push(remainder);
    
                    // Perform the division
                    remainder *= 10;
                    result.push_str(&format!("{}", remainder / denominator));
                    remainder %= denominator;
                    
                    idx += 1;
                }
    
                write!(f, "{} ", result)
            } else {
                // Add the integer part to the result
                let integer = numerator / denominator;
                if integer != 0 {
                    write!(f, "{integer} ")?;
                };
                let remainder = numerator / denominator;
                let (n,d) = super::simplify(remainder, denominator);
                write!(f, "{n}/{d}")
            }
        }
    }
}

#[test]
fn short_repeating_test() {
    let Some(fraction) = Fraction::try_new(4553, 9900) else {
        return
    };

    println!("{}", fraction);

    assert_eq!("0.45(98)", &format!("{fraction}"));
}


#[test]
fn long_non_repeating_teat() {
    let Some(fraction) = Fraction::try_new(1234567890987, 10000000000000) else {
        return
    };

    println!("{:}", fraction);
        assert_eq!("0.1234567890987", &format!("{fraction}"));

}

    #[test]
fn long_repeating_teat() {
    let Some(fraction) = Fraction::try_new(1, 97) else {
        return
    };

    println!("{:}", fraction);
        assert_eq!("0.(010309278350515463917525773195876288659793814432989690721649484536082474226804123711340206185567)", &format!("{fraction}"));

}