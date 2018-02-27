pub fn sieve(limit: u64) -> Vec<u64> {
    let mut sieve: Vec<u64> = Vec::with_capacity((limit-1) as usize);
    for i in 2..limit {
        sieve.push(i);
    }

    let mut results = vec![];

    while !sieve.is_empty() {
        let n = sieve.remove(0);
        results.push(n);
        sieve.retain(|&x| x%n != 0);
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_to_50() {
        let primes_to_50: Vec<u64> = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
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
        if n > 2 {
            for i in 2..n/2 {
                if n % i == 0 {
                    return false;
                }
            }
        }
        true
    }
}
