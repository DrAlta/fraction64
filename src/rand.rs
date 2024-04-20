extern crate rand;
use fraction::One;
use rand::{Rng, rngs::ThreadRng};

use crate::{geometry::{Coord, Fraction}, logy};
//use macroquad::prelude::*;


pub trait RandStuff {
    fn gen_fraction(&mut self) -> Fraction;
    fn gen_in_range(&mut self, a:Fraction, b: Fraction) -> Fraction;
    fn between(&mut self, start: &Coord, end: &Coord) -> Coord;
}

impl RandStuff for ThreadRng {
    fn gen_fraction(&mut self) -> Fraction {
        let rand: u16 = self.gen();
        Fraction::new(rand, u16::MAX)
    }
    fn gen_in_range(&mut self, a:Fraction, b: Fraction) -> Fraction {
        let min = a.min(b);
        let max = a.max(b);
        let diff = max - min;
        min + (diff * self.gen_fraction())
    }
    fn between(&mut self, start: &Coord, end: &Coord) -> Coord {
        let a = self.gen_fraction();
        let b = Fraction::one() - a;
        logy!("debug", "a: {a:.9}\nb: {b:.9}");
        let new_start = start.clone() * a;
        let new_end = end.clone() * b;
        new_start + new_end
    }
}
