use crate::math::factors::num_factors_memoize;
use crate::math::primes::Primes;
use crate::math::triangle_numbers::TriangleNumbers;

pub fn solution() -> u64 {
    solve(500)
}

fn solve(limit: u64) -> u64 {
    let mut primes = Primes::new();
    let mut triangle_numbers = TriangleNumbers::new();
    triangle_numbers
        .find(|&n| num_factors_memoize(n, &mut primes) > limit)
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_example() {
        let expected = 28;
        assert_eq!(expected, solve(5));
    }
}
