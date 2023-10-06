// use itertools::Itertools;
// use std::str::FromStr;

fn decrypt(input: String) -> String {
    let mut decrypted = "".to_string();
    for counter in 0.. {
        let hash = format!("{:x}", md5::compute(input.clone() + &counter.to_string()));
        if hash.starts_with("00000") {
            let ch = hash.chars().nth(5).unwrap();
            println!("found char : {ch}");
            decrypted.push(ch);
        }
        if decrypted.len() == 8 {
            break;
        }
    }
    decrypted
}

fn main() {
    let decrypted = decrypt(include_str!("input.txt").to_string());
    println!("{decrypted}");
}

#[test]
fn test_decrypt() {
    assert_eq!("18f47a30", decrypt("abc".to_string()));
}
