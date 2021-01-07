use anyhow::Result;
use std::ops::RangeInclusive;

struct PasswordValidator<'a> {
    range: RangeInclusive<usize>,
    letter: char,
    password: &'a str
}

impl<'a> PasswordValidator<'a> {
    fn new(range: RangeInclusive<usize>, letter: char, password: &'a str) -> Self {
        Self {
            range,
            letter,
            password
        }
    }

    fn from_input(input: &'a str) -> Self {
        let mut parts = input.split(' ');
        let numbs: Vec<_>= parts.next()
            .unwrap()
            .split("-")
            .map(str::parse::<_>)
            .collect::<Result< Vec<usize>, std::num::ParseIntError>>()
            .unwrap();

        let letter = parts.next().unwrap().chars().nth(0).unwrap();
            
        let password = parts.next().unwrap();
        PasswordValidator::new(numbs[0]..=numbs[1], letter, password)
    }

    fn is_valid(&self) -> bool {
        let count = self.password
            .chars()
            .filter(|c| *c == self.letter)
            .count();

        self.range.contains(&count)
    }

    fn is_valid2(&self) -> bool {
        let i1 = *self.range.start();
        let i2 = *self.range.end();
        
        let count = [i1, i2].iter()
            .filter(|n| self.password.chars().nth(**n-1).unwrap_or('?') == self.letter)
            .count();

        count == 1
    }
}

fn main() {
    let answer = include_str!("../input.txt")
        .lines()
        .filter_map(|l|  {
            match PasswordValidator::from_input(l).is_valid2() {
                true => Some(()),
                false => None,
            }
        })
        .count(); 

        dbg!(answer);
        dbg!();
}




mod test {
    use super::*;

    #[test]
    fn bla1() {
        let a = PasswordValidator::from_input("1-3 a: abcde");
        let b = PasswordValidator::from_input("1-3 b: cdefg");
        let c = PasswordValidator::from_input("2-9 c: ccccccccc");

        assert!(a.is_valid());
        assert!(!b.is_valid());
        assert!(c.is_valid());
    }

    #[test]
    fn bla2() {
        let a = PasswordValidator::from_input("1-3 a: abcde");
        let b = PasswordValidator::from_input("1-3 b: cdefg");
        let c = PasswordValidator::from_input("2-9 c: ccccccccc");

        assert!(a.is_valid2());
        assert!(!b.is_valid2());
        assert!(!c.is_valid2());
    }
}

