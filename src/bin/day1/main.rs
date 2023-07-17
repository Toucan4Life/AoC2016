fn main() {
    let input: Vec<&str> = include_str!("input.txt").split(", ").collect();
    let mut dir: i32 = 0;
    let mut place = (0, 0);
    let mut visited: Vec<(i32, i32)> = vec![(0, 0)];

    'outer: for direction in input {
        let (d, s) = direction.split_at(1);
        dir = (dir + if d == "R" { 1 } else { -1 }).rem_euclid(4);

        let x: &(i32, i32) = &visited.last().cloned().unwrap();
        let parse = s.parse().expect("Should be a number");
        for i in 1..=parse {
            let t = (
                x.0 + if dir == 1 {
                    i
                } else if dir == 3 {
                    -i
                } else {
                    0
                },
                x.1 + if dir == 0 {
                    i
                } else if dir == 2 {
                    -i
                } else {
                    0
                },
            );

            if visited.contains(&t) {
                place = t;
                break 'outer;
            }
            visited.push(t);
        }
    }
    println!("Distance parcourue: {}", place.0.abs() + place.1.abs())
}
