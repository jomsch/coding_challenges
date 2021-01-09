use std::convert::TryFrom;
use anyhow::anyhow;

#[derive(Debug)]
struct Seat {
    row: u16,
    column: u16,
}

impl Seat {
    fn row(&self) -> u16 {
        self.row
    }

    fn column(&self) -> u16 {
        self.column
    }

    fn id(&self) -> u16 {
        (self.row << 3) | self.column
    }
}

impl TryFrom<&str> for Seat {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let number = s
            .chars()
            .enumerate()
            .try_fold(0,|acc, (i, c)| match c {
                'F' | 'L' => Ok(acc),
                'B' | 'R' => Ok(acc | 1 << (9-i)), 
                _ => Err(anyhow!("Error Beep Boop Beep")),
            })?;

        Ok(Self {
            row: number >> 3,
            column: number & 0b_0000_0000_0000_0111,
        })
    }
}

fn main() -> Result<(), anyhow::Error>{
    let input = include_str!("../input.txt");

    part_one(input);
    part_two(input);
    


    Ok(())
}

fn part_one(input: &str) {
    let result = input.lines()
        .map(|s| Seat::try_from(s).unwrap().id())
        .max();

    println!("Part one: {}", result.unwrap());
}

fn part_two(input: &str) {
    let mut ids = input.lines()
        .map(|s| Seat::try_from(s).unwrap().id())
        .collect::<Vec<u16>>();

    ids.sort();
    let result = ids
        .windows(2)
        .find(|n| n[0]+2 == n[1])
        .map(|n| n[1]-1);

    println!("Part two: {}", result.unwrap());
}




#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_seat_row() {
        assert_eq!(Seat::try_from("FBFBBFFRLR").unwrap().row(), 44);
        assert_eq!(Seat::try_from("BFFFBBFRRR").unwrap().row(), 70);
        assert_eq!(Seat::try_from("FFFBBBFRRR").unwrap().row(), 14);
        assert_eq!(Seat::try_from("BBFFBBFRLL").unwrap().row(), 102);
    }
    
    #[test]
    fn test_seat_column() {
        assert_eq!(Seat::try_from("FBFBBFFRLR").unwrap().column(), 5);
        assert_eq!(Seat::try_from("BFFFBBFRRR").unwrap().column(), 7);
        assert_eq!(Seat::try_from("FFFBBBFRRR").unwrap().column(), 7);
        assert_eq!(Seat::try_from("BBFFBBFRLL").unwrap().column(), 4);
    }

    #[test]
    fn test_seat_id() {
        assert_eq!(Seat::try_from("FBFBBFFRLR").unwrap().id(), 357);
        assert_eq!(Seat::try_from("BFFFBBFRRR").unwrap().id(), 567);
        assert_eq!(Seat::try_from("FFFBBBFRRR").unwrap().id(), 119);
        assert_eq!(Seat::try_from("BBFFBBFRLL").unwrap().id(), 820);
    }
}
