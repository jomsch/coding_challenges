const INPUT: &str = include_str!("../input.txt");

struct SyntexChecker;

fn matching(c: char) -> char {
    match c {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => unreachable!(),
    }
}

impl SyntexChecker {
    fn illegal_character(input: &str) -> Option<char> {
        let mut stack: Vec<char> = Vec::new();
        for c in input.chars() {
            match c {
                '(' | '[' | '<' | '{' => stack.push(c),
                ')' | ']' | '>' | '}' => {
                    let matching = matching(c);

                    if let Some(sc) = stack.pop() {
                        if sc != matching {
                            return Some(c);
                        }
                    } else {
                        return Some(c);
                    }
                }
                _ => (),
            }
        }
        None
    }

    fn complete(input: &str) -> Vec<char> {
        let mut stack: Vec<char> = Vec::new();
        for c in input.chars() {
            match c {
                '(' | '[' | '<' | '{' => {
                    stack.push(c);
                }
                ')' | ']' | '>' | '}' => {
                    stack.pop();
                }
                _ => (),
            }
        }
        stack.into_iter().map(|c| matching(c)).rev().collect()
    }
}

fn part_one() -> usize {
    INPUT
        .lines()
        .filter_map(|line| SyntexChecker::illegal_character(line))
        .fold(0, |acc, c| {
            acc + match c {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => unreachable!(),
            }
        })
}

fn part_two() -> usize {
    let mut points: Vec<usize> = INPUT
        .lines()
        .filter(|l| SyntexChecker::illegal_character(l).is_none())
        .map(|line| {
            let completion = SyntexChecker::complete(line);
            completion.iter().fold(0usize, |acc, c| {
                acc * 5
                    + match c {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => unreachable!(),
                    }
            })
        })
        .collect();

    points.sort();
    let middle_pos = points.len() / 2;
    points[middle_pos]
}

fn main() {
    println!("Part one answer: {}", part_one());
    println!("Part two answer: {}", part_two());
}
