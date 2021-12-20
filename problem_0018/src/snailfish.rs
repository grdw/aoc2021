mod action;

use std::ops::Add;
use core::ops::Range;
use action::{*, Action};

#[derive(Debug, Eq, PartialEq)]
pub struct SnailfishPart {
    depth: isize,
    range: Range<usize>
}

impl SnailfishPart {
    pub fn new(depth: isize, range: Range<usize>) -> SnailfishPart {
        SnailfishPart {
            depth: depth,
            range: range
        }
    }
}

pub struct Snailfish {
    input: String
}

impl Snailfish {
    fn new(input: &str) -> Snailfish {
        Snailfish { input: String::from(input) }
    }

    fn parse(&self) -> Vec<SnailfishPart> {
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
                result.push(
                    SnailfishPart::new(depth, last..i)
                );
            }

            depth += depth_incr;
            last = i + matched.len();
        }

        if last < len {
            result.push(
                SnailfishPart::new(depth, last..len)
            );
        }

        result
    }

    fn action(&self) -> Action {
        action::get_action(self)
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
        SnailfishPart::new(4, 4..5),
        SnailfishPart::new(4, 6..7),
        SnailfishPart::new(3, 9..10),
        SnailfishPart::new(3, 13..14),
        SnailfishPart::new(5, 17..18),
        SnailfishPart::new(5, 19..20),
        SnailfishPart::new(4, 22..23),
        SnailfishPart::new(2, 28..29),
        SnailfishPart::new(2, 30..31)
    ]);
}

#[test]
fn test_parse_with_big_numbers() {
    let snailfish = Snailfish::new("[1,200]");

    assert_eq!(snailfish.parse(), vec![
        SnailfishPart::new(1, 1..2),
        SnailfishPart::new(1, 3..6),
    ]);
}
