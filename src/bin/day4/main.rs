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

impl Room {
    fn is_real_room(&self) -> bool {
        self.checksum
            == self
                .encrypted_name
                .chars()
                .filter(|&x| x != '-')
                .into_grouping_map_by(|&x| x)
                .fold(0, |acc, _key, _value| acc + 1)
                .into_iter()
                .sorted_by_key(|item| (-item.1, item.0))
                .take(5)
                .map(|(c, _)| c)
                .collect::<String>()
    }

    fn decrypt(&self) -> String {
        self.encrypted_name
            .chars()
            .map(|char| {
                if char == '-' {
                    ' '
                } else {
                    std::char::from_u32((char as u32 - 97 + self.sector_id) % 26 + 97)
                        .unwrap()
                }
            })
            .collect()
    }
}

fn main() {
    //part 1
    let code: u32 = include_str!("input.txt")
        .lines()
        .map(|line| line.parse::<Room>().unwrap())
        .filter(|room| room.is_real_room())
        .map(|r| r.sector_id)
        .sum();

    println!("{code}");

    //part 2
    include_str!("input.txt")
        .lines()
        .map(|line| line.parse::<Room>().unwrap())
        .filter(|room| room.is_real_room() && room.decrypt().contains("north"))
        .for_each(|room| println!("{0}-{1}", room.decrypt(), room.sector_id));
}

#[test]
fn test_decrypt() {
    assert_eq!(
        "very encrypted name",
        "qzmt-zixmtkozy-ivhz-343[abxyz]"
            .parse::<Room>()
            .unwrap()
            .decrypt()
    );
}

#[test]
fn test_freq() {
    assert_eq!(
        true,
        "aaaaa-bbb-z-y-x-123[abxyz]"
            .parse::<Room>()
            .unwrap()
            .is_real_room()
    );
    assert_eq!(
        true,
        "a-b-c-d-e-f-g-h-987[abcde]"
            .parse::<Room>()
            .unwrap()
            .is_real_room()
    );
    assert_eq!(
        true,
        "not-a-real-room-404[oarel]"
            .parse::<Room>()
            .unwrap()
            .is_real_room()
    );
    assert_eq!(
        false,
        "totally-real-room-200[decoy]"
            .parse::<Room>()
            .unwrap()
            .is_real_room()
    );
}
