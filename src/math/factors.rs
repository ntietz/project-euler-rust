use crate::math::primes::Primes;
use std::collections::HashMap;

/// Determines the prime factors of a given number. If a factor occurs
/// multiple times, it will be returned multiple times.
///
/// Factors are returned in sorted order.
pub fn prime_factors(x: u64) -> Vec<u64> {
    let mut primes = Primes::new();
    prime_factors_memoize(x, &mut primes)
}

pub fn prime_factors_memoize(x: u64, primes: &mut Primes) -> Vec<u64> {
    let mut factors = vec![];

    let mut x = x;
    let mut idx = 0;

    while x > 1 {
        let factor = primes.get(idx);
        idx += 1;

        while x % factor == 0 {
            factors.push(factor);
            x /= factor;
        }
    }

    factors
}

pub fn num_factors(x: u64) -> u64 {
    let mut primes = Primes::new();
    num_factors_memoize(x, &mut primes)
}

pub fn num_factors_memoize(x: u64, primes: &mut Primes) -> u64 {
    let factors = prime_factors_memoize(x, primes);

    let mut factor_counts: HashMap<u64, u64> = HashMap::new();

    for factor in factors {
        let count = factor_counts.entry(factor).or_insert(0);
        *count += 1;
    }

    factor_counts.values().map(|c| c + 1).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factors_13195() {
        let expected = vec![5, 7, 13, 29];
        let actual = prime_factors(13195);
        assert_eq!(expected, actual);
    }

    #[test]
    fn factors_300() {
        let expected = vec![2, 2, 3, 5, 5];
        let actual = prime_factors(300);
        assert_eq!(expected, actual);
    }

    #[test]
    fn checks_num_factors_of_28() {
        let expected = 6;
        let actual = num_factors(28);
        assert_eq!(expected, actual);
    }

    #[test]
    fn checks_num_factors_of_large_num() {
        let num = 2 * 2 * 3 * 3 * 3 * 5 * 7 * 11 * 11;
        let expected = (1..=num).filter(|&n| num % n == 0).count();

        assert_eq!(expected as u64, num_factors(num));
    }
}
