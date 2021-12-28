use gridit::{Grid, PositionsEnumerator};
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

fn part_one(paper: &mut TransparentPaper, instruction: FoldInstruction) -> usize {
    paper.fold(instruction);
    paper.visible_dots()
}

fn part_two(paper: &mut TransparentPaper, instructions: &[FoldInstruction]) {
    for instruction in instructions {
        paper.fold(*instruction);
    }

    paper.print();
}

fn main() {
    let (fold_instructions, mut paper) = parse_input();
    println!(
        "Part one answer: {}",
        part_one(&mut paper, fold_instructions[0])
    );

    println!("Part two answer: ");
    part_two(&mut paper, &fold_instructions[1..]);
}

#[derive(Debug, Clone, Copy)]
enum FoldInstruction {
    Up(usize),
    Left(usize),
}

impl FromStr for FoldInstruction {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: String = s.split(" ").skip(2).collect();
        let mut s = s.split("=");
        let direction = s.next().unwrap();
        let n = s.next().unwrap().parse::<usize>().unwrap();

        Ok(match direction {
            "x" => FoldInstruction::Left(n),
            "y" => FoldInstruction::Up(n),
            _ => unreachable!(),
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct Dot {
    x: usize,
    y: usize,
}

impl FromStr for Dot {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(",");
        let x = s.next().unwrap().parse::<usize>().unwrap();
        let y = s.next().unwrap().parse::<usize>().unwrap();
        Ok(Dot { x, y })
    }
}

fn parse_input() -> (Vec<FoldInstruction>, TransparentPaper) {
    let (instruction_folds, dots) = INPUT.lines().filter(|l| !l.is_empty()).fold(
        (Vec::new(), Vec::new()),
        |(mut fi, mut d), line| {
            if line.starts_with("f") {
                fi.push(FoldInstruction::from_str(line).unwrap());
            } else {
                d.push(Dot::from_str(line).unwrap())
            }
            (fi, d)
        },
    );

    let (max_x, max_y) = dots
        .iter()
        .fold((0, 0), |(mx, my), d| match (d.x > mx, d.y > my) {
            (true, true) => (d.x, d.y),
            (true, false) => (d.x, my),
            (false, true) => (mx, d.y),
            (false, false) => (mx, my),
        });

    let mut paper = TransparentPaper::new(max_x + 1, max_y + 1);
    for dot in dots {
        paper.add_dot(dot);
    }

    (instruction_folds, paper)
}

struct TransparentPaper {
    grid: Grid<bool>,
}

impl TransparentPaper {
    fn new(width: usize, height: usize) -> Self {
        Self {
            grid: Grid::new(width, height, false),
        }
    }

    fn add_dot(&mut self, dot: Dot) {
        self.grid.set((dot.x, dot.y), true);
    }

    fn print(&self) {
        let (s, _) =
            self.grid
                .iter()
                .grid_positions()
                .fold((String::new(), 0), |(mut s, mut y), (p, i)| {
                    if p.x == 0 && y + 1 == p.y {
                        s.push('\n');
                        y += 1;
                    }
                    let c = match i {
                        true => '#',
                        false => '.',
                    };
                    s.push(c);
                    (s, y)
                });
        println!("{}", s);
    }

    fn fold(&mut self, instruction: FoldInstruction) {
        self.grid = match instruction {
            FoldInstruction::Up(y) => self.fold_up(y),
            FoldInstruction::Left(x) => self.fold_left(x),
        };
    }

    fn fold_up(&self, y: usize) -> Grid<bool> {
        let (w, h) = self.grid.size();
        let new_height = h - y - 1;
        let new_grid: Vec<bool> = self.grid.iter().take(w * new_height).map(|b| *b).collect();
        let mut grid = Grid::from(new_grid, w, new_height);
        let mut counter = h;
        while counter > y {
            for (pos, i) in self.grid.row(counter - 1).grid_positions() {
                if *i {
                    grid.set((pos.x, h - counter), *i);
                }
            }
            counter -= 1;
        }
        grid
    }

    fn fold_left(&self, x: usize) -> Grid<bool> {
        let (w, h) = self.grid.size();
        let new_width = w - x - 1;
        let mut grid = Grid::new(new_width, h, false);
        let mut counter = w;
        while counter > x {
            for (pos, i) in self.grid.column(counter - 1).grid_positions() {
                if *i || *self.grid.get_unchecked((w - counter, pos.y)) {
                    grid.set((w - counter, pos.y), true);
                }
            }
            counter -= 1;
        }
        grid
    }

    fn visible_dots(&self) -> usize {
        self.grid.iter().filter(|d| **d).count()
    }
}
