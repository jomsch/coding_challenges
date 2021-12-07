const INPUT: &str = include_str!("../input.txt");

fn part_one(positions: &[usize]) -> usize {
    let max = positions.iter().max().unwrap();
    (0..max + 1).fold(usize::MAX, |min_fuel, i| {
        let fuel = positions
            .iter()
            .map(|n| if *n > i { n - i } else { i - n })
            .sum();
        if fuel < min_fuel {
            return fuel;
        }
        min_fuel
    })
}

fn part_two(positions: &[usize]) -> usize {
    let max = positions.iter().max().unwrap();
    (0..max + 1).fold(usize::MAX, |min_fuel, i| {
        let fuel = positions
            .iter()
            .map(|n| {
                let n = if *n > i { n - i } else { i - n };
                (n * (n + 1) + 2) / 2
            })
            .sum();
        if fuel < min_fuel {
            return fuel;
        }
        min_fuel
    })
}

fn main() {
    let positions: Vec<usize> = INPUT
        .trim()
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect();

    println!("Part one answer: {}", part_one(&positions));
    println!("Part two answer: {}", part_two(&positions));
}
