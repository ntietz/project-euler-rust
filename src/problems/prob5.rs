pub fn solution() -> u64 {
    solve(20)
}

fn solve(limit: u64) -> u64 {
    let mut nums: Vec<u64> = (1..limit).collect();

    for i in 0..nums.len() {
        if nums[i] == 1 {
            continue;
        }

        for j in (i + 1)..nums.len() {
            if nums[j] % nums[i] == 0 {
                nums[j] /= nums[i];
            }
        }
    }

    nums.iter().product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verifies_example() {
        assert_eq!(solve(10), 2520);
    }
}
