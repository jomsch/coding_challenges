use std::collections::{HashSet, HashMap};

#[derive(Debug)]
struct Group<'a> {
    grp: &'a str,
}

impl<'a> Group<'a> {
    fn new(grp: &'a str) -> Self {
        Self { grp }
    }

    fn unique_answers(&self) -> usize {
        let answers: HashSet<char> = self.grp.chars()
            .filter(|c| !c.is_whitespace())
            .collect();

        answers.iter().count()
    }

    fn same_answers(&self) -> usize {
        let line_count = self.grp.lines().count();

        self.grp.chars()
            .fold(HashMap::new(), |mut acc, c| {
                if let Some(v) = acc.get_mut(&c) {
                    *v += 1;
                } else {
                    acc.insert(c, 1);
                }
                acc
            })
            .into_iter()
            .filter(|(c, n)| *n == line_count)
            .count()
    }
}


fn main() {
    let input = include_str!("../input.txt")
        .split("\n\n")
        .map(Group::new)
        .collect();

    part_one(&input);
    part_two(&input);
}


fn part_one(groups: &Vec<Group>) {
    let result: usize = groups
        .iter()
        .map(|g| g.unique_answers())
        .sum();

    println!("Result Part 1: {}", result);
}

fn part_two(groups: &Vec<Group>) {
    let result: usize = groups
        .iter()
        .map(|g| g.same_answers())
        .sum();

    println!("Result Part 2: {}", result);
}


mod tests {
    use super::*;

    #[test]
    fn test_group_unique_answers() {
        let group = Group::new("abc");
        assert_eq!(group.unique_answers(), 3);

        let group = Group::new(r#"ab
        ac
        "#);
        assert_eq!(group.unique_answers(), 3);

        let group = Group::new(r#"a
            a
            a
        "#);
        assert_eq!(group.unique_answers(), 1);

        let group = Group::new(r#"
        j
        "#);
        assert_eq!(group.unique_answers(), 1);
    }

    #[test]
    fn test_group_same_answers() {
        let group = Group::new(r#"abc"#);
        assert_eq!(group.same_answers(), 3);

        let group = Group::new(r#"a
                               b
                               c"#);
        assert_eq!(group.same_answers(), 0);


        let group = Group::new(r#"ab
                                ac"#);
        assert_eq!(group.same_answers(), 1);

        let group = Group::new(r#"a
                               a
                               a
                               a"#);
        assert_eq!(group.same_answers(), 1);

        let group = Group::new(r#"b"#);
        assert_eq!(group.same_answers(), 1);

        let group = Group::new("");
        assert_eq!(group.same_answers(), 0);
    }
}
