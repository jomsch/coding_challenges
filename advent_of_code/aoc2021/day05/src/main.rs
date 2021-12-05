use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

fn part_one(lines: &Vec<Line>) -> usize {
    let straight_lines: Vec<&Line> = lines.iter().filter(|l| l.is_straight()).collect();

    let mut grid = Grid::new();

    for line in straight_lines {
        grid.draw_line(*line);
    }

    grid.0.iter().filter(|n| **n > 1).count()
}

fn part_two(lines: &Vec<Line>) -> usize {
    let mut grid = Grid::new();
    for line in lines {
        grid.draw_line(*line);
    }

    grid.0.iter().filter(|n| **n > 1).count()
}

fn main() {
    let lines: Vec<Line> = INPUT.lines().map(|l| Line::from_str(l).unwrap()).collect();

    println!("Part one answer: {}", part_one(&lines));
    println!("Part two answer: {}", part_two(&lines));
}

#[derive(Clone, Copy, Debug)]
struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    fn coords(&self) -> (usize, usize, usize, usize) {
        (self.p1.x, self.p1.y, self.p2.x, self.p2.y)
    }

    fn is_vertical(&self) -> bool {
        self.p1.x == self.p2.x
    }

    fn is_horizontal(&self) -> bool {
        self.p1.y == self.p2.y
    }

    fn is_straight(&self) -> bool {
        self.is_vertical() || self.is_horizontal()
    }
}

#[derive(Clone, Copy, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl FromStr for Line {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut points = s.split(" -> ");
        let p1 = points.next().unwrap();
        let p2 = points.next().unwrap();

        let p1: Vec<usize> = p1.split(',').map(|c| c.parse::<usize>().unwrap()).collect();
        let p2: Vec<usize> = p2.split(',').map(|c| c.parse::<usize>().unwrap()).collect();
        Ok(Self {
            p1: Point::new(p1[0], p1[1]),
            p2: Point::new(p2[0], p2[1]),
        })
    }
}

#[derive(Debug)]
struct Grid(Vec<usize>);

impl Grid {
    fn new() -> Self {
        Self(vec![0; 1000 * 1000])
    }

    fn draw_point(&mut self, x: usize, y: usize) {
        let idx = y * 1000 + x;
        self.0[idx] += 1;
    }

    fn draw_line(&mut self, line: Line) {
        let (x1, y1, x2, y2) = line.coords();
        if line.is_horizontal() {
            let (mut xn, max) = if x1 < x2 { (x1, x2 + 1) } else { (x2, x1 + 1) };
            while xn != max {
                self.draw_point(xn, y1);
                xn += 1;
            }
        } else if line.is_vertical() {
            let (mut yn, max) = if y1 < y2 { (y1, y2 + 1) } else { (y2, y1 + 1) };
            while yn != max {
                self.draw_point(x1, yn);
                yn += 1;
            }
        } else {
            let (mut xn, mut yn) = (x1, y1);
            while (xn, yn) != (x2, y2) {
                self.draw_point(xn, yn);

                if x1 < x2 {
                    xn += 1;
                } else {
                    xn -= 1;
                }
                if y1 < y2 {
                    yn += 1;
                } else {
                    yn -= 1;
                }
            }
            self.draw_point(x2, y2);
        }
    }
}
