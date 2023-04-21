// O(sqrt(n))
pub fn is_prime(n: usize) -> bool {
    if n < 2 {
        return false;
    }

    let mut i: usize = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }

        i += 1;
    }

    return true;
}

// Sieve of Eratosthenes: O(nloglogn)
pub fn get_is_prime_vec(n: usize) -> Vec<bool> {
    let mut is_prime_vec: Vec<bool> = vec![true; n + 1];

    is_prime_vec[0] = false;
    if n >= 1 {
        is_prime_vec[1] = false;
    }

    let mut i: usize = 2;
    while i * i <= n {
        if !is_prime_vec[i] {
            i += 1;
            continue;
        }

        let mut j: usize = i * i;
        while j <= n {
            is_prime_vec[j] = false;
            j += i;
        }

        i += 1;
    }

    return is_prime_vec;
}
