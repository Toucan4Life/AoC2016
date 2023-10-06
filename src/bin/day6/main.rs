use itertools::Itertools;

fn decrypt(input: String) -> String {
    let chars: Vec<Vec<char>> = input
        .lines()
        .into_iter()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let transposed: String = transpose(chars)
        .into_iter()
        .map(|line| {
            line.into_iter()
                .into_grouping_map_by(|&x| x)
                .fold(0, |acc, _key, _value| acc + 1)
                .into_iter()
                .sorted_by_key(|item| (-item.1))
                .take(1)
                .map(|(c, _)| c)
                .collect::<String>()
        })
        .collect();
    transposed
}

fn main() {
    let decrypted = decrypt(include_str!("input.txt").to_string());
    println!("{decrypted}");
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

#[test]
fn test_decrypt() {
    assert_eq!(
        "easter",
        decrypt(
            "eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar"
                .to_string()
        )
    );
}
