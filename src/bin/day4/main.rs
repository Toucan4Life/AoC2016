use itertools::Itertools;
use std::str::FromStr;

struct Room {
    encrypted_name: String,
    sector_id: u32,
    checksum: String,
}

impl FromStr for Room {
    type Err = std::num::ParseIntError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (not_checksum, checksum) = line.split_once('[').unwrap();
        let (encrypted_name, sector_id) = not_checksum.rsplit_once('-').unwrap();

        Ok(Room {
            encrypted_name: encrypted_name.to_string(),
            sector_id: sector_id.parse().unwrap(),
            checksum: checksum[0..checksum.len() - 1].to_string(),
        })
    }
}

fn most_freq(inp: &str) -> String {
    inp.chars()
        .filter(|&x| x != '-')
        .into_grouping_map_by(|&x| x)
        .fold(0, |acc, _key, _value| acc + 1)
        .into_iter()
        .sorted_by_key(|item| (-item.1, item.0))
        .take(5)
        .map(|(c, _)| c)
        .collect()
}

fn main() {
    //part 1
    let code: u32 = include_str!("input.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .filter(is_real_room)
        .map(|r| r.sector_id)
        .sum();

    println!("{code}");

    //part 2
    include_str!("input.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .filter(is_real_room)
        .map(|r| (decrypt(&r), r.sector_id))
        .filter(|(name,_)| name.contains("north"))
        .for_each(|(name,sector)| println!("{name}-{sector}"));
}

fn add1_char(c: char, pos: u32) -> char {
    if c == '-' {
        return ' ';
    }
    let i = (c as u32 - 97 + pos) % 26 + 97;
    std::char::from_u32(i).unwrap_or(c)
}

fn decrypt(room: &Room) -> String {
    room.encrypted_name
        .chars()
        .map(|char| add1_char(char, room.sector_id))
        .collect()
}

fn is_real_room(room: &Room) -> bool {
    room.checksum == most_freq(&room.encrypted_name)
}

#[test]
fn test_decrypt() {
    assert_eq!(
        "very encrypted name",
        decrypt(&"qzmt-zixmtkozy-ivhz-343[abxyz]".parse::<Room>().unwrap())
    );
}

#[test]
fn test_freq() {
    assert_eq!(
        true,
        is_real_room(&"aaaaa-bbb-z-y-x-123[abxyz]".parse::<Room>().unwrap())
    );
    assert_eq!(
        true,
        is_real_room(&"a-b-c-d-e-f-g-h-987[abcde]".parse::<Room>().unwrap())
    );
    assert_eq!(
        true,
        is_real_room(&"not-a-real-room-404[oarel]".parse::<Room>().unwrap())
    );
    assert_eq!(
        false,
        is_real_room(&"totally-real-room-200[decoy]".parse::<Room>().unwrap())
    );
}
