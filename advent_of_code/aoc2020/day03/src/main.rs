type Vec2D<T> = Vec<Vec<T>>;

#[derive(Debug)]
struct Grid(Vec2D<char>);

impl Grid {
    fn new(grid: Vec2D<char>) -> Self {
        Self(grid)
    }

    fn get(&self, x: usize, y: usize) -> Option<char> {
        let len_y = self.0.len();
        if y >= len_y {
            return None;
        }
        let len_x = self.0[y].len();

        Some(self.0[y][x % len_x])
    }

    fn pattern_coords(& self, x: usize, y: usize) -> GridIter {
        GridIter {
            grid: &self,
            pattern: (x, y),
            position: (0, 0),
        }
    }
}
 

#[derive(Debug)]
struct GridIter<'a> {
    grid: &'a Grid,
    pattern: (usize, usize),
    position: (usize, usize),

}

impl<'a> Iterator for GridIter<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let (acc_x, acc_y) = self.pattern;
        let (cur_x, cur_y) = self.position;
        self.position.0 = cur_x + acc_x;
        self.position.1 = cur_y + acc_y;
        self.grid.get(self.position.0, self.position.1)
    }
}

fn main() {
    let input = include_str!("../input.txt")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let grid = Grid::new(input);

    part_one(&grid);
    part_two(&grid);
}

fn part_one(grid: &Grid) {
    let result = grid.pattern_coords(3, 1)
        .filter(|c| *c == '#')
        .count();

    println!("Part one result: {}", result);
}

fn part_two(grid: &Grid) {
    let patterns = [
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ];
    let result: usize = patterns.iter()
        .map(|(x,y)| grid.pattern_coords(*x, *y).filter(|c| *c == '#').count())
        .product();

    println!("Part two result: {}", result);
}
