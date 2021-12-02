const INPUT: &str = include_str!("../input.txt");

fn part_one() -> isize {
    let (h, d) = INPUT.lines().fold((0, 0), |(h, d), v| {
        let s: Vec<_> = v.split(' ').collect();
        let n: isize = s[1].parse().unwrap();
        match s[0] {
            "forward" => (h + n, d),
            "down" => (h, d + n),
            "up" => (h, d - n),
            _ => unreachable!(),
        }
    });
    h * d
}
fn part_two() -> isize {
    let (h, d, _) = INPUT.lines().fold((0, 0, 0), |(h, d, a), v| {
        let s: Vec<_> = v.split(' ').collect();
        let n: isize = s[1].parse().unwrap();
        match s[0] {
            "forward" => (h + n, d + n * a, a),
            "down" => (h, d, a + n),
            "up" => (h, d, a - n),
            _ => unreachable!(),
        }
    });
    h * d
}

fn main() {
    println!("Answer part one: {}", part_one());
    println!("Answer part two: {}", part_two());
}
