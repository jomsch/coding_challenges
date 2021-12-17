use gridit::{pattern::SideStepsPattern, Grid, Position, PositionsEnumerator};

const INPUT: &str = include_str!("../input.txt");

fn parse_input() -> HeightMap {
    let height = INPUT.lines().count();
    let width = INPUT.lines().next().unwrap().len();
    let heights = INPUT.lines().fold(Vec::new(), |mut acc, line| {
        let mut line = line
            .chars()
            .map(|n| n.to_digit(10).unwrap() as usize)
            .collect();
        acc.append(&mut line);
        acc
    });

    HeightMap::new(heights, width, height)
}

struct HeightMap {
    grid: Grid<usize>,
}

impl HeightMap {
    fn new(heights: Vec<usize>, width: usize, height: usize) -> Self {
        Self {
            grid: Grid::from(heights, width, height),
        }
    }

    fn is_low_point(&self, pos: impl Into<Position>) -> bool {
        let pattern: SideStepsPattern = SideStepsPattern::new([(0, -1), (0, 1), (-1, 0), (1, 0)]);
        let pos = pos.into();
        let p = self.grid.get_unchecked(pos);
        self.grid.pattern(pos, pattern).all(|n| n > p)
    }

    fn get_low_points(&self) -> Vec<Position> {
        self.grid
            .positions()
            .filter(|p| self.is_low_point(*p))
            .collect()
    }

    fn get_low_points_heights(&self) -> Vec<usize> {
        self.get_low_points()
            .iter()
            .map(|p| *self.grid.get_unchecked(*p))
            .collect()
    }

    fn get_basin_size(&self, pos: impl Into<Position>) -> usize {
        let mut queue = vec![pos.into()];
        let mut visited = Vec::new();
        while let Some(pos) = queue.pop() {
            let pattern: SideStepsPattern =
                SideStepsPattern::new([(0, -1), (0, 1), (-1, 0), (1, 0)]);

            let neighbors = self.grid.pattern(pos, pattern).grid_positions();
            for (neighbor_pos, neighbor_value) in neighbors {
                if *neighbor_value != 9
                    && (!visited.contains(&neighbor_pos) && !queue.contains(&neighbor_pos))
                {
                    queue.push(neighbor_pos);
                }
            }
            visited.push(pos);
        }

        // dbg!(&visited);
        visited.len()
    }

    fn get_basin_sizes(&self) -> Vec<usize> {
        self.get_low_points()
            .iter()
            .map(|p| self.get_basin_size(*p))
            .collect()
    }
}

fn part_one(height_map: &HeightMap) -> usize {
    let low_point_heights = height_map.get_low_points_heights();
    let len = low_point_heights.len();
    low_point_heights.iter().sum::<usize>() + len
}

fn part_two(height_map: &HeightMap) -> usize {
    let mut basin_sizes = height_map.get_basin_sizes();
    basin_sizes.sort();
    let l = basin_sizes.len();
    let first = basin_sizes[l - 1];
    let second = basin_sizes[l - 2];
    let third = basin_sizes[l - 3];

    first * second * third
}

fn main() {
    let height_map = parse_input();

    println!("Part one answer: {}", part_one(&height_map));
    println!("Part two answer: {}", part_two(&height_map));
}
