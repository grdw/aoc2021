use std::fs;
use std::ops::Range;

type SnailfishPart = (isize, Range<usize>);
type Snailfish = Vec<SnailfishPart>;

#[derive(Debug, Eq, PartialEq)]
enum Action<'a> {
    Explode { pair: usize, left: Option<&'a SnailfishPart>, right: Option<&'a SnailfishPart> },
    Split,
    NonAction
}

fn main() {
    let contents = fs::read_to_string("input")
                      .unwrap_or("".to_string());

    let readings: Vec<&str> = contents
        .split_terminator("\n")
        .collect();

    println!("{:?}", readings);
}

fn add(snailfish: &str, other_snailfish: &str) -> String {
    let mut result = String::new();
    result.push('[');
    result.push_str(snailfish);
    result.push(',');
    result.push_str(other_snailfish);
    result.push(']');
    result
}

#[test]
fn test_sum_snailfish() {
    let snailfish_1 = "[1,2]";
    let snailfish_2 = "[[3,4],5]";

    assert_eq!(
        add(snailfish_1, snailfish_2),
        String::from("[[1,2],[[3,4],5]]")
    )
}

fn parse_snailfish(input: &str) -> Snailfish {
    let mut result = vec![];
    let mut last = 0;
    let mut depth = 0;

    for (i, matched) in input.match_indices(|c: char| !c.is_numeric()) {
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

    if last < input.len() {
        result.push((depth, last..input.len()));
    }

    result
}

#[test]
fn test_parse_snailfish() {
    let snailfish = "[[[[0,7],4],[7,[[8,4],9]]],[1,1]]";

    assert_eq!(parse_snailfish(&snailfish), vec![
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
fn test_parse_snailfish_2() {
    let snailfish = "[100,200]";

    assert_eq!(parse_snailfish(&snailfish), vec![
        (1, 1..4),
        (1, 5..8)
    ]);
}

fn action(input: &Snailfish) -> Action {
    for i in 0..input.len() {
        let (depth, range) = &input[i];

        if range.len() > 1 {
            return Action::Split
        }

        if *depth > 4 {
            let left = if i > 0 {
                input.get(i - 1)
            } else {
                None
            };

            return Action::Explode {
                pair: range.start,
                left: left,
                right: input.get(i + 2)
            }
        }
    }

    Action::NonAction
}

#[test]
fn test_action_explode() {
    let snailfish = parse_snailfish("[[[[[9,8],1],2],3],4]");
    let action = action(&snailfish);

    assert_eq!(action, Action::Explode {
        pair: 5,
        left: None,
        right: Some(&(4, 10..11))
    });
}

#[test]
fn test_action_no_action() {
    let snailfish = parse_snailfish("[2,4]");
    let action = action(&snailfish);

    assert_eq!(action, Action::NonAction);
}

#[test]
fn test_action_split() {
    let snailfish = parse_snailfish("[[[[0,7],4],[15,[0,13]]],[1,1]]");
    let action = action(&snailfish);

    assert_eq!(action, Action::Split);
}

fn explode(
    input: &str,
    pair: usize,
    left: Option<&SnailfishPart>,
    right: Option<&SnailfishPart>) -> String {

    let mut result = String::new();

    result.push_str(&input[0..pair-1]);

    let to_explode: Vec<u8> = input[pair..pair + 3]
        .split(",")
        .map(|n| n.parse::<u8>().unwrap())
        .collect();

    let mut left_sum_len = 0;

    if let Some((_, range)) = left {
        let left_t = input[range.start..range.end].parse::<u8>().unwrap();
        let sum = format!("{}", to_explode[0] + left_t);
        left_sum_len = sum.len();

        let mut start = 2;
        start += left_sum_len;
        result.replace_range(pair - start..pair - start + 1, &sum);
    } else {
        result.push_str("0,");
    };

    if let Some((_, range)) = right {
        let right_t = input[range.start..range.end].parse::<u8>().unwrap();
        let sum = format!("{}", to_explode[1] + right_t);

        if left_sum_len > 0 {
            result.push('0');
            result.push_str(&input[pair + 4..range.start])
        }

        result.push_str(&sum);
        result.push_str(&input[range.end..]);
    } else {
        result.push('0');
    };

    if right.is_none() {
        result.push_str(&input[pair + 4..]);
    }

    result
}

fn execute(input: &str, action: Action) -> String {
    match action {
        Action::Explode { pair, left, right } =>
            explode(input, pair, left, right),
        Action::Split => String::new(),
        _ => String::new()
    }
}

#[test]
fn test_explode_1() {
    let input = "[[[[[9,8],1],2],3],4]";
    let snailfish = parse_snailfish(&input);
    let action = action(&snailfish);

    assert_eq!(
        action,
        Action::Explode {
            pair: 5,
            left: None,
            right: Some(&(4, 10..11))
        }
    );

    let result = execute(&input, action);
    assert_eq!(result, String::from("[[[[0,9],2],3],4]"));
}

#[test]
fn test_explode_2() {
    let snailfish = "[7,[6,[5,[4,[3,2]]]]]";
    let parsed = parse_snailfish(&snailfish);
    let action = action(&parsed);

    assert_eq!(
        action,
        Action::Explode {
            pair: 13,
            left: Some(&(4, 10..11)),
            right: None
        }
    );

    let result = execute(&snailfish, action);
    assert_eq!(result, String::from("[7,[6,[5,[7,0]]]]"))
}

#[test]
fn test_explode_3() {
    let snailfish = "[[6,[5,[4,[3,2]]]],1]";
    let parsed = parse_snailfish(&snailfish);
    let action = action(&parsed);

    assert_eq!(
        action,
        Action::Explode {
            pair: 11,
            left: Some(&(4, 8..9)),
            right: Some(&(1, 19..20))
        }
    );

    let result = execute(&snailfish, action);
    assert_eq!(result, String::from("[[6,[5,[7,0]]],3]"))
}

#[test]
fn test_explode_4() {
    let snailfish = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]";
    let parsed = parse_snailfish(&snailfish);
    let action = action(&parsed);

    assert_eq!(
        action,
        Action::Explode {
            pair: 11,
            left: Some(&(4, 8..9)),
            right: Some(&(2, 20..21))
        }
    );

    let result = execute(&snailfish, action);
    assert_eq!(result, String::from("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"));
}

#[test]
fn test_explode_5() {
    let snailfish = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]";
    let parsed = parse_snailfish(&snailfish);
    let action = action(&parsed);

    assert_eq!(
        action,
        Action::Explode {
            pair: 25,
            left: Some(&(4, 22..23)),
            right: None
        }
    );

    let result = execute(&snailfish, action);
    assert_eq!(result, String::from("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"));
}

#[test]
fn test_explode_6() {
    let snailfish = "[[[[0,7],4],[7,[[8,4],9]]],[1,1]]";
    let parsed = parse_snailfish(&snailfish);
    let action = action(&parsed);

    assert_eq!(
        action,
        Action::Explode {
            pair: 17,
            left: Some(&(3, 13..14)),
            right: Some(&(4, 22..23))
        }
    );

    let result = execute(&snailfish, action);
    assert_eq!(result, String::from("[[[[0,7],4],[15,[0,13]]],[1,1]]"));
}
