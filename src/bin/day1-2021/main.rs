use itertools::Itertools;

fn main() {
    let t = include_str!("input.txt")
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect_vec();

    println!("{}", process_input(t));

    let x = include_str!("input.txt")
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect_vec();

    println!("{}", process_input2(x));
}

fn process_input(input: Vec<i32>) -> i32 {
    input
        .windows(2)
        .map(|cur| if cur[0] < cur[1] { 1 } else { 0 })
        .sum()
}

fn process_input2(input: Vec<i32>) -> i32 {
    input
        .windows(3)
        .collect_vec()
        .windows(2)
        .map(|cur| {
            if cur[0].iter().sum::<i32>() < cur[1].iter().sum() {
                1
            } else {
                0
            }
        })
        .sum()
}

#[test]
fn test() {
    let input = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    assert_eq!(7, process_input(input.to_vec()))
}

#[test]
fn test2() {
    let input = [607, 618, 618, 617, 647, 716, 769, 792];

    assert_eq!(5, process_input2(input.to_vec()))
}
