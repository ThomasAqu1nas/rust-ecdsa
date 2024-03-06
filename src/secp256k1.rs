use num_bigint::BigInt;
use num_traits::Zero;

use crate::arithmetic::Secp256k1Dot;
pub struct Secp256k1Params {
    pub a: BigInt,
    pub b: BigInt,
    pub p: BigInt,
    pub g: Secp256k1Dot,
    pub n: BigInt
}

impl Secp256k1Params {
    pub fn get() -> Self {
        Self { 
            a: Zero::zero(), 
            b: BigInt::parse_bytes(
                b"0000000000000000000000000000000000000000000000000000000000000007",
                16
            ).unwrap(), 
            p: BigInt::parse_bytes(
                b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f", 
                16
            ).unwrap(),
            g: Secp256k1Dot::free_dot(
                    BigInt::parse_bytes(
                        b"79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798", 
                        16
                    ).unwrap(),
                    BigInt::parse_bytes(
                        b"483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8", 
                        16
                    ).unwrap(),
                ),
            n: BigInt::parse_bytes(
                b"fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141",
                16
            ).unwrap(),
            
        }
    }
}