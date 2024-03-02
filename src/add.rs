use std::{ops::Add, num::NonZeroU64};

use super::{Sign, Fraction, convert_fraction};
////////////////////////////////////////////////////
impl Add for &Fraction {
    type Output = Fraction;

    fn add(self, other: Self) -> Self::Output {
        if self.denom == other.denom {
            // If denominators are the same, just add numerators
            let new_numer = match (&self.sign, &other.sign) {
                (Sign::Positive, Sign::Positive) => {
                    let Some(thing) = self.numer.checked_add(other.numer) else {
                        let (numer, denom)= super::convert_fraction(
                            self.numer as u128 + other.numer as u128,
                            self.denom.get() as u128
                        );
                        return Fraction{ sign: self.sign.clone(), numer, denom: NonZeroU64::new(denom).unwrap() }.reduce_consuming();
                    };
                    thing
                },
                (Sign::Positive, Sign::Negative) => {
                    if self.numer >= other.numer {
                        self.numer - other.numer
                    } else {
                        return Fraction {
                            sign: Sign::Negative,
                            numer: other.numer - self.numer,
                            denom: self.denom,
                        };
                    }
                }
                (Sign::Negative, Sign::Positive) => {
                    if other.numer >= self.numer {
                        other.numer - self.numer
                    } else {
                        return Fraction {
                            sign: Sign::Negative,
                            numer: self.numer - other.numer,
                            denom: self.denom,
                        };
                    }
                }
                (Sign::Negative, Sign::Negative) => {
                    let Some(thing) = self.numer.checked_add(other.numer) else {
                        let (numer, denom)= super::convert_fraction(
                            self.numer as u128 + other.numer as u128,
                            self.denom.get() as u128
                        );
                        return Fraction { sign: self.sign.clone(), numer, denom: NonZeroU64::new(denom).unwrap() };
                    };
                    thing
                },
            };

            Fraction {
                sign: self.sign.clone(),
                numer: new_numer,
                denom: self.denom,
            }.reduce_consuming()
        } else {
            // If denominators are different, find a common denominator

            match (self.denom.checked_mul(other.denom), self.numer.checked_mul(other.denom.get()), other.numer.checked_mul(self.denom.get())) {
                (Some(common_denom), Some(new_numer_self), Some(new_numer_other)) => {
                    let new_numer = match (&self.sign, other.sign.clone()) {
                        (Sign::Positive, Sign::Positive) => new_numer_self + new_numer_other,
                        (Sign::Positive, Sign::Negative) => {
                            if new_numer_self >= new_numer_other {
                                new_numer_self - new_numer_other
                            } else {
                                return Fraction {
                                    sign: Sign::Negative,
                                    numer: new_numer_other - new_numer_self,
                                    denom: common_denom,
                                };
                            }
                        }
                        (Sign::Negative, Sign::Positive) => {
                            if new_numer_other >= new_numer_self {
                                new_numer_other - new_numer_self
                            } else {
                                return Fraction {
                                    sign: Sign::Negative,
                                    numer: new_numer_self - new_numer_other,
                                    denom: common_denom,
                                };
                            }
                        }
                        (Sign::Negative, Sign::Negative) => new_numer_self + new_numer_other,
                    };
        
                    Fraction {
                        sign: self.sign.clone(),
                        numer: new_numer,
                        denom: common_denom,
                    }.reduce_consuming()
        
                },
                (_,_,_) => {
                    let common_denom = self.denom.get() as u128 * other.denom.get() as u128; 
                    let new_numer_self = self.numer as u128 * other.denom.get() as u128;
                    let new_numer_other = other.numer as u128 * self.denom.get() as u128;
                            let new_numer = match (&self.sign, other.sign.clone()) {
                        (Sign::Positive, Sign::Positive) => new_numer_self + new_numer_other,
                        (Sign::Positive, Sign::Negative) => {
                            if new_numer_self >= new_numer_other {
                                new_numer_self - new_numer_other
                            } else {
                                let (numer, denom)= convert_fraction(new_numer_other - new_numer_self, common_denom);
                                return Fraction {
                                    sign: Sign::Negative,
                                    numer,
                                    denom: NonZeroU64::new(denom).unwrap()
                                };
                            }
                        }
                        (Sign::Negative, Sign::Positive) => {
                            if new_numer_other >= new_numer_self {
                                new_numer_other - new_numer_self
                            } else {
                                let (numer, denom)= convert_fraction(new_numer_self - new_numer_other, common_denom);
                                return Fraction {
                                    sign: Sign::Negative,
                                    numer: numer,
                                    denom: NonZeroU64::new(denom).unwrap(),
                                };
                            }
                        }
                        (Sign::Negative, Sign::Negative) => new_numer_self + new_numer_other,
                    };
                    let (numer, denom)= convert_fraction(new_numer, common_denom);
                                
                    Fraction {
                        sign: self.sign.clone(),
                        numer,
                        denom: NonZeroU64::new(denom).unwrap(),
                    }.reduce_consuming()
        
                }
            }

        }
    }
}

impl Add for Fraction {
    type Output = Fraction;

    fn add(self, other: Self) -> Self::Output {
        &self + &other
    }
}
impl Add<&Fraction> for Fraction {
    type Output = Fraction;

    fn add(self, other: &Fraction) -> Self::Output {
        &self + other
    }
}
impl Add<Fraction> for &Fraction {
    type Output = Fraction;

    fn add(self, other: Fraction) -> Self::Output {
        self + &other
    }
}