//! The egcd module.
//!
//! Read: <https://brilliant.org/wiki/extended-euclidean-algorithm/>

use num_bigint::BigInt;

/// Provides egcd.
pub trait Egcd<U> {
    /// dings
    fn egcd(&self, other: &U) -> (U, U, U);
}

impl Egcd<BigInt> for BigInt {
    /// Compute the greatest common divisor (gcd) using the Extended Euclidean Algorithm.
    ///  
    /// $`\text{gcd}(a, b) = ax + by`$
    ///
    /// Returns:
    ///
    /// * Element 0 : $`\text{gcd}`$
    /// * Element 1 : $`x`$
    /// * Element 2 : $`y`$
    fn egcd(
        &self,
        other: &BigInt,
    ) -> (
        /*gcd: */ BigInt,
        /*x: */ BigInt,
        /*y: */ BigInt,
    ) {
        use num_integer::Integer;
        use num_traits::{One, Zero};
    
        let mut a = self.clone();
        let mut b = other.clone();
        let mut x = BigInt::zero();
        let mut y = BigInt::one();
        let mut u = BigInt::one();
        let mut v = BigInt::zero();
    
        while !a.is_zero() {
            let q = &b / &a;
            let r = b.mod_floor(&a);
            let m = &x - &u * &q;
            let n = &y - &v * &q;
    
            b = a.clone();
            a = r;
            x = u.clone();
            y = v.clone();
            u = m;
            v = n;
        }
    
        (b, x, y)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn tegcd() {
        use super::*;
        // gcd(a, b) = a*x + b*y
        let a = BigInt::from(1432i32);
        let b = BigInt::from(123211i32);
        let (g, x, y) = a.egcd(&b);
        println!("a={}, b={}, g={}, x={}, y={}", a, b, g, x, y);
        assert_eq!(x, BigInt::from(-22973i32));
        assert_eq!(y, BigInt::from(267i32));
    }
}
