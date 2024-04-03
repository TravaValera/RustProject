fn mod_pow(base: i64, exp: i64, modulus: i64) -> i64 {
    let mut base = base % modulus;
    let mut exp = exp;
    let mut result = 1;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        base = base * base % modulus;
        exp = exp >> 1;
    }
    result
}

fn encrypt(message: i64, public_key: i64) -> i64 {
    mod_pow(message, 2, public_key)
}

fn decrypt(ciphertext: i64, private_key: (i64, i64)) -> (i64, i64) {
    let m1 = mod_pow(ciphertext, (private_key.0 + 1) / 4, private_key.0);
    let m2 = private_key.0 - m1;
    (m1, m2)
}

fn main() {
    let public_key = 11 * 13;
    let private_key = (11, 13);
    let message = 7;
    let ciphertext = encrypt(message, public_key);
    let (decrypted_1, decrypted_2) = decrypt(ciphertext, private_key);
    println!("Original message: {}", message);
    println!("Encrypted message: {}", ciphertext);
    println!("Decrypted message: {} and {}", decrypted_1, decrypted_2);
}

