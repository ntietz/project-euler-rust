pub fn sum_multiples(start: u64, lim: u64, factors: &[u64]) -> u64 {
    (start..lim)
        .filter(|n| factors.iter().any(|k| n % k == 0))
        .sum()
}

pub fn solution() -> u64 {
    sum_multiples(1, 1000, &[3, 5])
}
