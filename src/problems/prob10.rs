use crate::math::primes::Primes;

pub fn solution() -> u64 {
    solve(2_000_000)
}

fn solve(limit: u64) -> u64 {
    let primes = Primes::new();

    primes.take_while(|&n| n < limit).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn checks_example() {
        assert_eq!(17, solve(10));
    }
}
