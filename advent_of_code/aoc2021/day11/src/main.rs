use gridit::{Grid, Position, PositionsEnumerator};

const INPUT: &str = include_str!("../input.txt");

struct Cave {
    octopuses: Grid<usize>,
    flashed: Vec<Position>,
}

impl Cave {
    fn new(octopuses: Vec<usize>) -> Self {
        Self {
            octopuses: Grid::from(octopuses, 10, 10),
            flashed: Vec::new(),
        }
    }

    // Returns how many octopuses flash after the given steps
    fn flashed_after_steps(&mut self, steps: usize) -> usize {
        let mut counter = 0;

        for _ in 0..steps {
            counter += self.flashed.len();
            self.flashed = Vec::new();
            for pos in self.octopuses.positions() {
                let octo = self.octopuses.get_mut_unchecked(pos);
                if *octo < 9 {
                    *octo += 1;
                } else {
                    self.flashed.push(pos);
                    let mut flashing = self.get_unflashed_neighbors(pos);
                    while let Some(npos) = flashing.pop() {
                        let neighbor = self.octopuses.get_mut_unchecked(npos);
                        if self.flashed.contains(&npos) {
                            continue;
                        } else if *neighbor < 9 {
                            *neighbor += 1;
                        } else {
                            self.flashed.push(npos);
                            let mut other = self.get_unflashed_neighbors(npos);
                            flashing.append(&mut other);
                        }
                    }
                }

                for fpos in &self.flashed {
                    let focto = self.octopuses.get_mut_unchecked(*fpos);
                    *focto = 0;
                }
            }
        }
        counter + self.flashed.len()
    }

    fn first_simltaenously_flash(&mut self) -> usize {
        let mut counter = 0;

        while self.flashed.len() != 100 {
            counter += 1;
            self.flashed = Vec::new();
            for pos in self.octopuses.positions() {
                let octo = self.octopuses.get_mut_unchecked(pos);
                if *octo < 9 {
                    *octo += 1;
                } else {
                    self.flashed.push(pos);
                    let mut flashing = self.get_unflashed_neighbors(pos);
                    while let Some(npos) = flashing.pop() {
                        let neighbor = self.octopuses.get_mut_unchecked(npos);
                        if self.flashed.contains(&npos) {
                            continue;
                        } else if *neighbor < 9 {
                            *neighbor += 1;
                        } else {
                            self.flashed.push(npos);
                            let mut other = self.get_unflashed_neighbors(npos);
                            flashing.append(&mut other);
                        }
                    }
                }

                for fpos in &self.flashed {
                    let focto = self.octopuses.get_mut_unchecked(*fpos);
                    *focto = 0;
                }
            }
        }
        counter
    }

    fn get_unflashed_neighbors(&self, pos: Position) -> Vec<Position> {
        self.octopuses
            .neighbors(pos)
            .grid_positions()
            .map(|(p, _)| p)
            .filter(|p| !self.flashed.contains(&p))
            .collect()
    }
}

fn parse_input() -> Cave {
    let octopuses: Vec<usize> = INPUT
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .flatten()
        .collect();

    Cave::new(octopuses)
}

fn part_one(mut cave: Cave) -> usize {
    cave.flashed_after_steps(100)
}

fn part_two(mut cave: Cave) -> usize {
    cave.first_simltaenously_flash()
}

fn main() {
    println!("Answer part one: {}", part_one(parse_input()));
    println!("Answer part two: {}", part_two(parse_input()));
}
