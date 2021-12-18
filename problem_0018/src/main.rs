use std::fs;
use std::ops::Range;
use std::collections::VecDeque;

type SnailfishPart = (isize, Range<usize>);
type Snailfish = Vec<SnailfishPart>;

#[derive(Debug, Eq, PartialEq)]
enum Action<'a> {
    Explode {
        pair: &'a Range<usize>,
        left: Option<&'a SnailfishPart>,
        right: Option<&'a SnailfishPart>
    },
    Split { range: &'a SnailfishPart },
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

#[test]
fn test_parse_snailfish_3() {
    let part = "[[[[1,1],[2,2]],[3,3]],[4,4]]";
    let final_part = "[5,5]";
    let snailfish = add(part, final_part);

    assert_eq!(parse_snailfish(&snailfish), vec![
        (5, 5..6),
        (5, 7..8),
        (5, 11..12),
        (5, 13..14),
        (4, 18..19),
        (4, 20..21),
        (3, 25..26),
        (3, 27..28),
        (2, 32..33),
        (2, 34..35)
    ]);
}

fn action(input: &Snailfish) -> Action {
    for i in 0..input.len() {
        let (depth, range) = &input[i];

        if range.len() > 1 {
            return Action::Split {
                range: &input[i]
            }
        }

        if *depth > 4 {
            let left = if i > 0 {
                input.get(i - 1)
            } else {
                None
            };

            return Action::Explode {
                pair: range,
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
        pair: &(5..6),
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

    assert_eq!(action, Action::Split { range: &(3, 13..15) });
}

fn explode(
    input: &str,
    pair: &Range<usize>,
    left: Option<&SnailfishPart>,
    right: Option<&SnailfishPart>) -> String {

    let mut result = String::from(input);

    let to_explode: Vec<u8> = input[pair.start..pair.end + 2]
        .split(",")
        .map(|n| n.parse::<u8>().unwrap())
        .collect();

    match (left, right) {
        (Some((_, l_range)), Some((_, r_range))) => {
            let right_t = input[r_range.start..r_range.end]
                .parse::<u8>()
                .unwrap();

            let left_t = input[l_range.start..l_range.end]
                .parse::<u8>()
                .unwrap();

            let l_sum = format!("{}", to_explode[0] + left_t);
            let r_sum = format!("{}", to_explode[1] + right_t);

            result.replace_range(r_range.start..r_range.end, &r_sum);
            result.replace_range(l_range.start..l_range.end, &l_sum);

            let s = pair.start - (2 - l_sum.len());
            let e = pair.end + (2 + l_sum.len());
            result.replace_range(s..e, "0");
        },
        (None, Some((_, range))) => {
            let right_t = input[range.start..range.end]
                .parse::<u8>()
                .unwrap();

            let sum = format!("{}", to_explode[1] + right_t);

            result.replace_range(range.start..range.end, &sum);

            let s = pair.start - (2 - sum.len());
            let e = pair.end + (2 + sum.len());
            result.replace_range(s..e, "0");
        },
        (Some((_, range)), None) => {
            let left_t = input[range.start..range.end]
                .parse::<u8>()
                .unwrap();

            let sum = format!("{}", to_explode[0] + left_t);

            result.replace_range(range.start..range.end, &sum);

            let s = pair.start - (2 - sum.len());
            let e = pair.end + (2 + sum.len());
            result.replace_range(s..e, "0");
        },
        _ => panic!("Invalid action"),
    }

    result
}

fn split(input: &str, sfp: &SnailfishPart) -> String {
    let mut result = String::from(input);
    let (_, range) = sfp;
    let number = input[range.start..range.end].parse::<u8>().unwrap();
    let div = number / 2;
    let pair = format!("[{},{}]", div, number - div);

    result.replace_range(range.start..range.end, &pair);
    result
}

fn execute(input: &str, action: Action) -> Option<String> {
    match action {
        Action::Explode { pair, left, right } =>
            Some(explode(input, pair, left, right)),
        Action::Split { range } => Some(split(input, range)),
        _ => None
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
            pair: &(5..6),
            left: None,
            right: Some(&(4, 10..11))
        }
    );

    let result = execute(&input, action).unwrap();
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
            pair: &(13..14),
            left: Some(&(4, 10..11)),
            right: None
        }
    );

    let result = execute(&snailfish, action).unwrap();
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
            pair: &(11..12),
            left: Some(&(4, 8..9)),
            right: Some(&(1, 19..20))
        }
    );

    let result = execute(&snailfish, action).unwrap();
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
            pair: &(11..12),
            left: Some(&(4, 8..9)),
            right: Some(&(2, 20..21))
        }
    );

    let result = execute(&snailfish, action).unwrap();
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
            pair: &(25..26),
            left: Some(&(4, 22..23)),
            right: None
        }
    );

    let result = execute(&snailfish, action).unwrap();
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
            pair: &(17..18),
            left: Some(&(3, 13..14)),
            right: Some(&(4, 22..23))
        }
    );

    let result = execute(&snailfish, action).unwrap();
    assert_eq!(result, String::from("[[[[0,7],4],[15,[0,13]]],[1,1]]"));
}

#[test]
fn test_explode_7() {
    let snailfish = "[[[[[1,1],[2,2]],[3,3]],[4,4]],[5,5]]";
    let parsed = parse_snailfish(&snailfish);
    let first_action = action(&parsed);

    assert_eq!(
        first_action,
        Action::Explode {
            pair: &(5..6),
            left: None,
            right: Some(&(5, 11..12))
        }
    );

    let result = execute(&snailfish, first_action).unwrap();
    assert_eq!(result, String::from("[[[[0,[3,2]],[3,3]],[4,4]],[5,5]]"));

    let snailfish = "[[[[0,[3,2]],[3,3]],[4,4]],[5,5]]";
    let parsed = parse_snailfish(&snailfish);
    let second_action = action(&parsed);

    assert_eq!(
        second_action,
        Action::Explode {
            pair: &(7..8),
            left: Some(&(4, 4..5)),
            right: Some(&(4, 14..15))
        }
    );

    let result = execute(&snailfish, second_action).unwrap();
    assert_eq!(result, String::from("[[[[3,0],[5,3]],[4,4]],[5,5]]"));
}

#[test]
fn test_explode_8() {
    let snailfish_1 = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]";
    let snailfish_2 = "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]";
    let sum = add(snailfish_1, snailfish_2);
    let parsed = parse_snailfish(&sum);
    let action = action(&parsed);

    assert_eq!(
        action,
        Action::Explode {
            pair: &(7..8),
            left: Some(&(4, 4..5)),
            right: Some(&(4, 14..15))
        }
    );

    let result = execute(&sum, action).unwrap();
    assert_eq!(
        result,
        String::from("[[[[4,0],[5,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]")
    );
}

#[test]
fn test_explode_9() {
    let snailfish = "[[[[0,7],4],[7,[[8,4],4]]],[1,1]]";
    let parsed = parse_snailfish(&snailfish);
    let action = action(&parsed);

    assert_eq!(
        action,
        Action::Explode {
            pair: &(17..18),
            left: Some(&(3, 13..14)),
            right: Some(&(4, 22..23))
        }
    );

    let result = execute(&snailfish, action).unwrap();
    assert_eq!(result, String::from("[[[[0,7],4],[15,[0,8]]],[1,1]]"));
}

#[test]
fn test_explode_10() {
    let snailfish = "[[[[0,7],4],[7,[[1,4],9]]],[1,1]]";
    let parsed = parse_snailfish(&snailfish);
    let action = action(&parsed);

    assert_eq!(
        action,
        Action::Explode {
            pair: &(17..18),
            left: Some(&(3, 13..14)),
            right: Some(&(4, 22..23))
        }
    );

    let result = execute(&snailfish, action).unwrap();
    assert_eq!(result, String::from("[[[[0,7],4],[8,[0,13]]],[1,1]]"));
}


#[test]
fn test_split_1() {
    let snailfish = "[[[[0,7],4],[15,[0,13]]],[1,1]]";
    let parsed = parse_snailfish(&snailfish);
    let action = action(&parsed);

    assert_eq!(
        action,
        Action::Split { range: &(3, 13..15) }
    );

    let result = execute(&snailfish, action).unwrap();
    assert_eq!(result, String::from("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]"));
}

#[test]
fn test_split_2() {
    let snailfish = "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]";
    let parsed = parse_snailfish(&snailfish);
    let action = action(&parsed);

    assert_eq!(
        action,
        Action::Split { range: &(4, 22..24) }
    );

    let result = execute(&snailfish, action).unwrap();
    assert_eq!(result, String::from("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]"));
}

fn reduce(snailfish: &str) -> String {
    let parsed = parse_snailfish(snailfish);
    let action = action(&parsed);

    println!("{:?}", action);
    match execute(snailfish, action) {
        Some(n) => {
            println!("{}", n);
            reduce(&n)
        },
        None => snailfish.to_string()
    }
}

#[test]
fn test_reduce() {
    let snailfish_1 = "[[[[4,3],4],4],[7,[[8,4],9]]]";
    let snailfish_2 = "[1,1]";
    let sum = add(snailfish_1, snailfish_2);
    let result = reduce(&sum);
    assert_eq!(result, "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
}

fn queue_sum(queue: &mut VecDeque<&str>) -> String {
    let start = queue.pop_front().unwrap();

    queue.iter().fold(String::from(start), |acc, next| {
        let sum = add(&acc, next);
        println!("{}", sum);
        reduce(&sum)
    })
}

#[test]
fn test_reduce_2() {
    let mut snailfish_numbers = VecDeque::from([
        "[1,1]",
        "[2,2]",
        "[3,3]",
        "[4,4]"
    ]);

    let result = queue_sum(&mut snailfish_numbers);
    assert_eq!(result, "[[[[1,1],[2,2]],[3,3]],[4,4]]");
}

#[test]
fn test_reduce_3() {
    let mut snailfish_numbers = VecDeque::from([
        "[1,1]",
        "[2,2]",
        "[3,3]",
        "[4,4]",
        "[5,5]"
    ]);

    let result = queue_sum(&mut snailfish_numbers);
    assert_eq!(result, "[[[[3,0],[5,3]],[4,4]],[5,5]]");
}

#[test]
fn test_reduce_4() {
    let mut snailfish_numbers = VecDeque::from([
        "[1,1]",
        "[2,2]",
        "[3,3]",
        "[4,4]",
        "[5,5]",
        "[6,6]"
    ]);

    let result = queue_sum(&mut snailfish_numbers);
    assert_eq!(result, "[[[[5,0],[7,4]],[5,5]],[6,6]]");
}


#[test]
fn test_reduce_complex_1() {
    let mut snailfish_numbers = VecDeque::from([
        "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
        "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
        //"[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
        //"[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
        //"[7,[5,[[3,8],[1,4]]]]",
        //"[[2,[2,2]],[8,[8,1]]]",
        //"[2,9]",
        //"[1,[[[9,3],9],[[9,0],[0,7]]]]",
        //"[[[5,[7,4]],7],1]",
        //"[[[[4,2],2],6],[8,7]]"
    ]);

    let result = queue_sum(&mut snailfish_numbers);
    assert_eq!(
        result,
        "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"
    );
}
