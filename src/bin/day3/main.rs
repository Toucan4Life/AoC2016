use itertools::Itertools;

fn main() {
    let code = include_str!("input.txt")
        .lines()
        .filter(|line| {
            let vec: Vec<i32> = [2, 7, 12]
                .into_iter()
                .map(|i| line[i..i + 3].trim_start().parse().unwrap())
                .sorted()
                .collect();
            return vec[0] + vec[1] > vec[2];
        })
        .count();

    println!("{code}");
}
