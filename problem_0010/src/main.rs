use std::fs;
use std::collections::HashMap;

fn main() {
    let contents = fs::read_to_string("input")
                      .unwrap_or("".to_string());

    let readings: Vec<&str> = contents
        .split_terminator("\n")
        .collect();

    println!("Invalid first chars {:?}", parse_navigation_subsystem(&readings));
    println!("Autocomplete score {:?}", autocomplete_subsystem(&readings));
}

fn parse(input: &Vec<&str>) -> Vec<(Vec<char>, Vec<char>)> {
    let map = HashMap::from([
        ('}', '{'),
        (')', '('),
        ('>', '<'),
        (']', '['),
    ]);

    let mut result = vec![];

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

        result.push((opens, closes));
    }

    result
}

fn parse_navigation_subsystem(input: &Vec<&str>) -> u64 {
    let points = HashMap::from([
        ('}', 1197),
        (')', 3),
        ('>', 25137),
        (']', 57),
    ]);


    let mut subtotal = 0;
    let parsed = parse(input);

    for (_, closes) in parsed {
        if closes.len() > 0 {
            subtotal += points.get(&closes[0]).unwrap();
        }
    }

    subtotal
}

fn autocomplete_subsystem(input: &Vec<&str>) -> u64 {
    let mut scores = vec![];
    let complete_points = HashMap::from([
        ('{', 3),
        ('(', 1),
        ('<', 4),
        ('[', 2),
    ]);

    let parsed = parse(&input);

    for (opens, closes) in parsed {
        if closes.is_empty() {
            let mut autocomplete_total = 0;
            for open in opens.iter().rev() {
                autocomplete_total *= 5;
                autocomplete_total += complete_points.get(open).unwrap();
            }

            scores.push(autocomplete_total)
        }
    }

    scores.sort();
    scores[(scores.len() / 2)]
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

    assert_eq!(parse_navigation_subsystem(&example), 26397);
    assert_eq!(autocomplete_subsystem(&example), 288957);
}
