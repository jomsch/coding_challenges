const INPUT: &str = include_str!("../input.txt");
const TEST_INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
";

type BitVec = Vec<Vec<u8>>;

fn part_one(bitvec: &BitVec) -> usize {
    let mut gamma_filter = Bitter::<Common, BITS>::new(&bitvec);
    let mut epsilon_filter = Bitter::<Uncommon, BITS>::new(&bitvec);
    for i in 0..BITS {
        gamma_filter.common_bit_at(i, None);
        epsilon_filter.common_bit_at(i, None);
    }
    let gamma: usize = gamma_filter.common_bits_as_number();
    let epsilon: usize = epsilon_filter.common_bits_as_number();

    gamma * epsilon
}

fn part_two(bitvec: &BitVec) -> usize {
    let mut oxygen_filter = Bitter::<Common, BITS>::new(&bitvec);
    let mut co2_filter = Bitter::<Uncommon, BITS>::new(&bitvec);
    oxygen_filter.common_bit_at(0, None);
    co2_filter.common_bit_at(0, None);
    for i in 1..BITS {
        oxygen_filter.common_bit_at(i, Some(i));
        co2_filter.common_bit_at(i, Some(i));
    }
    let oxygen: usize = oxygen_filter.common_bits_as_number();
    let co2: usize = co2_filter.common_bits_as_number();

    oxygen * co2
}

const BITS: usize = 12;

fn main() {
    let bits: Vec<Vec<u8>> = INPUT
        .lines()
        .map(|l| {
            l.chars()
                .map(|n| n.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect();

    println!("Part one answer: {}", part_one(&bits));
    println!("Part two answer: {}", part_two(&bits));
}

trait Filter {
    fn filter(bit: u8) -> u8 {
        bit
    }
}

#[derive(Default)]
struct Common;
impl Filter for Common {}

#[derive(Default)]
struct Uncommon;
impl Filter for Uncommon {
    fn filter(bit: u8) -> u8 {
        if bit == 1 {
            0
        } else {
            1
        }
    }
}

struct Bitter<'a, T: Filter, const N: usize> {
    bits: &'a BitVec,
    common: [u8; N],
    _filter: T,
    done: bool,
}

impl<'a, T: Filter + Default, const N: usize> Bitter<'a, T, N> {
    fn new(bits: &'a BitVec) -> Self {
        Self {
            bits,
            common: [0; N],
            _filter: T::default(),
            done: false,
        }
    }

    fn common_count(&self, until: usize) -> usize {
        self.bits
            .iter()
            .filter(|b| {
                b[0..until]
                    .iter()
                    .enumerate()
                    .all(|(i, n)| self.common[i] == *n)
            })
            .count()
    }

    fn get_first_common(&self, until: usize) -> [u8; N] {
        let bits = self
            .bits
            .iter()
            .filter(|b| {
                b[0..until]
                    .iter()
                    .enumerate()
                    .all(|(i, n)| self.common[i] == *n)
            })
            .next()
            .unwrap();
        let mut new_bits = [0; N];
        for i in 0..N {
            new_bits[i] = bits[i];
        }
        new_bits
    }

    fn common_bit_at(&mut self, at: usize, consider: Option<usize>) {
        if self.done {
            return;
        }
        let counter: isize = if let Some(i) = consider {
            if self.common_count(i) == 1 {
                let first = self.get_first_common(i);
                self.common = first;
                self.done = true;
                return;
            }
            self.bits
                .iter()
                .filter(|b| {
                    b[0..i]
                        .iter()
                        .enumerate()
                        .all(|(i, n)| self.common[i] == *n)
                })
                .fold(0, |acc, i| if i[at] == 1 { acc + 1 } else { acc - 1 })
        } else {
            self.bits
                .iter()
                .fold(0, |acc, i| if i[at] == 1 { acc + 1 } else { acc - 1 })
        };

        self.common[at] = T::filter(match counter < 0 {
            true => 0,
            false => 1,
        });
    }

    fn common_bits_as_number(&self) -> usize {
        let mut number: usize = 0;
        for i in 0..N {
            number = (number << 1) | (self.common[i] as usize);
        }
        number
    }
}
