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
    last_place: i32,
}

impl KeyPad {
    fn new(start_position: i32) -> Self {
        Self {
            last_place: start_position,
        }
    }
    fn move_to(&mut self, instruction: Direction) {
        match instruction {
            Direction::Left => {
                if (self.last_place % 3) - 1 != 0 {
                    self.last_place -= 1
                }
            }
            Direction::Right => {
                if (self.last_place % 3) + 1 != 1 {
                    self.last_place += 1
                }
            }
            Direction::Up => {
                if (self.last_place - 3) > 0 {
                    self.last_place -= 3
                }
            }
            Direction::Down => {
                if (self.last_place + 3) < 10 {
                    self.last_place += 3
                }
            }
        }
    }
}

fn main() {
    let mut key_pad = KeyPad::new(5);
    for lines in include_str!("input.txt").lines() {
        for direction in lines.chars() {
            key_pad.move_to(Direction::from_str(&direction.to_string()).unwrap())
        }
        println!("digit: {}", key_pad.last_place)
    }
}
