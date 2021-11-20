use crate::math::primes::Primes;

pub fn solution() -> u64 {
    let mut primes = Primes::new();
    primes.nth(10_000).unwrap()
}
