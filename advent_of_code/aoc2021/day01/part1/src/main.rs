const INPUT: &'static str = include_str!("../../input.txt");

fn main() {
    let depths: Vec<_> = INPUT.lines().map(|n| n.parse::<u32>().unwrap()).collect();
    let answer = depths
        .iter()
        .zip(depths.iter().skip(1))
        .filter(|(a, b)| b > a)
        .count();

    println!("Answer: {}", answer);
}
