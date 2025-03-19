//! LCD Algorithm

use std::collections::HashMap;

use super::gcd::Powers;
use super::prime::PrimeFactorizable;

/// Gets the least common multiple of two integers
pub fn lcm(a: u32, b: u32) -> i32 {
    let mut powers_a = a.prime_factorize().generate_powers();
    let powers_b = b.prime_factorize().generate_powers();

    powers_a.extend(powers_b.iter());

    let mut powers: HashMap<u32, Vec<u32>> = HashMap::new();

    for (val, pow) in powers_a.iter() {
        powers.entry(*val).or_default().push(*pow);
    }

    let mut lcm = 1;

    for (val, power) in powers {
        let max = if power.len() == 1 {
            power[0]
        } else {
            power.into_iter().max().unwrap_or(0)
        };

        lcm *= val.pow(max as u32);
    }

    lcm as i32
}

#[cfg(test)]
mod tests {
    use super::lcm;

    #[test]
    fn simple_lcm() {
        let lcm = lcm(12, 15);
        assert_eq!(lcm, 60)
    }
}
