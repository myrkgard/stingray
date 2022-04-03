//! The modinverse module.

use num_bigint::BigInt;

/// Provides modinverse.
pub trait Modinverse<U> {
    /// Modular multiplicative inverse of `this number` modulo `modulus`.
    fn modinverse(&self, modulus: &U) -> U;
}

impl Modinverse<BigInt> for BigInt {
    /// Modular multiplicative inverse of [`BigInt`] modulo `modulus`.
    fn modinverse(&self, modulus: &BigInt) -> BigInt {
        use crate::egcd::Egcd;
        use num_traits::Zero;
        let (_, mut s, _) = self.egcd(modulus);
        while !(s > BigInt::zero()) {
            s += modulus;
        }
        s
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn t_modinv0() {
        use super::*;
        // modular multiplicative inverse of a (mod b)
        // computer q_inv = mmi of q (mod p)
        let a = BigInt::from(53i32); // q
        let b = BigInt::from(61i32); // p
        let i = a.modinverse(&b);
        println!("a={}, b={}, i={}", a, b, i);
        assert_eq!(i, BigInt::from(38i32));
    }

    #[test]
    fn t_modinv1() {
        use super::*;
        // modular multiplicative inverse of a (mod b)
        // compute d = mmi of e (mod λ(n))
        let a = BigInt::from(17u32); // e
        let b = BigInt::from(780u32); // λ(n)
        let i = a.modinverse(&b);
        println!("a={}, b={}, i={}", a, b, i);
        assert_eq!(i, BigInt::from(413i32));
    }
}
