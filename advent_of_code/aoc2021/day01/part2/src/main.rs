use itertools::Itertools;

const INPUT: &'static str = include_str!("../../input.txt");

fn main() {
    let depths = INPUT
        .lines()
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let answer = depths
        .iter()
        .tuple_windows::<(&u32, &u32, &u32)>()
        .zip(depths.iter().skip(1).tuple_windows::<(&u32, &u32, &u32)>())
        .filter(|(a, b)| (a.0 + a.1 + a.2) < (b.0 + b.1 + b.2))
        .count();

    println!("Answer: {}", answer);
}
