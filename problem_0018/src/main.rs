use std::fs;

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

fn explode(input: &str) -> String {
    let mut s = String::new();
    let mut depth = 0;
    let mut p = 0;

    for c in input.chars() {
        if c == '[' {
            depth += 1;
        } else if c == ']' {
            depth -= 1;
        }

        if depth == 5 {
            let exploded_pair = &input[p..p + 5];
            println!("{}", exploded_pair);
            break;
        }

        p += 1;
    }

    s
}

#[test]
fn test_parse_snailfish_explode_1() {
    let snailfish = "[[[[[9,8],1],2],3],4]";

    assert_eq!(explode(&snailfish), String::from("[[[[0,9],2],3],4]"));
}

#[test]
fn test_parse_snailfish_explode_2() {
    let snailfish = "[7,[6,[5,[4,[3,2]]]]]";

    assert_eq!(explode(&snailfish), String::from("[7,[6,[5,[7,0]]]]"));
}

#[test]
fn test_parse_snailfish_explode_3() {
    let snailfish = "[[6,[5,[4,[3,2]]]],1]";

    assert_eq!(explode(&snailfish), String::from("[[6,[5,[7,0]]],3]"));
}

#[test]
fn test_parse_snailfish_explode_4() {
    let snailfish = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]";

    assert_eq!(
        explode(&snailfish),
        String::from("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")
    );
}

#[test]
fn test_parse_snailfish_explode_5() {
    let snailfish = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]";

    assert_eq!(
        explode(&snailfish),
        String::from("[[3,[2,[8,0]]],[9,[5,[7,0]]]]")
    )
}
