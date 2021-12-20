use crate::snailfish::Snailfish;
use core::ops::Range;

#[derive(Debug, Eq, PartialEq)]
pub struct SnailfishPart {
    pub depth: isize,
    pub range: Range<usize>
}

impl SnailfishPart {
    pub fn new(depth: isize, range: Range<usize>) -> SnailfishPart {
        SnailfishPart {
            depth: depth,
            range: range
        }
    }
}

impl Snailfish {
    pub fn parse(&self) -> Vec<SnailfishPart> {
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
