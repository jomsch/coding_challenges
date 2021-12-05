use gridit::{Grid, PositionsEnumerator};

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
struct Board(Grid<(usize, bool)>, bool);

impl Board {
    fn from(v: Vec<(usize, bool)>) -> Self {
        Board(Grid::from(v, 5, 5), false)
    }

    fn is_winning_position(&self, (row, col): (usize, usize)) -> bool {
        let is_row_winning = self.0.row(col).all(|(_, checked)| *checked);
        let is_col_winning = self.0.column(row).all(|(_, checked)| *checked);

        is_row_winning || is_col_winning
    }

    fn apply_drawing(&mut self, number: usize) -> Option<(usize, usize)> {
        self.0.iter_mut().grid_positions().find_map(|(p, v)| {
            if v.0 == number {
                v.1 = true;
                Some(p.into())
            } else {
                None
            }
        })
    }

    fn unchecked_sum(&self) -> usize {
        self.0
            .iter()
            .filter_map(|(v, c)| if *c { None } else { Some(*v) })
            .sum()
    }
}

fn parse_input() -> (Vec<usize>, Vec<Board>) {
    let mut lines = INPUT.lines();

    let drawings = lines.next().unwrap();
    let drawings = drawings
        .split(",")
        .map(|n| n.parse::<usize>().unwrap())
        .collect();

    let boards = lines
        .filter(|l| *l != "")
        .collect::<Vec<&str>>()
        .chunks(5)
        .fold(Vec::new(), |mut boards, raw| {
            let grid: Vec<(usize, bool)> = raw
                .iter()
                .map(|l| {
                    l.split(' ')
                        .filter_map(|c| Some((c.trim().parse::<usize>().ok()?, false)))
                })
                .flatten()
                .collect();
            boards.push(Board::from(grid));
            boards
        });

    (drawings, boards)
}

fn get_answer(drawings: &[usize], boards: &mut Vec<Board>) -> (usize, usize) {
    let mut last_sum = 0;
    let mut first_sum = 0;
    for drawing in drawings {
        for board in boards.iter_mut() {
            if !board.1 {
                if let Some(pos) = board.apply_drawing(*drawing) {
                    if board.is_winning_position(pos) {
                        if first_sum == 0 {
                            first_sum = board.unchecked_sum() * drawing;
                        }
                        last_sum = board.unchecked_sum() * drawing;
                        board.1 = true;
                    }
                }
            }
        }
    }
    (first_sum, last_sum)
}

fn main() {
    let (drawings, mut boards) = parse_input();
    let (one, two) = get_answer(&drawings, &mut boards);
    println!("Part one answer: {}", one);
    println!("Part two answer: {}", two);
}
