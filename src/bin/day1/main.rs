fn main() {
    let input: Vec<&str> = include_str!("input.txt").split(", ").collect();
    let mut dir: i32 = 0;
    let mut distance: [i32; 4] = [0; 4];

    for direction in input {
        let (d, s) = direction.split_at(1);

        dir = (dir + if d == "R" { 1 } else { -1 }).rem_euclid(4);

        distance[dir as usize] += s.parse::<i32>().expect("Should be a number")
    }

    println!(
        "Distance parcourue: {}",
        (distance[0]).abs_diff(distance[2]) + (distance[1]).abs_diff(distance[3])
    )
}
