//! Miller-Rabin primality test

use firescout::random::get_random_biguint_from_range;
use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::{One, Zero};

/// Checks if `n` is probably a prime.
///
/// Probability of correct positive result depends on the number of rounds
/// applied. Pessimistic bound for probability is $`4^{-\text{rounds}}`$. E.g.
/// with $`16`$ rounds, $`1`$ in $`4^{16} \approx 4{,}29 \cdot 10^{9}`$
/// numbers reported to probably be primes is in fact not a prime.
///
/// Arguments:
/// * The number to test for primality. Must be odd. Must be greater than 3.
/// * Number of rounds
///
/// Returns:
/// * True, if `n` is probably a prime
/// * False, if `n` is not a prime
pub fn mrt(n: &BigUint, rounds: u32) -> bool {
    assert!(n > &BigUint::from(3u32));
    assert!(n.is_odd());
    assert!(rounds > 0);
    let (r, d) = factorize(&n);
    for _i in 0..rounds {
        let a = pick_random(&n);
        if !mrt_round(n, &r, &d, &a) {
            return false;
        }
    }
    true
}

/// Factoring out powers of 2 from n − 1
///
/// Write $`n`$ as $`2r \cdot d + 1`$ with $`d`$ odd.
fn factorize(n: &BigUint) -> (/* r: */ BigUint, /* d: */ BigUint) {
    debug_assert!(n > &BigUint::from(3u32));
    debug_assert!(n.is_odd());
    let one = BigUint::one();
    let two = BigUint::from(2u32);
    let mut d = n - &one;
    let mut r = BigUint::zero();
    while d.is_even() {
        d /= &two;
        r += &one;
    }
    (r, d)
}

fn pick_random(n: &BigUint) -> BigUint {
    debug_assert!(n > &BigUint::from(3u32));
    debug_assert!(n.is_odd());
    let two = BigUint::from(2u32);
    let m = n - &two;
    get_random_biguint_from_range(two, m)
}

fn mrt_round(n: &BigUint, r: &BigUint, d: &BigUint, a: &BigUint) -> bool {
    debug_assert!(n > &BigUint::from(3u32));
    debug_assert!(n.is_odd());
    debug_assert!(d.is_odd());
    let one = BigUint::one();
    let two = BigUint::from(2u32);
    let m = n - &one;
    let mut x = a.modpow(d, n);
    if x == one || x == m {
        return true;
    }
    let mut j = BigUint::zero();
    while j < r - &one {
        j += &one;
        x = x.modpow(&two, &n);
        if x == m {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    #[test]
    fn mrt0() {
        use super::*;
        for _i in 0..1000 {
            assert_eq!(mrt(&BigUint::from(1111u32), 16), false);
        }
    }

    #[test]
    fn mrt1() {
        use super::*;
        for _i in 0..1000 {
            assert_eq!(mrt(&BigUint::from(1117u32), 16), true);
        }
    }

    #[test]
    fn tfactorize() {
        use super::*;
        let n = BigUint::from(3041u32);
        let (r, d) = factorize(&n);
        println!("{}, {}", r, d);
    }
}
