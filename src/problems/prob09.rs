pub fn solution() -> u64 {
    for a in 1..=999 {
        for b in a..=1000 {
            if a + b < 1000 && is_pythagorean_triplet(a, b, 1000 - a - b) {
                return a * b * (1000 - a - b);
            }
        }
    }
    0
}

fn is_pythagorean_triplet(a: u64, b: u64, c: u64) -> bool {
    if b < a {
        is_pythagorean_triplet(b, a, c)
    } else if c < b {
        is_pythagorean_triplet(a, c, b)
    } else {
        (a * a + b * b == c * c) && (a < b) && (b < c)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_triplets() {
        assert_eq!(true, is_pythagorean_triplet(3, 4, 5));
        assert_eq!(true, is_pythagorean_triplet(3, 5, 4));
        assert_eq!(true, is_pythagorean_triplet(4, 3, 5));
        assert_eq!(true, is_pythagorean_triplet(5, 3, 4));
        assert_eq!(true, is_pythagorean_triplet(4, 5, 3));
        assert_eq!(true, is_pythagorean_triplet(5, 4, 3));
        assert_eq!(false, is_pythagorean_triplet(3, 4, 6));
        assert_eq!(false, is_pythagorean_triplet(4, 5, 6));
    }
}
