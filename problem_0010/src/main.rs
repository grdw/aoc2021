use std::fs;
use std::collections::HashMap;

fn main() {
    let contents = fs::read_to_string("input")
                      .unwrap_or("".to_string());

    let readings: Vec<&str> = contents
        .split_terminator("\n")
        .collect();

    println!("{:?}", readings);
}

fn parse_navigation_subsystem(input: &Vec<&str>) -> u64 {
    let map = HashMap::from([
        ('}', '{'),
        (')', '('),
        ('>', '<'),
        (']', '['),
    ]);

    let points = HashMap::from([
        ('}', 1197),
        (')', 3),
        ('>', 25137),
        (']', 57),
    ]);

    let mut subtotal = 0;

    for line in input {
        let mut opens = vec![];
        let mut closes = vec![];

        for ch in line.chars() {
            match ch {
                '[' | '(' | '{' | '<' => opens.push(ch),
                ']' | ')' | '}' | '>' => {
                    let opener = map.get(&ch).unwrap();
                    if opens.last().unwrap() == opener {
                        opens.pop();
                    } else {
                        closes.push(ch)
                    }
                },
                _ => panic!("Invalid syntax")
            }

        }

        if closes.len() > 0 {
            subtotal += points.get(&closes[0]).unwrap();
        }
    }
    subtotal
}

#[test]
fn test_parse_navigation_subsystem() {
    let example = vec![
        "[({(<(())[]>[[{[]{<()<>>",
        "[(()[<>])]({[<{<<[]>>(",
        "{([(<{}[<>[]}>{[]{[(<()>",
        "(((({<>}<{<{<>}{[]{[]{}",
        "[[<[([]))<([[{}[[()]]]",
        "[{[{({}]{}}([{[{{{}}([]",
        "{<[[]]>}<{[{[{[]{()[[[]",
        "[<(<(<(<{}))><([]([]()",
        "<{([([[(<>()){}]>(<<{{",
        "<{([{{}}[<[[[<>{}]]]>[]]"
    ];

    assert_eq!(parse_navigation_subsystem(&example), 26397)
}
