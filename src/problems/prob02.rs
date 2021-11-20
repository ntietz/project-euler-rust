use crate::math::fibonacci::FibonacciSeq;

pub fn solution() -> u64 {
    let fib = FibonacciSeq::new();
    const LIMIT: u64 = 4_000_000;

    fib.take_while(|&n| n < LIMIT).filter(|n| n % 2 == 0).sum()
}
