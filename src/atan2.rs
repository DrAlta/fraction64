//! based on https://yal.cc/fast-atan2/  which is based on 
//! https://nghiaho.com/?p=997 which is based on 
//! “Efficient approximations for the arctangent function”, 
//! Rajan, S. Sichun Wang Inkol, R. Joyal, A., May 2006

use super::Fraction;

impl Fraction {
    pub fn atan2(&self, x: &Self) -> Self {
        let y = self;
     
        match(x >= &Self::ZERO, y >= &Self::ZERO, y < x) {
            (true, true, true) => {
                (x / y).atan()
            },
            (true, true, false) => {
                Self::FRAC_PI_2 - (x /y).atan()
            },
            (true, false, true) => {
                (y / x).atan()
            },
            (true, false, false) => {
                -Self::FRAC_PI_2 - (x /y).atan()
            },
            (false, true, true) => {
                (y / x).atan() + Self::PI
            },
            (false, true, false) => {
                Self::FRAC_PI_2 - (x / y).atan()
            },
            (false, false, true) => {
                (y / x).atan() - Self::PI
            },
            (false, false, false) => {
                -Self::FRAC_PI_2 - (x / y).atan()
            },
        }
    }
}



/*
pub fn test(){
    // -pi/4 radians (45 deg clockwise)
    let x1l = 3.0_f64;
    let y1l = -3.0_f64;

    // 3pi/4 radians (135 deg counter-clockwise)
    let x2l = -3.0_f64;
    let y2l = 3.0_f64;

    // -pi/4 radians (45 deg clockwise)
    let x1a = Fraction::from( 3_u8);
    let y1a = Fraction::from(-3_i8);

    // 3pi/4 radians (135 deg counter-clockwise)
    let x2a = Fraction::from(-3_i8);
    let y2a = Fraction::from( 3_u8);

    let atan1l = y1l.atan2(x1l);
    let atan2l = y2l.atan2(x2l);

    let atan1a: Fraction = y1a.atan2(x1a);
    let atan2a: Fraction = y2a.atan2(x2a);    

}
*/