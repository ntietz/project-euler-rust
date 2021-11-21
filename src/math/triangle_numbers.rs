pub struct TriangleNumbers {
    sum: u64,
    nat: u64,
}

impl TriangleNumbers {
    pub fn new() -> Self {
        TriangleNumbers { sum: 0, nat: 0 }
    }
}

impl Default for TriangleNumbers {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for TriangleNumbers {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.nat += 1;
        self.sum += self.nat;

        Some(self.sum)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generates_first_ten() {
        let expected: Vec<u64> = vec![1, 3, 6, 10, 15, 21, 28, 36, 45, 55];

        let triangles = TriangleNumbers::new();
        let actual: Vec<_> = triangles.take(10).collect();

        assert_eq!(expected, actual);
    }
}
