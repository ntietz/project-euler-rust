pub struct FibonacciSeq {
    current: u64,
    prev: u64,
}

impl FibonacciSeq {
    pub fn new() -> FibonacciSeq {
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


