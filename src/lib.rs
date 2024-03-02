
mod abs;
mod add;
mod atan;
mod atan2;
mod consts;
mod display;
mod div;
mod from;
mod into;
mod mul;
mod neg;
mod ord;
mod partial_eq;
mod rem;
mod sub;
use std::num::NonZeroU64;
mod try_from;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Sign {
    Positive,
    Negative,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Eq, Hash)]
pub struct Fraction {
    sign: Sign,
    numer: u64,
    denom: NonZeroU64,
}
/// Creation 
impl Fraction {
    pub fn new(numer: u64, denom: NonZeroU64) -> Self {
        Self { sign: Sign::Positive, numer, denom }
    }

    pub fn try_new(numer: u64, denom: u64) -> Option<Self> {
        let Some(denom) = NonZeroU64::new(denom) else {
            return None
        };
        Some(Self { sign: Sign::Positive, numer, denom })
    }

    pub fn from<T: Into<Fraction>>(value:T) -> Self {
        value.into()
    }
}

// Internal
#[allow(dead_code)]
impl Fraction {
    fn reduce(&mut self) {
        let gcd = gcd(self.numer, self.denom.get());
        self.numer /= gcd;
        self.denom = NonZeroU64::new(self.denom.get() / gcd).expect("fraction shouldn't reduce to zero");
    }

    fn reduce_consuming(mut self) -> Self {
        let gcd = gcd(self.numer, self.denom.get());
        self.numer /= gcd;
        self.denom = NonZeroU64::new(self.denom.get() / gcd).expect("fraction shouldn't reduce to zero");
        self
    }
}

impl Fraction {
    pub fn is_negative(&self) -> bool {
        match self.sign {
            Sign::Positive => false,
            Sign::Negative => !(self == &Self::ZERO),
        }
    }
    pub fn is_positive(&self) -> bool {
        match self.sign {
            Sign::Positive => !(self == &Self::ZERO),
            Sign::Negative => false,
        }
    }
    pub fn to_f32(&self) -> f32 {
        let sign_multiplier = match self.sign {
            Sign::Positive => 1.0,
            Sign::Negative => -1.0,
        };

        sign_multiplier * (self.numer as f32 / self.denom.get() as f32)
    }
    pub fn to_f64(&self) -> f64 {
        let sign_multiplier = match self.sign {
            Sign::Positive => 1.0,
            Sign::Negative => -1.0,
        };

        sign_multiplier * (self.numer as f64 / self.denom.get() as f64)
    }
}


fn gcd<T: Clone + std::cmp::PartialEq<T> + From<u8> + std::ops::Rem<Output = T>>(mut a: T, mut b: T) -> T {
    let zero: T = 0_u8.into();
    while b != zero {
        let temp = b.clone();
        b = a % b;
        a = temp;
    }
    a
}

fn reduce<T: Clone + std::cmp::PartialEq<T> + From<u8> + std::ops::Rem<Output = T> + std::ops::Div<Output = T>>(numer: T, denom: T ) -> (T, T) {
    let gcd = gcd(numer.clone(), denom.clone());
    (
        numer / gcd.clone(),
        denom / gcd
    )
}

fn convert_fraction(mut numer: u128, mut denom: u128) -> (u64, u64) {
    // first simplify the fraction so the numer and denom as the smallist they can be.
    (numer, denom) = reduce(numer, denom);
        
    let max = numer.max(denom);
    let u64_max = u128::from(u64::MAX);
    if max > u64_max {
        let ratio = max / u64_max;

        
        let numer64 = (numer * ratio) as u64;
        let denom64 = (denom * ratio) as u64;
        return (numer64, denom64);
    } else {
         return (numer as u64, denom as u64);
    }
}