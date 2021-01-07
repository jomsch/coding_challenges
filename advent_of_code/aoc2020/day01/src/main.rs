use std::collections::HashSet;
use itertools::Itertools;

fn main() {
    let answer: (u32, u32, u32) = include_str!("../input.txt")
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .tuple_combinations()
        .filter(|(a, b, c)| a+b+c == 2020)
        .take(1)
        .exactly_one()
        .unwrap();

    println!("{}", answer.0 * answer.1 * answer.2);

}
