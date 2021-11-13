pub struct Primes {
    known: Vec<u64>,
    curr: usize,
}

impl Primes {
    pub fn new() -> Primes {
        Primes {
            known: vec![2, 3],
            curr: 0,
        }
    }

    pub fn refill(&mut self) {
        let max_known = self.known[self.known.len() - 1];
        let chunk_size = 1000;

        let lower = max_known + 2;
        let upper = lower + chunk_size;

        for n in (lower..upper).step_by(2) {
            if !self.known.iter().any(|p| n % p == 0) {
                self.known.push(n)
            }
        }
    }
}

impl Iterator for Primes {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr >= self.known.len() {
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
}
