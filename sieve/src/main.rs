use utils::get_i64;

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

// Print out the primes in the sieve.
fn print_sieve(sieve: &Vec<bool>) {
    let size = sieve.len();

    print!("2 ");

    for i in (3..size).step_by(2) {
        if sieve[i] {
            print!("{i} ");
        }
    }
    println!();
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

// Print the vector of numbers.
fn print_numbers(primes: &Vec<i64>) {
    for prime in primes {
        print!("{prime} ");
    }
    println!();
}

fn main() {
    let max = get_i64("Max: ");
    let sieve = sieve_of_eratosthenes(max as usize);
    if max < 1000 {
        print_sieve(&sieve);
    }

    let primes = sieve_to_primes(&sieve);
    if max < 1000 {
        print_numbers(&primes);
    }
}
