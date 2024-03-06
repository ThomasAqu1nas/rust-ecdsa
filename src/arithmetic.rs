use std::{fmt::Display, mem::size_of, ops::{Add, Div, Mul, Neg, Rem, Sub}};

use num_bigint::BigInt;
use num_traits::{FromPrimitive, One, Pow, Signed, Zero};
use secp256k1::Secp256k1Params;

use crate::secp256k1;

pub trait Modular: Add + Div + Mul + Sub + Sized
{
    fn addmod(&self, rhs: &Self, modulus: &Self) -> Self;
    fn modulus(&self, rhs: &Self) -> Self;
    fn mulmod(&self, rhs: &Self, modulus: &Self) -> Self;
    fn submod(&self, rhs: &Self, modulus: &Self) -> Self;
    fn powmod(&self, exp: &Self, modulus: &Self) -> Self;
    fn invmod(&self, modulus: &Self) -> Option<Self>;
    fn gcd(m: &Self, n: &Self) -> (Self, Self, Self);
}

impl Modular for BigInt {

    fn addmod(&self, rhs: &Self, modulus: &Self) -> Self {
        self.modulus(modulus) + rhs.modulus(modulus)
    }

    fn modulus(&self, rhs: &Self) -> Self {
        let rem = self.rem(rhs);
        match rem.sign() {
            num_bigint::Sign::Minus => (&rem + rhs).rem(rhs),
            num_bigint::Sign::NoSign => rem,
            num_bigint::Sign::Plus => rem,
        }
    }

    fn mulmod(&self, rhs: &Self, modulus: &Self) -> Self {
        (self.modulus(modulus) * rhs.modulus(modulus)).modulus(modulus) 
    }

    fn submod(&self, rhs: &Self, modulus: &Self) -> Self {
        (self.modulus(modulus) - rhs.modulus(modulus)).modulus(modulus)
    }

    fn powmod(&self, exp: &Self, modulus: &Self) -> Self {
        self.modpow(&exp, &modulus)
    }
    
    fn invmod(&self, modulus: &Self) -> Option<Self> {
        let (gcd, x, _) = Modular::gcd(self, modulus);
        if gcd.eq(&One::one()) {
            Some((x.modulus(modulus) + modulus).modulus(modulus))
        } else {
            None
        }
    }
    
    fn gcd(a: &Self, b: &Self) -> (Self, Self, Self) {
        if a.eq(&Zero::zero()) {
            (b.clone(), Zero::zero(), One::one())
        } else {
            let (g, x, y) = Modular::gcd(&b.modulus(a), a);
            (g, &y - &x * (b / a), x)
        }
    } 
}

#[derive(Clone, Debug)]
pub struct Secp256k1Point {
    pub x: Option<num_bigint::BigInt>,
    pub y: Option<num_bigint::BigInt>
}

impl Secp256k1Point {

    pub fn init(x: BigInt) -> Self {
        let a = Secp256k1Params::get().a;
        let b = Secp256k1Params::get().b;
        let p = Secp256k1Params::get().p;
        let y = BigInt::sqrt(
            &(&x.clone().pow(3u8) + &a * &x + &b).modulus(&p)
        );
        Self { x: Some(x), y: Some(y) }
    }

    pub fn free_dot(x: BigInt, y: BigInt) -> Self {
        Self { x: Some(x), y: Some(y) }
    }

    pub fn times_u64(&self, n: u64) -> Secp256k1Point {
        let bits = n.to_bits();
        let this = (*self).clone();
        let mut res = (*self).clone();
        for i in bits {
            if i {
                res = res.times_two() + this.clone();
            } else {
                res = res.times_two();
            }
        }
        res
    }

    pub fn times(&self, n: &BigInt) -> Secp256k1Point {
        let bits = n.to_bits();
        let this = (*self).clone();
        let mut res = (*self).clone();
        for i in bits {
            if i {
                res = res.times_two() + this.clone();
            } else {
                res = res.times_two();
            }
        }
        res
    }

    pub fn times_two(&self) -> Secp256k1Point {
        if self.eq(&Secp256k1Point::zero()) {
            return Secp256k1Point::zero();
        }
        if let (
            Some(x), 
            Some(y), 
        ) = (&self.x, &self.y) {
            let three: BigInt = BigInt::from_i8(3i8)
                .unwrap();
            let two: BigInt = BigInt::from_i8(2i8)
                .unwrap();
            let a: BigInt = Secp256k1Params::get().a;
            let modulus: BigInt = Secp256k1Params::get().p;
            let lambda_1: BigInt = &(three * x.clone().pow(2u8)) + &a;
            let lambda_2: BigInt = (&two * y).invmod(&modulus).unwrap();
            let lambda = (&lambda_1 * &lambda_2).modulus(&modulus);
            let res_x = (&lambda.clone().pow(2u8) - x - x).modulus(&modulus);
            let res_y = (&(lambda * (x - &res_x)) - y).modulus(&modulus);
            Secp256k1Point {
                x: Some(res_x),
                y: Some(res_y),
            }
        } else {
            return Secp256k1Point::zero();
        }
    }
}

impl Zero for Secp256k1Point {
    fn zero() -> Self {
        Secp256k1Point { x: None::<BigInt>, y: None::<BigInt> }
    }

    fn is_zero(&self) -> bool {
        self.eq(&Secp256k1Point { x: None::<BigInt>, y: None::<BigInt> })
    }
}

impl Default for Secp256k1Point {
    fn default() -> Self {
        Self { x: Default::default(), y: Default::default()}
    }
}

impl PartialEq for Secp256k1Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Add for Secp256k1Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.eq(&rhs) {
            if self.is_zero() {
                return Secp256k1Point::zero();
            }
            return self.times_two();
        }
        if self.is_zero() {
            return rhs;
        }
        if rhs.is_zero() {
            return self;
        }
        if (self.x).eq(&rhs.x) {
            return Secp256k1Point {
                x: None,
                y: None,
            };
        }
        if let (
            Some(a_x), 
            Some(a_y), 
            Some(b_x), 
            Some(b_y)
        ) = (&self.x, &self.y, &rhs.x, &rhs.y) {
            let modulus = Secp256k1Params::get().p;
            let lambda = ((b_y - a_y) * (b_x - a_x).invmod(&modulus).unwrap()).modulus(&modulus);
            let res_x = (&lambda.clone().pow(2u8) - a_x - b_x).modulus(&modulus);
            let res_y = (&lambda * &(a_x - &res_x) - a_y).modulus(&modulus);
            Secp256k1Point {
                x: Some(res_x),
                y: Some(res_y),
            }
        } else {
            Secp256k1Point {
                x: None,
                y: None,
            }
        }
    }
}

impl<'a, 'b> Add<&'b Secp256k1Point> for &'a Secp256k1Point {
    type Output = Secp256k1Point;

    fn add(self, rhs: &'b Secp256k1Point) -> Self::Output {
        if self.eq(rhs) {
            if self.is_zero() {
                return Secp256k1Point::zero();
            }
            return self.times_two();
        }
        if self.is_zero() {
            return rhs.clone();
        }
        if rhs.is_zero() {
            return self.clone();
        }
        if (self.x).eq(&rhs.x) {
            return Secp256k1Point {
                x: None,
                y: None,
            };
        }
        if let (
            Some(a_x), 
            Some(a_y), 
            Some(b_x), 
            Some(b_y)
        ) = (&self.x, &self.y, &rhs.x, &rhs.y) {
            let modulus = Secp256k1Params::get().p;
            let lambda = ((b_y - a_y) * (b_x - a_x).invmod(&modulus).unwrap()).modulus(&modulus);
            let res_x = (&lambda.clone().pow(2u8) - a_x - b_x).modulus(&modulus);
            let res_y = (&lambda * &(a_x - &res_x) - a_y).modulus(&modulus);
            Secp256k1Point {
                x: Some(res_x),
                y: Some(res_y),
            }
        } else {
            Secp256k1Point {
                x: None,
                y: None,
            }
        }
    }
}

impl Neg for Secp256k1Point {
    type Output = Self;

    fn neg(self) -> Self::Output {
        if self.eq(&Secp256k1Point::zero()) {
            return Secp256k1Point::zero();
        }
        let modulus = Secp256k1Params::get().p;
        if let (
            Some(x),
            Some(y)
        ) = (self.x, self.y) {
            Secp256k1Point {
                x: Some(x),
                y: Some((-y).modulus(&modulus)),
            }
        } else {
            Secp256k1Point::zero()
        }
    }
}

impl Display for Secp256k1Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let (
            Some(x),
            Some(y)
        ) = (&self.x, &self.y) {
            write!(f, "x: {}, y: {}", *x, *y)
        } else {
            write!(f, "x: {:?}, y: {:?}", None::<BigInt>, None::<BigInt>)
        }
    }
}

pub trait ToBits: Add + Div + Mul + Sub + Sized {
    fn to_bits(&self) -> Vec<bool>;  
}

impl ToBits for u64 {
    fn to_bits(&self) -> Vec<bool> {
        let bits = size_of::<Self>() * 8; // Количество битов в типе T
        (0..bits).rev().map(|i| *self & (1 << i) != 0).collect()
    }
}

impl ToBits for BigInt {
    fn to_bits(&self) -> Vec<bool> {
        let bytes = self.to_signed_bytes_be();
        let mut bits: Vec<bool> = Vec::new();
        for byte in bytes {
            for i in 0..8 {
                let bit = (byte >> (7 - i)) & 1;
                bits.push(bit == 1);
            }
        }
        bits
    }
}

#[cfg(test)]
mod tests {

    use num_bigint::BigInt;
    use num_traits::FromPrimitive;

    use crate::{ecdsa::PrivateKey, secp256k1::Secp256k1Params};

    use super::Modular;

    #[test]
    fn test_invmod() {
        let int = BigInt::from_u64(1514511242u64).unwrap();
        let modulus= BigInt::from_u64(123u64).unwrap();
        let invmod = int.invmod(&modulus).unwrap();
        println!("invmod: {:?}", invmod);
    }

    #[test]
    fn test_gcd() {
        
        let modulus = BigInt::from_u64(1241231234124).unwrap();
        let int = BigInt::from_u64(13124).unwrap();
        let gcd: (BigInt, BigInt, BigInt) = Modular::gcd(&modulus, &int);
        println!("gcd: {:?}", gcd);
    }

    #[test]
    fn test_times_two() {
        let g = Secp256k1Params::get().g;
        let times_two = g.times_two();
        println!("g * 2: {:?}", times_two);
    }

    #[test]
    fn test_times() {
        let times = PrivateKey::generate().0;
        let g = Secp256k1Params::get().g;
        let s = BigInt::from_u16(12u16).unwrap();
        let pub_key_value = g.times(&s);
        println!("public key value: {:?}", pub_key_value);
    }
}
