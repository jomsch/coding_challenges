const INPUT: &str = include_str!("../input.txt");

fn part_one() -> usize {
    INPUT
        .lines()
        .map(|l| {
            l.split('|')
                .skip(1)
                .next()
                .unwrap()
                .trim()
                .split(" ")
                .filter(|s| matches!(s.len(), 2 | 3 | 4 | 7))
                .count()
        })
        .sum()
}

fn part_two() -> usize {
    let segments: Vec<(Vec<Segment>, Vec<Segment>)> = INPUT
        .lines()
        .map(|l| {
            let mut s = l.split('|');
            let input: Vec<Segment> = s
                .next()
                .unwrap()
                .trim()
                .split(" ")
                .map(Segment::new)
                .collect();
            let output: Vec<Segment> = s
                .next()
                .unwrap()
                .trim()
                .split(" ")
                .map(Segment::new)
                .collect();
            (input, output)
        })
        .collect();

    segments.into_iter().fold(0usize, |acc, (input, output)| {
        let mut segments = Segments::new(input);
        segments.decode_segments();
        acc + (output
            .iter()
            .map(|s| segments.get_number(s).to_string())
            .collect::<String>())
        .parse::<usize>()
        .unwrap()
    })
}

struct Segments {
    unknown_segments: Vec<Segment>,
    segments: [Option<Segment>; 10],
}

impl Segments {
    fn new(segments: Vec<Segment>) -> Self {
        Self {
            unknown_segments: segments,
            segments: [(); 10].map(|_| None),
        }
    }

    fn decode_segments(&mut self) {
        let mut index = 0;
        while !self.all_decoded() {
            if let Some(number) = self.decode_segment(index) {
                self.segments[number] = Some(self.unknown_segments.remove(index));
            }
            index = if index >= self.unknown_segments.len().saturating_sub(1) {
                0
            } else {
                index + 1
            }
        }
    }

    fn decode_segment(&self, index: usize) -> Option<usize> {
        let segment = &self.unknown_segments[index];
        match segment.0.len() {
            2 => return Some(1),
            3 => return Some(7),
            4 => return Some(4),
            7 => return Some(8),
            _ => (),
        }

        if !self.segment_is_known(8) {
            return None;
        }

        if self.segment_is_known(4) {
            let seg4 = self.segments[4].as_ref().unwrap();
            if segment.0.len() == 6 && seg4.0.iter().all(|c| segment.0.contains(c)) {
                return Some(9);
            }
        }

        if let Some(seg9) = self.segments[9].as_ref() {
            if segment.0.len() == 5 && segment.0.iter().all(|c| seg9.0.contains(c)) {
                if let Some(seg1) = self.segments[1].as_ref() {
                    if seg1.0.iter().all(|c| segment.0.contains(c)) {
                        return Some(3);
                    } else {
                        return Some(5);
                    }
                }
            }
        }

        if let Some(seg5) = self.segments[5].as_ref() {
            if segment.0.len() == 6 && seg5.0.iter().all(|c| segment.0.contains(c)) {
                return Some(6);
            }
        }

        if self.unknown_segments.len() < 3 {
            if segment.0.len() == 6 {
                return Some(0);
            } else {
                return Some(2);
            }
        }

        None
    }

    fn all_decoded(&self) -> bool {
        self.unknown_segments.is_empty()
    }

    fn segment_is_known(&self, number: usize) -> bool {
        self.segments[number].is_some()
    }

    fn get_number(&self, other: &Segment) -> usize {
        for i in 0..10 {
            let segment = self.segments[i].as_ref().unwrap();
            if segment.0.len() == other.0.len() && segment.0.iter().all(|c| other.0.contains(c)) {
                return i;
            }
        }
        panic!("{:?}", other.0);
    }
}

struct Segment(Vec<char>);

impl Segment {
    fn new(seq: &str) -> Self {
        Self(seq.chars().collect())
    }
}

fn main() {
    println!("Part one answer: {}", part_one());
    println!("Part two answer: {}", part_two());
}
