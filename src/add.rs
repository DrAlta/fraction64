use std::{num::NonZeroU64, ops::Add};

use qol::logy;

use super::{convert_fraction, Fraction, Sign};
////////////////////////////////////////////////////
impl Add for &Fraction {
    type Output = Fraction;
    fn add(self, other: Self) -> Self::Output {
        if self.denom == other.denom {
            logy!("debug", "[{}]denom's the same");
            // If denominators are the same, just add numerators
            let (new_sign, new_numer) = match (&self.sign, &other.sign) {
                (Sign::Positive, Sign::Positive) => {
                    let Some(thing) = self.numer.checked_add(other.numer) else {
                        let (numer, denom) = super::convert_fraction(
                            self.numer as u128 + other.numer as u128,
                            self.denom.get() as u128,
                        );
                        return Fraction {
                            sign: self.sign.clone(),
                            numer,
                            denom: NonZeroU64::new(denom).unwrap(),
                        }
                        .reduce_consuming();
                    };
                    (Sign::Positive, thing)
                }
                (Sign::Positive, Sign::Negative) => {
                    // 1s + -3o
                    // 1s >= 3o
                    // 30 - s1 = 2 -> -2
                    if self.numer >= other.numer {
                        (Sign::Positive, self.numer - other.numer)
                    } else {
                        (Sign::Negative, other.numer - self.numer)
                    }
                }
                (Sign::Negative, Sign::Positive) => {
                    // -1s + 3o
                    // 3o >= 1s
                    // then 3o - 1s = 2 -> +2
                    // -3s + 1o
                    // 1o >= 3s
                    // else 3s - 1o = 2 ->-2
                    if other.numer >= self.numer {
                        logy!(
                            "debug",
                            "other number{} >= self.number{}",
                            other.numer,
                            self.numer
                        );
                        (Sign::Positive, other.numer - self.numer)
                    } else {
                        (Sign::Negative, self.numer - other.numer)
                    }
                }
                (Sign::Negative, Sign::Negative) => {
                    let Some(thing) = self.numer.checked_add(other.numer) else {
                        let (numer, denom) = super::convert_fraction(
                            self.numer as u128 + other.numer as u128,
                            self.denom.get() as u128,
                        );
                        return Fraction {
                            sign: Sign::Negative,
                            numer,
                            denom: NonZeroU64::new(denom).unwrap(),
                        };
                    };
                    (Sign::Negative, thing)
                }
            };

            Fraction {
                sign: new_sign,
                numer: new_numer,
                denom: self.denom,
            }
            .reduce_consuming()
        } else {
            // If denominators are different, find a common denominator
            let common_denom = self.denom.get() as u128 * other.denom.get() as u128;
            let new_numer_self = self.numer as u128 * other.denom.get() as u128;
            let new_numer_other = other.numer as u128 * self.denom.get() as u128;
            let new_numer = match (&self.sign, other.sign.clone()) {
                (Sign::Positive, Sign::Positive) => new_numer_self + new_numer_other,
                (Sign::Positive, Sign::Negative) => {
                    if new_numer_self >= new_numer_other {
                        new_numer_self - new_numer_other
                    } else {
                        let (numer, denom) =
                            convert_fraction(new_numer_other - new_numer_self, common_denom);
                        return Fraction {
                            sign: Sign::Negative,
                            numer,
                            denom: NonZeroU64::new(denom).unwrap(),
                        };
                    }
                }
                (Sign::Negative, Sign::Positive) => {
                    if new_numer_other >= new_numer_self {
                        logy!("debug", "new_num_other >= new_num_self");
                        new_numer_other - new_numer_self
                    } else {
                        logy!("debug", "not (new_num_other >= new_num_self)");
                        let (numer, denom) =
                            convert_fraction(new_numer_self - new_numer_other, common_denom);
                        return Fraction {
                            sign: Sign::Negative,
                            numer: numer,
                            denom: NonZeroU64::new(denom).unwrap(),
                        };
                    }
                }
                (Sign::Negative, Sign::Negative) => new_numer_self + new_numer_other,
            };
            let (numer, denom) = convert_fraction(new_numer, common_denom);

            Fraction {
                sign: self.sign.clone(),
                numer,
                denom: NonZeroU64::new(denom).unwrap(),
            }
            .reduce_consuming()
        }
    }

    /*
        fn add(self, other: Self) -> Self::Output {
            if self.denom == other.denom {
                logy!("debug", "denom's the same");
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
                                    logy!("debug", "new_num_other >= new_num_self");
                                    new_numer_other - new_numer_self
                                } else {
                                    logy!("debug", "not (new_num_other >= new_num_self)");
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
    */
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

#[cfg(test)]
mod tests {
    use crate::Fraction;
    #[test]
    fn one_plus_one() {
        assert_eq!(Fraction::ONE + Fraction::ONE, Fraction::TWO)
    }
    #[test]
    fn one_plus_neg_one() {
        assert_eq!(Fraction::ONE + Fraction::NEG_ONE, Fraction::ZERO);
    }
    #[test]
    fn two_plus_neg_one() {
        assert_eq!(Fraction::TWO + Fraction::NEG_ONE, Fraction::ONE);
    }
    #[test]
    fn one_plus_neg_two() {
        assert_eq!(Fraction::ONE + Fraction::NEG_TWO, Fraction::NEG_ONE);
    }
    #[test]
    fn neg_one_plus_one() {
        assert_eq!(Fraction::NEG_ONE + Fraction::ONE, Fraction::ZERO);
    }
    #[test]
    fn neg_one_plus_two() {
        assert_eq!(Fraction::NEG_ONE + Fraction::TWO, Fraction::ONE);
    }
    #[test]
    fn neg_one_plus_neg_one() {
        assert_eq!(Fraction::NEG_ONE + Fraction::NEG_ONE, Fraction::NEG_TWO)
    }

    /////////////////////////////////////////////////////////////
    #[test]
    fn one_plus_third() {
        assert_eq!(Fraction::ONE + Fraction::THIRD, Fraction::FOUR_THIRDS)
    }
    #[test]
    fn one_plus_neg_third() {
        assert_eq!(Fraction::ONE + Fraction::NEG_THIRD, Fraction::TWO_THIRDS);
    }
    #[test]
    fn third_plus_one() {
        assert_eq!(Fraction::THIRD + Fraction::ONE, Fraction::FOUR_THIRDS);
    }
    #[test]
    fn third_plus_neg_one() {
        assert_eq!(Fraction::THIRD + Fraction::ONE, Fraction::FOUR_THIRDS);
    }

    #[test]
    fn neg_one_plus_third() {
        assert_eq!(
            Fraction::NEG_ONE + Fraction::THIRD,
            Fraction::NEG_TWO_THIRDS
        );
    }
    #[test]
    fn neg_one_plus_neg_third() {
        assert_eq!(
            Fraction::NEG_ONE + Fraction::NEG_THIRD,
            Fraction::NEG_FOUR_THIRDS
        );
    }
    #[test]
    fn neg_third_plus_one() {
        assert_eq!(Fraction::NEG_THIRD + Fraction::ONE, Fraction::TWO_THIRDS)
    }
    #[test]
    fn neg_third_plus_neg_one() {
        assert_eq!(
            Fraction::NEG_THIRD + Fraction::NEG_ONE,
            Fraction::NEG_FOUR_THIRDS
        )
    }
}
