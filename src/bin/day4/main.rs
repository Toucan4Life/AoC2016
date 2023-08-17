use itertools::Itertools;

fn most_freq(inp: &str) -> String {
    inp.chars()
        .filter(|&x| x != '-')
        .into_grouping_map_by(|&x| x)
        .fold(0, |acc, _key, _value| acc + 1)
        .into_iter()
        .sorted_by_key(|item| (-item.1, item.0))
        .take(5)
        .map(|(c, _)| c)
        .collect()
}

fn process_line(line: &str) -> bool {
    let (mut room, mut check) = line.split('[').collect_tuple().unwrap();
    room = room.split(char::is_numeric).collect_vec()[0];
    check = &check[0..check.len() - 1];
    let result = most_freq(room);
    result == check
}
fn main() {
    //part 1
    let code = include_str!("input.txt")
        .lines()
        .filter(|line| process_line(line))
        .count();

    println!("{code}");
}

#[test]
fn test_freq() {
    assert_eq!(true, process_line("aaaaa-bbb-z-y-x-123[abxyz]"));
    assert_eq!(true, process_line("a-b-c-d-e-f-g-h-987[abcde]"));
    assert_eq!(true, process_line("not-a-real-room-404[oarel]"));
    assert_eq!(false, process_line("totally-real-room-200[decoy]"));
}
