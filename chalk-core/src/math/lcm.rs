//! LCD Algorithm

use std::collections::HashMap;

use super::gcd::Powers;
use super::prime::PrimeFactorizable;

/// Gets the least common multiple of two integers
pub fn lcm(a: usize, b: usize) -> usize {
    let mut powers_a = a.prime_factorize().generate_powers();
    let powers_b = b.prime_factorize().generate_powers();

    powers_a.extend(powers_b.iter());

    let mut powers: HashMap<usize, Vec<usize>> = HashMap::new();

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

    lcm
}

#[cfg(test)]
mod tests {
    use super::lcm;

    #[test]
    fn crazy_lcm() {
        let lcm = lcm(3059423, 23905102);
        assert_eq!(lcm, 73135818876146)
    }

    #[test]
    fn simple_lcm() {
        let lcm = lcm(12, 15);
        assert_eq!(lcm, 60)
    }
}
