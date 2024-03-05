use qol::logy;

use crate::Fraction;


impl Fraction {
    fn newtons_guess(&self, guess: &Fraction) -> Option<Self> {
        let Fraction { sign: _, numer: a1, denom: b1 } = guess;
        let Fraction { sign, numer: c1, denom: d1 } = self;
        let a = *a1 as u128;
        let b = b1.get() as u128;
        let c = *c1 as u128;
        let d = d1.get() as u128;
        
        let asq = a.checked_mul(a)?;
        let lhs = asq.checked_mul(d)?;
        let bsq = b.checked_mul(b)?;
        let rhs = bsq.checked_mul(c)?;

        let numer1 = lhs.checked_add(rhs)?;
        let denom1 = 2_u128
            .checked_mul(a)?
            .checked_mul(b)?
            .checked_mul(d)?;
        let (numer, denom2)= super::convert_fraction(numer1, denom1);
        let denom = std::num::NonZeroU64::new(denom2)?;
        if &numer == a1 && &denom == b1 {
            return None
        }
        //let gcd = gcd(&numer, &denom);
        Some(Self {sign: sign.clone(), numer, denom})
    }

    
    pub fn sqrt(&self) -> Fraction{
        let simple = self.clone().reduce_consuming();
        let mut guess: Fraction = &simple / Fraction::TWO;
        
        for _idx in 0..10 {
            let Some(thing) = simple.newtons_guess(&guess) else {
                logy!("trace", "found in {_idx} interations");
                return guess.reduce_consuming()
            };
            guess = thing;
            //println!("{}/{}={}", guess.numer, guess.denom, guess.numer as f64 / guess.denom as f64);
        }
        guess.reduce_consuming()
    }
}
