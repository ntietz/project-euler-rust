pub fn solution() -> u64 {
    solve(100)
}

fn solve(limit: u64) -> u64 {
    let sum_of_squares = (1..=limit).map(|n| n * n).sum::<u64>();
    let square_of_sum = (1..=limit).sum::<u64>().pow(2);

    square_of_sum - sum_of_squares
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verifies_example() {
        assert_eq!(solve(10), 2640);
    }
}
