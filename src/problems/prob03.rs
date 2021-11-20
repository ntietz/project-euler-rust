use crate::math::factors::prime_factors;

pub fn solution() -> u64 {
    let factors = prime_factors(600851475143);
    factors[factors.len() - 1]
}
