use itertools::Itertools;

fn most_freq(inp: &str) -> String {
    inp.chars()
        .filter(|&x| x != '-')
        .fold(vec![0; 26], |mut acc, c| {
            let idx = c as u8 - 'a' as u8;
            acc[idx as usize] += 1;
            acc
        })
        .into_iter()
        .enumerate()
        .map(|(i, freq)| ((i as u8 + 'a' as u8) as char, freq))
        .sorted_by(|a, b| Ord::cmp(&b.1, &a.1))
        .into_iter()
        .take(5)
        .map(|(c, _)| c)
        .collect()
}

fn main() {
    //part 1
    let code = include_str!("input.txt")
        .lines()
        .filter(|line| {
            let (mut room, mut check) = line.split('[').collect_tuple().unwrap();
            room = room.split(char::is_numeric).collect_vec()[0];
            check = &check[0..check.len() - 1];
            let result = most_freq(room);
            result == check
        })
        .count();

    println!("{code}");
}
