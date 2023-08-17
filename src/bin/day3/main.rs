use itertools::Itertools;

fn main() {
    //part 1
    let code = include_str!("input.txt")
        .lines()
        .map(|line| {
            [2, 7, 12]
                .into_iter()
                .map(|i| line[i..i + 3].trim_start().parse::<i32>().unwrap())
                .sorted()
                .collect()
        })
        .filter(|row: &Vec<_>| row[0] + row[1] > row[2])
        .count();

    println!("{code}");

    //part 2

    let code = include_str!("input.txt")
        .lines()
        .map(|line| {
            [2, 7, 12]
                .into_iter()
                .map(|i| line[i..i + 3].trim_start().parse::<i32>().unwrap())
                .collect()
        })
        .tuples()
        .flat_map(|(a, b, c)| transpose(vec![a, b, c]))
        .filter(|row2| {
            let mut row = row2.clone();
            row.sort();
            row[0] + row[1] > row[2]
        })
        .count();

    println!("{code}");
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}
