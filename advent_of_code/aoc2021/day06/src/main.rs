const INPUT: &str = include_str!("../input.txt");
const TEST_INPUT: &str = "3,4,3,1,2";

fn part_one(fishes: &[usize]) -> usize {
    let mut fishes = FishPopulation::new(&fishes);
    fishes.calculate_growht(80)
}

fn part_two(fishes: &[usize]) -> usize {
    let mut fishes = FishPopulation::new(&fishes);
    fishes.calculate_growht(256)
}

fn main() {
    let fishes: Vec<usize> = INPUT
        .trim()
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect();

    println!("Answer part one: {}", part_one(&fishes));
    println!("Answer part two: {}", part_two(&fishes));
}

struct FishPopulation {
    population: [usize; 9],
}

impl FishPopulation {
    fn new(starting_population: &[usize]) -> Self {
        let mut population = [0; 9];
        for p in starting_population {
            population[*p] += 1;
        }
        Self { population }
    }

    fn calculate_growht(&mut self, days: usize) -> usize {
        for _ in 0..days {
            let mut buf = [0; 9];
            for i in (1..9).rev() {
                buf[i - 1] = self.population[i];
            }
            buf[8] = self.population[0];
            buf[6] += self.population[0];
            self.population = buf;
        }
        self.population.iter().sum()
    }
}
