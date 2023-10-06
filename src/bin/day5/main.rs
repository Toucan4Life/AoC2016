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

fn decrypt2(input: String) -> String {
    let mut decrypted: Vec<char> = "XXXXXXXX".chars().collect();
    for counter in 0.. {
        let hash = format!("{:x}", md5::compute(input.clone() + &counter.to_string()));
        if hash.starts_with("00000") {
            let pos = hash.chars().nth(5).unwrap();
            if ['0', '1', '2', '3', '4', '5', '6', '7'].contains(&pos) {
                let ch = hash.chars().nth(6).unwrap();
                let posint = pos.to_digit(10).unwrap() as usize;
                if decrypted[posint] == 'X' {
                    decrypted[posint] = ch;
                    println!("found char : {}", decrypted.iter().collect::<String>());
                }
            }
        }
        if !decrypted.contains(&'X') {
            break;
        }
    }
    decrypted.iter().collect()
}

fn main() {
    let decrypted = decrypt2(include_str!("input.txt").to_string());
    println!("{decrypted}");
}

#[test]
fn test_decrypt() {
    assert_eq!("18f47a30", decrypt("abc".to_string()));
}

#[test]
fn test_decrypt2() {
    assert_eq!("05ace8e3", decrypt2("abc".to_string()));
}
