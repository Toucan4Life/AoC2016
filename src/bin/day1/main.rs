fn main() {
    let mut dir: i32 = 0;
    let mut last_place: (i32, i32) = (0, 0);
    let mut visited: Vec<(i32, i32)> = vec![(0, 0)];
    let change_vectors = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    
    'outer: for direction in include_str!("input.txt").split(", ") {
        let (d, s) = direction.split_at(1);
        dir = (dir + if d == "R" { 1 } else { -1 }).rem_euclid(4);
        for _ in 1..=s.parse().expect("Should be a number") {
            last_place = (
                last_place.0 + change_vectors[dir as usize].0,
                last_place.1 + change_vectors[dir as usize].1,
            );

            if visited.contains(&last_place) {
                break 'outer;
            }

            visited.push(last_place);
        }
    }

    println!(
        "Distance parcourue: {}",
        last_place.0.abs() + last_place.1.abs()
    )
}
