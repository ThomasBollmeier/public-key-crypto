use std::time::Instant;
use utils::get_i64;

fn find_factors(num: i64) -> Vec<i64> {
    let mut rest = num;
    let mut factors = vec![];

    let mut factor = 2;
    while rest % factor == 0 {
        factors.push(factor);
        rest /= factor;
    }

    factor = 3;
    while factor * factor <= rest {
        while rest % factor == 0 {
            factors.push(factor);
            rest /= factor;
        }
        factor += 2;
    }

    if rest != 1 {
        factors.push(rest);
    }

    factors
}

fn multiply_vector(numbers: &Vec<i64>) -> i64 {
    numbers.iter().fold(1, |acc, x| acc * x)
}

// Build a sieve of Eratosthenes.
fn sieve_of_eratosthenes(max: usize) -> Vec<bool> {
    let mut numbers = vec![true; max + 1];
    numbers[0] = false;
    numbers[1] = false;

    let mut start = 2;

    while let Some(prime) = find_next_prime(start, max, &numbers) {
        let mut i = 2 * prime;
        while i <= max {
            numbers[i] = false;
            i += prime;
        }
        start = prime + 1;
    }

    numbers
}

fn find_next_prime(start: usize, max: usize, numbers: &Vec<bool>) -> Option<usize> {
    let mut i = start;

    while i <= max {
        if numbers[i] {
            return Some(i);
        }
        i += 1;
    }

    None
}

// Convert the sieve into a vector holding prime numbers.
fn sieve_to_primes(sieve: &Vec<bool>) -> Vec<i64> {
    sieve
        .iter()
        .enumerate()
        .filter(|(_, is_prime)| **is_prime)
        .map(|entry| entry.0 as i64)
        .collect()
}

fn create_primes(max: usize) -> Vec<i64> {
    let sieve = sieve_of_eratosthenes(max);
    sieve_to_primes(&sieve)
}

fn find_factors_sieve(num: i64, primes: &Vec<i64>) -> Vec<i64> {
    let mut rest = num;
    let mut factors = vec![];

    for prime in primes.iter() {
        if prime * prime > rest {
            break;
        }
        while rest % prime == 0 {
            factors.push(*prime);
            rest /= prime;
        }
        if rest == 1 {
            break;
        }
    }

    if rest != 1 {
        factors.push(rest);
    }

    factors
}

fn main() {
    let mut start = Instant::now();
    let primes = create_primes(100_000_000);
    let mut duration = start.elapsed();
    println!("Time to create primes: {:?}", duration);

    loop {
        let num = get_i64("Enter a number: ");
        if num <= 0 {
            break;
        }
        start = Instant::now();
        let factors = find_factors(num);
        duration = start.elapsed();
        println!("Runtime find_factors: {:?}", duration);
        println!(
            "Factors of {num}: {factors:?}",
            num = num,
            factors = factors
        );
        println!("Multiplied factors: {}", multiply_vector(&factors));

        start = Instant::now();
        let factors = find_factors_sieve(num, &primes);
        duration = start.elapsed();
        println!("Runtime find_factors_sieve: {:?}", duration);

        println!(
            "Factors of {num}: {factors:?}",
            num = num,
            factors = factors
        );
        println!("Multiplied factors: {}", multiply_vector(&factors));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_factors() {
        let primes = create_primes(100_000_000);
        assert_eq!(
            find_factors_sieve(312680865509917, &primes),
            vec![7791799, 40129483]
        );
        assert_eq!(
            find_factors_sieve(1819448968910731, &primes),
            vec![40129483, 45339457]
        );
        assert_eq!(
            find_factors_sieve(12345678901234, &primes),
            vec![2, 7, 73, 12079920647]
        );
        assert_eq!(
            find_factors_sieve(6795742697625173, &primes),
            vec![6880691, 987654103]
        );
        assert_eq!(
            find_factors_sieve(64374108854777, &primes),
            vec![64374108854777]
        );
    }
}
