pub fn nth (n: u32) -> u32 {
    if n == 0 {
        return 2;
    }

    let mut primes = vec![2];
    let mut candidate = 3;

    while primes.len() <= n as usize {
        if is_prime(candidate, &primes) {
            primes.push(candidate);
        }
        candidate += 2;
    }
    primes[n as usize]
}

    fn is_prime(n: u32, primes: &[u32]) -> bool {
        let sqrt_n = (n as f64).sqrt() as u32;

        for &prime in primes {
            if prime > sqrt_n {
                break;
            }
            if n % prime == 0 {
                return false;
            }
        }
        true
    }