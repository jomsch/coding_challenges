use rustc_hash::FxHashMap;
const INPUT: &'static str = include_str!("../input.txt");

#[derive(Clone)]
struct PolymerCounter {
    map: FxHashMap<String, usize>,
    map_buffer: FxHashMap<String, usize>,
    rules: FxHashMap<String, char>,
    char_counter: FxHashMap<char, usize>,
}

impl PolymerCounter {
    fn new(rules: FxHashMap<String, char>) -> Self {
        let map: FxHashMap<String, usize> = rules.iter().map(|(s, _)| (s.to_string(), 0)).collect();
        let map_buffer = map.clone();
        let char_counter = rules.values().map(|c| (*c, 0)).collect();
        Self {
            map,
            map_buffer,
            rules,
            char_counter,
        }
    }

    fn add_template(&mut self, template: &str) {
        for c in template.chars() {
            self.char_counter.entry(c).and_modify(|n| *n += 1);
        }
        for pair in template.as_bytes().windows(2) {
            let pair = std::str::from_utf8(pair).unwrap().to_string();
            println!("{}", pair);
            self.map.entry(pair).and_modify(|n| *n += 1);
        }
    }

    fn steps(&mut self, steps: usize) {
        for _ in 0..steps {
            for (key, value) in self.map.iter_mut() {
                let insert_char = self.rules.get(key).unwrap();
                let mut first_poly = key.clone();
                let last_char = first_poly.remove(1);
                first_poly.push(*insert_char);
                let mut last_poly = insert_char.to_string();
                last_poly.push(last_char);

                // Insert Char in char_counter
                self.char_counter
                    .entry(*insert_char)
                    .and_modify(|n| *n += *value);

                self.map_buffer
                    .entry(first_poly)
                    .and_modify(|n| *n += *value);
                self.map_buffer
                    .entry(last_poly)
                    .and_modify(|n| *n += *value);

                *value = 0;
            }

            for (key, value) in self.map_buffer.iter_mut() {
                self.map.entry(key.clone()).and_modify(|n| *n += *value);
                *value = 0;
            }
        }
    }

    fn least_char(&self) -> usize {
        *self.char_counter.iter().map(|(_, n)| n).min().unwrap()
    }

    fn most_char(&self) -> usize {
        *self.char_counter.iter().map(|(_, n)| n).max().unwrap()
    }
}

fn parse_input<'a>() -> PolymerCounter {
    let mut lines = INPUT.lines();
    let template = lines.next().unwrap();
    let rules = lines
        .skip(1)
        .map(|line| {
            let mut split = line.split(" -> ");
            let left = split.next().unwrap();
            let right = split.next().unwrap();
            (left.to_string(), right.chars().next().unwrap())
        })
        .collect();

    let mut pc = PolymerCounter::new(rules);
    pc.add_template(template);
    pc
}

fn part_one(mut pc: PolymerCounter) -> usize {
    pc.steps(10);
    let least = pc.least_char();
    let most = pc.most_char();
    most - least
}

fn part_two(mut pc: PolymerCounter) -> usize {
    pc.steps(40);
    let least = pc.least_char();
    let most = pc.most_char();
    most - least
}

fn main() {
    let polymer_counter = parse_input();
    println!("Part one answer: {}", part_one(polymer_counter.clone()));
    println!("Part two answer: {}", part_two(polymer_counter));
}
