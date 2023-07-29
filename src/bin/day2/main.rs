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

struct KeyPad<'a> {
    keys: [[Option<&'a str>; 5]; 5],
}

impl KeyPad<'_> {
    fn new(keys: [[Option<&str>; 5]; 5]) -> Self {
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

    fn try_moving(&mut self, position: [i32; 2]) -> Option<&str> {
        return *self.keys.get((position[0]) as usize)?.get((position[1]) as usize)?;
    }
}

fn main() {
    let start_place = [2, 0];

    let mut key_pad = KeyPad::new([
        [None,  None,  Some("1"), None,  None],
        [None,  Some("2"), Some("3"), Some("4"), None],
        [Some("5"), Some("6"), Some("7"), Some("8"), Some("9")],
        [None,  Some("A"),Some("B"),Some("C"), None],
        [None,  None,  Some("D"), None,  None],
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
                ""
            };
        })
        .join("");

    println!("{}", code);
}
