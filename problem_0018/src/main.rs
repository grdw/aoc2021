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

#[test]
fn test_parse_snailfish_explode_1() {
    let snailfish = "[[[[[9,8],1],2],3],4]";

    assert_eq!(
        action(&snailfish),
        Action::Explode { pair: 4, left: None, right: Some(10) }
    );
    //String::from("[[[[0,9],2],3],4]")
}

#[test]
fn test_parse_snailfish_explode_2() {
    let snailfish = "[7,[6,[5,[4,[3,2]]]]]";

    assert_eq!(
        action(&snailfish),
        Action::Explode { pair: 12, left: Some(10), right: None }
    );
    //String::from("[7,[6,[5,[7,0]]]]")
}

#[test]
fn test_parse_snailfish_explode_3() {
    let snailfish = "[[6,[5,[4,[3,2]]]],1]";

    assert_eq!(
        action(&snailfish),
        Action::Explode { pair: 10, left: Some(8), right: Some(19) }
    );

    //String::from("[[6,[5,[7,0]]],3]")
}

#[test]
fn test_parse_snailfish_explode_4() {
    let snailfish = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]";

    assert_eq!(
        action(&snailfish),
        Action::Explode { pair: 10, left: Some(8), right: Some(20) }
    );
    //String::from("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")
}

#[test]
fn test_parse_snailfish_explode_5() {
    let snailfish = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]";

    assert_eq!(
        action(&snailfish),
        Action::Explode { pair: 24, left: Some(22), right: None }
    )
    //String::from("[[3,[2,[8,0]]],[9,[5,[7,0]]]]")
}
