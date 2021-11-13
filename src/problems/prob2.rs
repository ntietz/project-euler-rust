struct FibonacciSeq {
    current: u64,
    prev: u64,
}

impl FibonacciSeq {
    fn new() -> FibonacciSeq {
        FibonacciSeq {
            current: 1,
            prev: 0,
        }
    }
}

impl Iterator for FibonacciSeq {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let tmp = self.prev;
        self.prev = self.current;
        self.current = self.current + tmp;

        Some(self.current)
    }
}

pub fn solution() -> u64 {
    let fib = FibonacciSeq::new();
    const limit: u64 = 4_000_000;

    fib.take_while(|&n| n < limit).filter(|n| n % 2 == 0).sum()
}
