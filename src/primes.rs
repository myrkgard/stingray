//! The primes module.

use num_bigint::{BigUint, BigInt};

/// Provides convenience primality test with Miller-Rabin test.
pub trait IsProbablyPrimeMrt {
    /// Checks if this number is probably a prime.
    /// 
    /// Performs some short-cut tests and proceeds with Miller-Rabin test if needed.
    /// See [`mrt`][`crate::mrt::mrt`] for more info.
    /// 
    /// Arguments:
    /// * `rounds`: Must be greater than 0.
    /// 
    /// Result:
    /// * `True`, if number is probably a prime
    /// * `False`, if number is not a prime 
    fn is_probably_prime_mrt(&self, rounds: u32) -> bool;
}

impl IsProbablyPrimeMrt for BigUint {
    fn is_probably_prime_mrt(&self, rounds: u32) -> bool {
        use num_integer::Integer;
        use num_traits::{Zero, One};
        use crate::mrt;
    
        match self {
            x if x == &BigUint::zero() => false,
            x if x == &BigUint::one() => false,
            x if x == &BigUint::from(2u32) => true,
            x if x == &BigUint::from(3u32) => true,
            x if x.is_even() => false,
            _ => mrt::mrt(self, rounds),
        }
    }
}

impl IsProbablyPrimeMrt for BigInt {
    fn is_probably_prime_mrt(&self, rounds: u32) -> bool {
        use num_traits::Signed;
        if self.is_negative() {
            false
        } else {
            self.to_biguint().unwrap().is_probably_prime_mrt(rounds)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn do_it() {
        use super::*;
        use num_traits::Num;
        const ROUNDS: u32 = 16;

        assert_eq!(BigUint::from(0u32).is_probably_prime_mrt(ROUNDS), false);
        assert_eq!(BigUint::from(1u32).is_probably_prime_mrt(ROUNDS), false);
        assert_eq!(BigUint::from(2u32).is_probably_prime_mrt(ROUNDS), true);
        assert_eq!(BigUint::from(3u32).is_probably_prime_mrt(ROUNDS), true);
        assert_eq!(BigUint::from(4u32).is_probably_prime_mrt(ROUNDS), false);
        assert_eq!(BigUint::from(5u32).is_probably_prime_mrt(ROUNDS), true);
        assert_eq!(BigUint::from(6u32).is_probably_prime_mrt(ROUNDS), false);
        assert_eq!(BigUint::from(7u32).is_probably_prime_mrt(ROUNDS), true);
        assert_eq!(BigUint::from(8u32).is_probably_prime_mrt(ROUNDS), false);
        assert_eq!(BigUint::from(9u32).is_probably_prime_mrt(ROUNDS), false);
        assert_eq!(BigUint::from(10u32).is_probably_prime_mrt(ROUNDS), false);
        assert_eq!(BigUint::from(11u32).is_probably_prime_mrt(ROUNDS), true);
        assert_eq!(BigUint::from(12u32).is_probably_prime_mrt(ROUNDS), false);
        assert_eq!(BigUint::from(13u32).is_probably_prime_mrt(ROUNDS), true);
        assert_eq!(BigUint::from(14u32).is_probably_prime_mrt(ROUNDS), false);
        assert_eq!(BigUint::from(15u32).is_probably_prime_mrt(ROUNDS), false);

        let not_a_prime = BigUint::from_str_radix("918387933381198629487862179527787779653135890315238062816931", 10).unwrap();
        assert_eq!(not_a_prime.is_probably_prime_mrt(ROUNDS), false);

        let a_prime = BigUint::from_str_radix("19655025531041902030478224584153231378978641764268088923479", 10).unwrap();
        assert_eq!(a_prime.is_probably_prime_mrt(ROUNDS), true);
    }
}