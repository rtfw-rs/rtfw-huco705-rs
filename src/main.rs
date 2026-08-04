use huco705_rs::{decrypt, encrypt};

fn main() {
    let plain = "hello world!";
    let key = "this is a secret key";
    println!("Encrypting `{plain}` with key `{key}`...");

    let cipher = encrypt(plain, key, 10, 10);
    println!("encrypted: {cipher}");

    let decrypted = decrypt(&cipher, key, 10);
    println!("decrypted: {decrypted}");
}
