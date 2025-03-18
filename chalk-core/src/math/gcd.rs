//! GCD Algorithm

use std::collections::{HashMap, HashSet};

use super::prime::PrimeFactorizable;

/// Gets the gcd of two integers
pub fn gcd(a: usize, b: usize) -> usize {
    let mut powers_a = a.prime_factorize().generate_powers();
    let powers_b = b.prime_factorize().generate_powers();

    powers_a.extend(powers_b.iter());

    let mut powers: HashMap<usize, Vec<usize>> = HashMap::new();

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

        gcd *= val.pow(min as u32);
    }

    gcd
}

/// Raise something to a power
pub trait Powerable {
    /// Raise it to a power
    fn power(&self) -> usize;
}

impl Powerable for (usize, usize) {
    fn power(&self) -> usize {
        self.0.pow(self.1 as u32)
    }
}

/// Generate powers
pub trait Powers {
    /// Generates some powers
    fn generate_powers(&self) -> Vec<(usize, usize)>;
}

impl Powers for Vec<usize> {
    fn generate_powers(&self) -> Vec<(usize, usize)> {
        let mut seen = HashSet::new();
        let mut powers = vec![];
        for val in self {
            if !seen.contains(val) {
                let power = self.iter().filter(|v| *v == val).count();
                powers.push((*val, power));

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

    #[test]
    fn test_crazy_gcd() {
        let gcd = gcd(423123409190, 123409190);

        assert_eq!(gcd, 10)
    }
}
