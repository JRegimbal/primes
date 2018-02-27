use std::collections::BTreeMap;

struct Squares {
    count: u64,
    max: u64,
}

impl Squares {
    fn new(max: u64) -> Squares {
        Squares { count: 0, max: max }
    }
}

impl Iterator for Squares {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        let square = self.count*self.count;
        if square > self.max {
            None
        } else {
            Some(square)
        }
    }
}

pub fn sieve(limit: u64) -> Vec<u64> {
    let mut results = vec![2,3,5];
    let mut sieve: BTreeMap<u64, bool> = BTreeMap::new();
    if limit < 2 {
        return vec![];
    }

    // initialize the list
    for i in 2..limit {
        sieve.insert(i, false);
    }
    
    // mod 60 marking
    for (value, is_prime) in sieve.iter_mut() {
        let case: u8 = match value % 60 {
            1 | 13 | 17 | 29 | 37 | 41 | 49 | 53    => 1,
            7 | 19 | 31 | 43                        => 2,
            11 | 23 | 47 | 59                       => 3,
            _                                       => 4,
        };
        
        let (coeff_x, coeff_y): (u64, u64) = match case {
            1       => (4,1),
            2 | 3   => (3,1),
            _       => { continue; },
        };

        let expr: Box<Fn(u64, u64) -> u64> = match case {
            3   => Box::new( |x, y| { if x > y { coeff_x * x - coeff_y * y } else { 0 } } ),
            _   => Box::new( |x, y| { coeff_x*x + coeff_y*y } ),
        };

        let n = *value;
        for i in Squares::new(n) {
            for j in Squares::new(n) {
                if expr(i, j) == n {
                    *is_prime ^= true;
                }
            }
        }
    }

    // begin sieving
    while !sieve.is_empty() {
        let vals: Vec<u64> = sieve.keys().cloned().collect();
        for key in vals.iter() {
            if *sieve.get(&key).unwrap() == false {
                sieve.remove(&key);
            } else {
                results.push(*key);
                let square = key*key;
                for i in 1.. {
                    if i*square > *vals.last().unwrap() {
                        break;
                    } else {
                        if let Some(x) = sieve.get_mut(&(i*square)) {
                            *x = false;
                        }
                    }
                }
                sieve.remove(&key);
                break;
            }
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_to_50() {
        let primes_to_50 = [2u64, 3u64, 5u64, 7u64, 11u64, 13u64, 17u64, 19u64, 23u64, 29u64, 31u64, 37u64, 41u64, 43u64, 47u64];
        let results = sieve(50);
        assert_eq!(results, primes_to_50);
    }

    #[test]
    fn verify_output() {
        let results = sieve(1000);
        for prime in results.iter() {
            assert!(is_n_prime(*prime), "Failed with {}", prime);
        }
    }

    fn is_n_prime(n: u64) -> bool {
        let mut is_prime: bool = true;
        if n > 2 {
            for i in 2..n/2 {
                if n % i == 0 {
                    is_prime = false;
                    break;
                }
            }
        }
        is_prime
    }
}
