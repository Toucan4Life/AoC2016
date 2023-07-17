use std::collections::HashSet;

fn main() {
    let places = include_str!("input.txt")
        .split(", ")
        .fold((0, vec![(0, 0)]), |(last_dir, places), direction| {
            walk(direction, last_dir, places)
        })
        .1;

    let mut place: (i32, i32) = places
        .last()
        .cloned()
        .expect("The should be at least one place");
    println!("Distance parcourue: {}", place.0.abs() + place.1.abs());

    let mut unique_ports = HashSet::new();
    for visit in places {
        if !unique_ports.insert(visit) {
            place = visit;
            break;
        }
    }

    println!("Distance parcourue: {}", place.0.abs() + place.1.abs());
}

fn walk(direction: &str, dir: i32, mut visited: Vec<(i32, i32)>) -> (i32, Vec<(i32, i32)>) {
    let (d, s) = direction.split_at(1);
    let dir = (dir + if d == "R" { 1 } else { -1 }).rem_euclid(4);
    let last_place = visited.last().cloned().unwrap();
    let mut vis: Vec<(i32, i32)> = (1..=s.parse().expect("Should be a number"))
        .map(|i| {
            (
                last_place.0
                    + if dir == 1 {
                        i
                    } else if dir == 3 {
                        -i
                    } else {
                        0
                    },
                last_place.1
                    + if dir == 0 {
                        i
                    } else if dir == 2 {
                        -i
                    } else {
                        0
                    },
            )
        })
        .collect();
    visited.append(&mut vis);
    (dir, visited)
}
