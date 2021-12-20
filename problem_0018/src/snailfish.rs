use std::ops::Add;
use core::ops::Range;

struct Snailfish {
    input: String
}

impl Snailfish {
    fn new(input: &str) -> Snailfish {
        Snailfish { input: String::from(input) }
    }

    fn parse(&self) -> Vec<(isize, Range<usize>)> {
        let len = self.input.len();
        let mut result = vec![];
        let mut last = 0;
        let mut depth = 0;
        let indices = self.input.match_indices(|c: char| !c.is_numeric());

        for (i, matched) in indices {
            let depth_incr = match matched {
                "[" => 1,
                "]" => -1,
                _ => 0
            };

            if last != i {
                result.push((depth, last..i));
            }

            depth += depth_incr;
            last = i + matched.len();
        }

        if last < len {
            result.push((depth, last..len));
        }

        result
    }
}

impl Add for Snailfish {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut result = String::new();
        result.push('[');
        result.push_str(&self.input);
        result.push(',');
        result.push_str(&other.input);
        result.push(']');

        Self {
            input: result
        }
    }
}

#[test]
fn test_add() {
    let snailfish_1 = Snailfish::new("[1,2]");
    let snailfish_2 = Snailfish::new("[[3,4],5]");
    let sum = snailfish_1 + snailfish_2;

    assert_eq!(
        sum.input,
        String::from("[[1,2],[[3,4],5]]")
    )
}

#[test]
fn test_parse() {
    let snailfish = Snailfish::new("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]");

    assert_eq!(snailfish.parse(), vec![
        (4, 4..5),
        (4, 6..7),
        (3, 9..10),
        (3, 13..14),
        (5, 17..18),
        (5, 19..20),
        (4, 22..23),
        (2, 28..29),
        (2, 30..31)
    ]);
}

#[test]
fn test_parse_with_big_numbers() {
    let snailfish = Snailfish::new("[1,200]");

    assert_eq!(snailfish.parse(), vec![
        (1, 1..2),
        (1, 3..6),
    ]);
}
