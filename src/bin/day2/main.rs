use std::str::FromStr;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            _ => Err(format!("Can't' parse {}", s)),
        }
    }
}

struct KeyPad {
    last_place: [i32;2],
    keys : [[String;5];5]
}

impl KeyPad {
    fn new(start_position: [i32;2]) -> Self {
        Self {
            last_place: start_position,
            keys : [[String::from(""),String::from(""),String::from("1"),String::from(""),String::from("")],
                    [String::from(""),String::from("2"),String::from("3"),String::from("4"),String::from("")],
                    [String::from("5"),String::from("6"),String::from("7"),String::from("8"),String::from("9")],
                    [String::from(""),String::from("A"),String::from("B"),String::from("C"),String::from("")],
                    [String::from(""),String::from(""),String::from("D"),String::from(""),String::from("")]]
        }
    }
    fn move_to(&mut self, instruction: Direction) {
        match instruction {
            Direction::Left => {
                if self.last_place[1] != 0 && !self.keys.get(self.last_place[0] as usize).unwrap().get((self.last_place[1] - 1) as usize).unwrap().is_empty() {
                    self.last_place[1] -= 1
                }
            }
            Direction::Right => {
                if self.last_place[1] != 4 && !self.keys.get(self.last_place[0] as usize).unwrap().get((self.last_place[1] + 1) as usize).unwrap().is_empty() {
                    self.last_place[1] += 1
                }
            }
            Direction::Up => {
                if self.last_place[0] != 0 && !self.keys.get((self.last_place[0] - 1) as usize).unwrap().get(self.last_place[1] as usize).unwrap().is_empty() {
                    self.last_place[0] -= 1
                }
            }
            Direction::Down => {
                if self.last_place[0] != 4 && !self.keys.get((self.last_place[0] + 1) as usize).unwrap().get(self.last_place[1] as usize).unwrap().is_empty() {
                    self.last_place[0] += 1
                }
            }
        }
    }
}

fn main() {
    let mut key_pad = KeyPad::new([2,0]);
    for lines in include_str!("input.txt").lines() {
        for direction in lines.chars() {
            key_pad.move_to(Direction::from_str(&direction.to_string()).unwrap())
        }
        println!("digit: {}", key_pad.keys.get(key_pad.last_place[0] as usize).unwrap().get(key_pad.last_place[1] as usize).unwrap())
    }
}
