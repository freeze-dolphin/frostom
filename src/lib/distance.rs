use std::mem;
use std::ops::AddAssign;

// original source: https://stackoverflow.com/questions/39638363/how-can-i-use-a-hashmap-with-f64-as-key-in-rust

pub fn integer_decode(val: f64) -> (u64, i16, i8) {
    let bits: u64 = unsafe { mem::transmute(val) };
    let sign: i8 = if bits >> 63 == 0 { 1 } else { -1 };
    let mut exponent: i16 = ((bits >> 52) & 0x7ff) as i16;
    let mantissa = if exponent == 0 {
        (bits & 0xfffffffffffff) << 1
    } else {
        (bits & 0xfffffffffffff) | 0x10000000000000
    };

    exponent -= 1023 + 52;
    (mantissa, exponent, sign)
}

#[derive(Hash, Eq, PartialEq)]
pub struct Distance((u64, i16, i8));

impl Distance {
    pub fn new(val: f64) -> Distance {
        Distance(integer_decode(val))
    }
}

impl AddAssign for Distance {
    fn add_assign(&mut self, rhs: Self) {
        todo!();
    }
}
