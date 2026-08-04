use rand::{RngExt, SeedableRng, rngs::StdRng};

const ORI_KEY_STR: &str = "maXxZ70:#;BDLO_[raEb@Rned|-.3tMp/Dh9*Til^=k1!";

pub fn decrypt(cipher: &str, key: &str, loops: usize) -> String {
    let mut user_key_sum = 0;

    for c in key.chars() {
        user_key_sum += c as u32;
    }

    let user_key_sum = (user_key_sum % 95 + 32) as u8;
    let mut cipher = cipher.clone();

    if cipher.chars().nth(0).unwrap() == user_key_sum as char {
        cipher = &cipher[1..];
    }

    let mut rand_nums = Vec::new();
    for i in 0..loops {}

    for (i, c) in cipher.chars().enumerate() {
        rand_nums.push(c as u8);
        if i > loops {
            break;
        }
    }

    cipher = &cipher[loops..];
    let mut jh_key = ORI_KEY_STR.to_string();

    for encry_loop in 1..=loops {
        jh_key = jh_key
            .chars()
            .map(|c| {
                if encry_loop % 2 != 0 {
                    ((c as u8 + cipher.len() as u8 + rand_nums[encry_loop - 1]) % 95 + 32) as char
                } else {
                    ((c as u8 + rand_nums[encry_loop - 1]) % 95 + 32) as char
                }
            })
            .collect::<String>();
    }

    let mut plain = String::new();
    for (i, c) in cipher.chars().enumerate() {
        let k = jh_key.chars().nth(i % jh_key.len()).unwrap() as i32 - 32;
        let m = c as i32 - 32;
        let res = (m - k + 95) % 95 + 32;
        plain.push(res as u8 as char);
    }

    plain
}

pub fn encrypt(plain: &str, key: &str, loops: usize, seed: u64) -> String {
    let mut jh_key = ORI_KEY_STR.to_string();
    let mut user_key_sum = 0;
    let mut rng = StdRng::seed_from_u64(seed);

    for c in key.chars() {
        user_key_sum += c as u32;
    }

    let user_key_sum = (user_key_sum % 95 + 32) as u8;

    let mut rand_nums = Vec::new();
    for encry_loop in 1..=loops {
        let num = rng.random_range(32..95) as u8;
        rand_nums.push(num);

        jh_key = jh_key
            .chars()
            .map(|c| {
                if encry_loop % 2 != 0 {
                    ((c as u8 + plain.len() as u8 + num) % 95 + 32) as char
                } else {
                    ((c as u8 + num) % 95 + 32) as char
                }
            })
            .collect::<String>();
    }

    let mut cipher = String::new();
    for (i, c) in plain.chars().enumerate() {
        let k = jh_key.chars().nth(i % jh_key.len()).unwrap() as u8 - 32;
        let m = c as u8 - 32;
        cipher.push(((m + k) % 95 + 32) as char);
    }

    for n in rand_nums {
        cipher.insert(0, n as char);
    }

    cipher.insert(0, user_key_sum as char);
    cipher
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let actual = encrypt("hello world", "foobarbaz", 10, 5);
        let expected = " O/IT<@ +5+YJHhM:+-x+*";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_decode() {
        let actual = decrypt(" 27]e@>X5>vC42R7$tvbts", "foobarbaz", 10);
        let expected = "hello world";
        assert_eq!(actual, expected);
    }
}
