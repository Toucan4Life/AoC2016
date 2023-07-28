use itertools::Itertools;

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl From<char> for Direction {
    fn from(s: char) -> Self {
        match s {
            'L' => Self::Left,
            'R' => Self::Right,
            'U' => Self::Up,
            'D' => Self::Down,
            _ => todo!(),
        }
    }
}

struct KeyPad {
    keys: [[String; 5]; 5],
}

impl KeyPad {
    fn new(keys: [[String; 5]; 5]) -> Self {
        Self { keys }
    }

    fn move_to(&mut self, instruction: Direction, last_place: [i32; 2]) -> [i32; 2] {
        match instruction {
            Direction::Left  => if let Some(_) = self.try_moving([last_place[0], last_place[1] - 1]) { [last_place[0], last_place[1] - 1] } else { last_place },
            Direction::Right => if let Some(_) = self.try_moving([last_place[0], last_place[1] + 1]) { [last_place[0], last_place[1] + 1] } else { last_place },
            Direction::Up    => if let Some(_) = self.try_moving([last_place[0] - 1, last_place[1]]) { [last_place[0] - 1, last_place[1]] } else { last_place },
            Direction::Down  => if let Some(_) = self.try_moving([last_place[0] + 1, last_place[1]]) { [last_place[0] + 1, last_place[1]] } else { last_place },
        }
    }

    fn try_moving(&mut self, position: [i32; 2]) -> Option<String> {
        let Some(row) = self.keys.get((position[0]) as usize) else{return None};
        let Some(digit) = row.get((position[1]) as usize) else{return None};
        if digit.is_empty() {
            return None;
        }
        return Some(digit.to_string());
    }
}

fn main() {
    let start_place = [2, 0];

    let mut key_pad = KeyPad::new([
        [String::from(""),  String::from(""),  String::from("1"), String::from(""),  String::from("")],
        [String::from(""),  String::from("2"), String::from("3"), String::from("4"), String::from("")],
        [String::from("5"), String::from("6"), String::from("7"), String::from("8"), String::from("9")],
        [String::from(""),  String::from("A"), String::from("B"), String::from("C"), String::from("")],
        [String::from(""),  String::from(""),  String::from("D"), String::from(""),  String::from("")],
    ]);

    let code = include_str!("input.txt")
        .lines()
        .map(|lines| {
            let digit_position = lines.chars().fold(start_place, |acc, direction| {
                key_pad.move_to(Direction::from(direction), acc)
            });
            return if let Some(digit) = key_pad.try_moving(digit_position) {
                digit
            } else {
                "".to_string()
            };
        })
        .join("");

    println!("{}", code);
}
