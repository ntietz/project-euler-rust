use std::collections::BTreeSet;

pub struct Primes {
    known: Vec<u64>,
    curr: usize,
    max_tried: u64,
}

impl Primes {
    pub fn new() -> Self {
        Primes {
            known: vec![2, 3],
            curr: 0,
            max_tried: 3,
        }
    }

    pub fn refill(&mut self) {
        let chunk_size = 1000;

        let lower = self.max_tried;
        let upper = lower + chunk_size;

        let mut sieve: BTreeSet<u64> = (lower..upper).step_by(2).collect();

        self.known
            .iter()
            .copied()
            .for_each(|n| sieve_out(n, &mut sieve));

        while !sieve.is_empty() {
            let next_prime = sieve.pop_first().unwrap();
            self.known.push(next_prime);
            sieve_out(next_prime, &mut sieve);
        }

        self.max_tried = upper;
    }
}

impl Default for Primes {
    fn default() -> Self {
        Self::new()
    }
}

fn sieve_out(prime: u64, sieve: &mut BTreeSet<u64>) {
    if sieve.is_empty() {
        return;
    }

    let first = sieve.first().unwrap();
    let last = sieve.last().unwrap();

    let lower = first - (first % prime);
    let upper = last - (last % prime);

    for n in (lower..=upper).step_by(prime as usize) {
        sieve.remove(&n);
    }
}

impl Iterator for Primes {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        while self.curr >= self.known.len() {
            self.refill();
        }
        let result = self.known[self.curr];
        self.curr += 1;

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_primes() {
        let expected = vec![2, 3, 5, 7, 11, 13, 17, 19];

        let primes = Primes::new();
        let actual: Vec<_> = primes.take(8).collect();

        assert_eq!(expected, actual);
    }

    #[test]
    fn gets_later_prime() {
        let expected = 104743;
        let mut primes = Primes::new();
        let actual = primes.nth(10_000).unwrap();

        assert_eq!(expected, actual);
    }
}
