//! GCD Algorithm

use std::collections::{HashMap, HashSet};

use super::prime::PrimeFactorizable;

/// Gets the gcd of two integers
pub fn gcd(a: u32, b: u32) -> i32 {
    let mut powers_a = a.prime_factorize().generate_powers();
    let powers_b = b.prime_factorize().generate_powers();

    powers_a.extend(powers_b.iter());

    let mut powers: HashMap<u32, Vec<u32>> = HashMap::new();

    for (val, pow) in powers_a.iter() {
        powers.entry(*val).or_default().push(*pow);
    }

    let mut gcd = 1;

    for (val, power) in powers {
        let min = if power.len() == 1 {
            0
        } else {
            power.into_iter().min().unwrap_or(0)
        };

        gcd *= val.pow(min);
    }

    gcd as i32
}

/// Raise something to a power
pub trait Powerable {
    /// Raise it to a power
    fn power(&self) -> u32;
}

impl Powerable for (u32, u32) {
    fn power(&self) -> u32 {
        self.0.pow(self.1)
    }
}

/// Generate powers
pub trait Powers {
    /// Generates some powers
    fn generate_powers(&self) -> Vec<(u32, u32)>;
}

impl Powers for Vec<u32> {
    fn generate_powers(&self) -> Vec<(u32, u32)> {
        let mut seen = HashSet::new();
        let mut powers = vec![];
        for val in self {
            if !seen.contains(val) {
                let power = self.iter().filter(|v| *v == val).count();
                powers.push((*val, power as u32));

                seen.insert(*val);
            }
        }

        powers
    }
}

#[cfg(test)]
mod tests {

    use crate::math::{gcd::Powers, prime::PrimeFactorizable};

    use super::gcd;

    #[test]
    fn powers_power_properly() {
        let factorization = 100.prime_factorize().generate_powers();
        assert_eq!(factorization, &[(2, 2), (5, 2)])
    }

    #[test]
    fn test_gcd() {
        let gcd = gcd(8, 12);

        assert_eq!(gcd, 4)
    }
}
