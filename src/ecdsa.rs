use std::ops::Mul;

use num_bigint::{BigInt, RandBigInt};
use num_traits::{One, Zero};
use rand::thread_rng;

use crate::{arithmetic::{Modular, Secp256k1Point}, secp256k1::Secp256k1Params};

#[derive(Debug, Clone, Default)]
pub struct BigInt256Bounds(pub BigInt, pub BigInt);

impl BigInt256Bounds {
    pub fn get() -> Self {
        BigInt256Bounds(
            BigInt::parse_bytes(
                b"-57896044618658097711785492504343953926634992332820282019728792003956564819968", 
                10
            ).unwrap(),
            BigInt::parse_bytes(
                b"57896044618658097711785492504343953926634992332820282019728792003956564819967",
                10
            ).unwrap()
        )
    }
}

#[derive(Debug)]
pub struct PrivateKey(pub BigInt);
#[derive(Debug)]
pub struct PublicKey(pub Secp256k1Point);

impl PrivateKey {
    pub fn generate() -> Self {
        let mut rng = thread_rng();
        let BigInt256Bounds(lbound, ubound) = BigInt256Bounds::get();
        let pk = rng.gen_bigint_range(&lbound, &ubound);
        Self(pk)
    }
}

impl PublicKey {
    pub fn new(private_key: &PrivateKey) -> Self {
        let g = Secp256k1Params::get().g;
        let PrivateKey(priv_key) = private_key;
        let pub_key = g.times(priv_key);
        PublicKey(pub_key)
    }
}

impl PartialEq for PrivateKey {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[derive(Debug)]
pub struct Signature {
    pub r: BigInt,
    pub s: BigInt
}

impl Signature {
    pub fn sign_message(message: &str, private_key: &PrivateKey) -> (Self, BigInt) {
        let Secp256k1Params{
            a: _, 
            b: _, 
            p: _, 
            g, 
            n
        } = Secp256k1Params::get();
        let (r, gen_k) = loop {
            let mut rng = thread_rng();
            let (lbound, ubound) = (One::one(), n.clone());
            let gen_k_temp = rng.gen_bigint_range(&lbound, &ubound);
            let R = g.times(&gen_k_temp);
            let r_temp = (R.x.unwrap()).modulus(&n);
            if r_temp.ne(&Zero::zero()) {
                break (r_temp, gen_k_temp);
            }
        };

        let msg_as_bigint = BigInt::from_signed_bytes_be(message.as_bytes());
         let s= ((msg_as_bigint + r.clone().mul(&private_key.0)) * gen_k.invmod(&n)
            .unwrap()).modulus(&n);
        (Self { r, s }, gen_k)
    }

    pub fn validate(message: &str, public_key: &PublicKey, signature: &Signature) -> bool {
        let Secp256k1Params{a: _, b: _, p: _, g, n} = Secp256k1Params::get();
        let msg_as_bigint = BigInt::from_signed_bytes_be(message.as_bytes());
        let inv_s = (&signature.s).invmod(&n).unwrap();
        let u = (msg_as_bigint.mul(&inv_s)).modulus(&n);
        let v = (&signature.r).mul(&inv_s).modulus(&n);
        let c = g.times(&u) + public_key.0.times(&v);
        if let (Some(c_x), Some(_)) = (c.x, c.y) {
            c_x.modulus(&n).eq(&signature.r)
        } else {
            false
        }
    }
}
