use std::{collections::HashMap, ops::Add, str::FromStr};

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => Err(format!("Can not parse {}", s)),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq,Hash)]
struct Vector {
    x: i32,
    y: i32,
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Vector {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum CardinalPoint {
    North,
    East,
    South,
    West,
}

impl From<CardinalPoint> for Vector {
    fn from(card: CardinalPoint) -> Self {
        match card {
            CardinalPoint::North => Vector { x: 0, y: 1 },
            CardinalPoint::East => Vector { x: 1, y: 0 },
            CardinalPoint::South => Vector { x: 0, y: -1 },
            CardinalPoint::West => Vector { x: -1, y: 0 },
        }
    }
}

struct Walker {
    visited: Vec<Vector>,
    orientation: CardinalPoint,
    last_place: Vector,
}

impl Walker {
    fn new(card_point: CardinalPoint) -> Self {
        Self {
            orientation: card_point,
            visited: vec![Vector { x: 0, y: 0 }],
            last_place: Vector { x: 0, y: 0 },
        }
    }
    fn turn(&mut self, dir: Direction) {
        match (&self.orientation, dir) {
            (CardinalPoint::North, Direction::Left) => self.orientation = CardinalPoint::West,
            (CardinalPoint::North, Direction::Right) => self.orientation = CardinalPoint::East,
            (CardinalPoint::East, Direction::Left) => self.orientation = CardinalPoint::North,
            (CardinalPoint::East, Direction::Right) => self.orientation = CardinalPoint::South,
            (CardinalPoint::South, Direction::Left) => self.orientation = CardinalPoint::East,
            (CardinalPoint::South, Direction::Right) => self.orientation = CardinalPoint::West,
            (CardinalPoint::West, Direction::Left) => self.orientation = CardinalPoint::South,
            (CardinalPoint::West, Direction::Right) => self.orientation = CardinalPoint::North,
        }
    }
    fn walk(&mut self, dist: i32) {
        for _ in 1..=dist {
            self.last_place = self.last_place + Vector::from(self.orientation);
            self.visited.push(self.last_place)
        }
    }
}

fn main() {
    let mut walker = Walker::new(CardinalPoint::North);
    for direction in include_str!("input.txt").split(", ") {
        let (d, s) = direction.split_at(1);
        walker.turn(d.parse().expect("Should be left or right"));
        walker.walk(s.parse().expect("Should be a number"));
    }

    let mut hash = HashMap::new();
    for visit in walker.visited {
        if hash.contains_key(&visit) {
            println!(
                "Distance parcourue pour duplicate: {}",
                visit.x.abs() + visit.y.abs()
            );
            break;
        }
        hash.insert(visit, 1);
    }
    println!(
        "Distance parcourue: {}",
        walker.last_place.x.abs() + walker.last_place.y.abs()
    )
}
