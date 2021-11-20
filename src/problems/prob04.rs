pub fn solution() -> u64 {
    let mut palindrome = 12321;

    for i in 100..999 {
        for j in 100..999 {
            let candidate = i * j;
            if candidate > palindrome && is_palindrome(candidate) {
                palindrome = candidate;
            }
        }
    }

    palindrome
}

fn is_palindrome(n: u64) -> bool {
    let mut digits: Vec<u8> = vec![];
    let mut n = n;

    while n > 0 {
        digits.push((n % 10) as u8);
        n /= 10;
    }

    let midpoint = digits.len() / 2;

    for i in 0..midpoint {
        if digits[i] != digits[digits.len() - i - 1] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verifies_palindromes() {
        assert_eq!(is_palindrome(99), true);
        assert_eq!(is_palindrome(101), true);
        assert_eq!(is_palindrome(12321), true);
    }

    #[test]
    fn rejects_non_palindromes() {
        assert_eq!(is_palindrome(991), false);
        assert_eq!(is_palindrome(10), false);
        assert_eq!(is_palindrome(1234), false);
    }
}
