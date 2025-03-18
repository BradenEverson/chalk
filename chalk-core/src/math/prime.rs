//! Prime number formula

/// A prime generation iterator
#[derive(Default)]
pub struct PrimeMachine {
    /// The cache of previous primes for deriving new primes
    cache: Vec<usize>,
}

impl Iterator for PrimeMachine {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let next = if self.cache.is_empty() {
            2
        } else {
            let mut curr = self.cache[self.cache.len() - 1] + 1;

            while self.cache.iter().any(|val| curr % *val == 0) {
                curr += 1;
            }

            curr
        };

        self.cache.push(next);
        Some(next)
    }
}

/// Checking if a number is prime
pub trait PrimeCheck {
    /// Prime check with an existent prime machine
    fn is_prime_with_machine(&self, primes: &mut PrimeMachine) -> bool;
    /// Prime check with a new prime machine
    fn is_prime(&self) -> bool {
        let mut primes = PrimeMachine::default();
        self.is_prime_with_machine(&mut primes)
    }
}

impl PrimeCheck for usize {
    fn is_prime_with_machine(&self, primes: &mut PrimeMachine) -> bool {
        let num_sqrt = (*self as f32).sqrt() as usize;

        for prime in primes.by_ref() {
            if prime > num_sqrt {
                break;
            } else if self % prime == 0 {
                return false;
            }
        }

        true
    }
}

/// Any type that can be prime factorized
pub trait PrimeFactorizable {
    /// Generates the prime factors of a number
    fn prime_factorize(&self) -> Vec<usize>;
}

impl PrimeFactorizable for usize {
    fn prime_factorize(&self) -> Vec<usize> {
        let mut curr = *self;
        let mut factors = vec![];

        while curr != 1 {
            let curr_sqrt = f32::sqrt(curr as f32).ceil() as usize;
            let primes = PrimeMachine::default();

            for prime in primes {
                if prime > curr_sqrt {
                    factors.push(curr);
                    curr /= curr;
                    break;
                } else if curr % prime == 0 {
                    factors.push(prime);
                    curr /= prime;
                    break;
                }
            }
        }

        factors.sort();
        factors
    }
}

#[cfg(test)]
mod tests {
    use std::usize;

    use crate::math::prime::PrimeFactorizable;

    #[test]
    fn prime_factorization() {
        let factors = 8976986.prime_factorize();
        assert_eq!(factors, &[2, 17, 264029]);
    }

    #[test]
    fn prime_numbers_factorize_to_themselves() {
        let factors = 3.prime_factorize();
        assert_eq!(factors, &[3]);
    }

    #[test]
    fn prime_factorize_max_usize() {
        let factors = usize::MAX.prime_factorize();
        assert_eq!(factors, &[3, 5, 17, 257, 641, 65537, 6700417]);
    }
}
