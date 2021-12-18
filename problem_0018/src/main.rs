use std::fs;

const PAIR_LENGTH: usize = 5;

#[derive(Debug, Eq, PartialEq)]
enum Action {
    Explode { pair: usize, left: Option<usize>, right: Option<usize> },
    Split,
    NonAction
}

fn main() {
    let contents = fs::read_to_string("input")
                      .unwrap_or("".to_string());

    let readings: Vec<&str> = contents
        .split_terminator("\n")
        .collect();
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

fn action(input: &str) -> Action {
    let mut action = Action::NonAction;
    let mut depth = 0;
    let mut left_index = None;

    for (p, c) in input.chars().enumerate() {
        match c {
            '[' => depth += 1,
            ']' => depth -= 1,
            '0'..='9' => left_index = Some(p),
            _ => ()
        }

        if depth == 5 {
            let mut right_index = None;

            for i in (p + PAIR_LENGTH)..input.len() {
                let cha = input.chars().nth(i).unwrap();

                match cha {
                    '0'..='9' => {
                        right_index = Some(i);
                        break;
                    },
                    _ => ()
                }
            }

            action = Action::Explode {
                pair: p,
                left: left_index,
                right: right_index
            };

            break;
        }
    }

    action
}

fn execute(input: &str, action: Action) -> String {
    let mut result = String::new();

    match action {
        Action::Explode { pair, left, right } => {
            result.push_str(&input[0..pair]);

            let to_explode: Vec<u8> = input[pair + 1..pair + 4]
                .split(",")
                .map(|n| n.parse::<u8>().unwrap())
                .collect();

            if let Some(li) = left {
                let left_t = input[li..li+1].parse::<u8>().unwrap();
                let sum = format!("{}", to_explode[0] + left_t);
                result.replace_range(pair - 2..pair, &sum);
            } else {
                result.push('0');
            };

            result.push(',');

            if let Some(ri) = right {
                let right_t = input[ri..ri+1].parse::<u8>().unwrap();
                let sum = format!("{}", to_explode[1] + right_t);

                let bits = &input[pair + PAIR_LENGTH..ri];
                let control_bits = &input[pair + PAIR_LENGTH..ri-1];

                if !control_bits.is_empty() {
                    result.push('0');
                    result.push_str(bits);
                }

                result.push_str(&sum);
                result.push_str(&input[ri + 1..]);
            } else {
                result.push('0');
            };

            if right.is_none() {
                result.push_str(&input[pair + 5..]);
            }
        },
        _ => ()
    }
    result
}

#[test]
fn test_parse_snailfish_explode_1() {
    let snailfish = "[[[[[9,8],1],2],3],4]";
    let action = action(&snailfish);

    assert_eq!(action, Action::Explode { pair: 4, left: None, right: Some(10) });

    let result = execute(&snailfish, action);
    assert_eq!(result, String::from("[[[[0,9],2],3],4]"));
}

#[test]
fn test_parse_snailfish_explode_2() {
    let snailfish = "[7,[6,[5,[4,[3,2]]]]]";
    let action = action(&snailfish);

    assert_eq!(action, Action::Explode { pair: 12, left: Some(10), right: None });

    let result = execute(&snailfish, action);
    assert_eq!(result, String::from("[7,[6,[5,[7,0]]]]"))
}

#[test]
fn test_parse_snailfish_explode_3() {
    let snailfish = "[[6,[5,[4,[3,2]]]],1]";
    let action = action(&snailfish);

    assert_eq!(
        action,
        Action::Explode { pair: 10, left: Some(8), right: Some(19) }
    );

    let result = execute(&snailfish, action);
    assert_eq!(result, String::from("[[6,[5,[7,0]]],3]"))
}

#[test]
fn test_parse_snailfish_explode_4() {
    let snailfish = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]";
    let action = action(&snailfish);

    assert_eq!(
        action,
        Action::Explode { pair: 10, left: Some(8), right: Some(20) }
    );

    let result = execute(&snailfish, action);
    assert_eq!(result, String::from("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"));
}

#[test]
fn test_parse_snailfish_explode_5() {
    let snailfish = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]";
    let action = action(&snailfish);
    assert_eq!(
        action,
        Action::Explode { pair: 24, left: Some(22), right: None }
    );

    let result = execute(&snailfish, action);
    assert_eq!(result, String::from("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"));
}
