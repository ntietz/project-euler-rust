use crate::math::primes::Primes;

/// Determines the prime factors of a given number. If a factor occurs
/// multiple times, it will be returned multiple times.
///
/// Factors are returned in sorted order.
pub fn prime_factors(x: u64) -> Vec<u64> {
    let mut primes = Primes::new();
    let mut factors = vec![];

    let mut x = x;

    while x > 1 {
        let factor = primes.next().unwrap();

        while x % factor == 0 {
            factors.push(factor);
            x /= factor;
        }
    }

    factors
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
}
