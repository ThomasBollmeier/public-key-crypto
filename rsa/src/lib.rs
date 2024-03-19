use utils::Prng;

pub struct Rsa {
    e: i64,
    p: i64,
    q: i64,
    λ_n: i64,
    d: i64,
}

impl Rsa {
    pub fn new(prng: &mut Prng, num_tests: i64) -> Self {
        let p = find_prime(prng, 1_000, 10_000, num_tests);
        let mut q: i64;
        loop {
            q = find_prime(prng, 1_000, 10_000, num_tests);
            if p != q {
                break;
            }
        }
        let λ_n = totient(p, q);
        let e = random_exponent(prng, λ_n);
        let d = inverse_mod(e, λ_n).unwrap();
        Self { e, p, q, λ_n, d }
    }

    pub fn public_key(&self) -> (i64, i64) {
        (self.p * self.q, self.e)
    }

    pub fn encrypt(&self, message: i64) -> i64 {
        fast_exp_mod(message, self.e, self.p * self.q)
    }

    pub fn decrypt(&self, cipher: i64) -> i64 {
        fast_exp_mod(cipher, self.d, self.p * self.q)
    }

    pub fn print_info(&self) {
        println!("*** Public ***");
        println!("Public key modulus: {}", self.p * self.q);
        println!("Public key exponent e: {}", self.e);
        println!();
        println!("*** Private ***");
        println!("Primes: {}, {}", self.p, self.q);
        println!("λ(n): {}", self.λ_n);
        println!("d: {}", self.d);
    }
}

pub fn find_prime(prng: &mut Prng, min: i64, max: i64, num_tests: i64) -> i64 {
    loop {
        let num = prng.next_i64(min, max);
        if is_prime(prng, num, num_tests) {
            return num;
        }
    }
}

fn is_prime(prng: &mut Prng, p: i64, num_tests: i64) -> bool {
    for _ in 0..num_tests {
        let n = prng.next_i64(2, p - 1);
        if fast_exp_mod(n, p - 1, p) != 1 {
            return false;
        }
    }
    true
}

// Calculate Carmichael's totient function λ(n)
// where n = p * q and p and q are prime.
pub fn totient(p: i64, q: i64) -> i64 {
    (p - 1) * (q - 1) / gcd(p - 1, q - 1)
}

// Pick a random exponent e in the range [3, λ_n)
// such that gcd(e, λ_n) = 1.
pub fn random_exponent(prng: &mut Prng, λ_n: i64) -> i64 {
    loop {
        let e = prng.next_i64(3, λ_n);
        if gcd(e, λ_n) == 1 {
            return e;
        }
    }
}

pub fn inverse_mod(num: i64, modulus: i64) -> Result<i64, String> {
    let mut t = 0;
    let mut newt = 1;
    let mut r = modulus;
    let mut newr = num;
    while newr != 0 {
        let quotient = r / newr;
        let temp = t - quotient * newt;
        t = newt;
        newt = temp;
        let temp = r - quotient * newr;
        r = newr;
        newr = temp;
    }
    if r > 1 {
        return Err(format!("{} is not invertible mod {}", num, modulus));
    }
    if t < 0 {
        t += modulus;
    }
    Ok(t)
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    if a < b {
        std::mem::swap(&mut a, &mut b);
    }
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn fast_exp_mod(num: i64, mut pow: i64, modulus: i64) -> i64 {
    let mut result = 1;
    let mut n = num;
    while pow > 0 {
        if pow % 2 == 1 {
            result = (result * n) % modulus;
        }
        n = (n * n) % modulus;
        pow /= 2;
    }
    result
}
