use rsa::Rsa;
use utils::{get_i64, Prng};

const NUM_TESTS: i64 = 20;

fn main() {
    let mut prng = Prng::new();
    let rsa = Rsa::new(&mut prng, NUM_TESTS);

    rsa.print_info();

    let (n, _) = rsa.public_key();

    loop {
        let m = get_i64(&format!("Message number in [2..{}]: ", n - 2));
        if m == 0 {
            break;
        }
        let cipher = rsa.encrypt(m);
        println!("Ciphertext: {}", cipher);
        let plain = rsa.decrypt(cipher);
        println!("Plaintext: {}", plain);
        println!()
    }
}
