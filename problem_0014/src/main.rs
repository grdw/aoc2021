use std::fs;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("input")
                   .unwrap_or("".to_string());

    let mut rules = HashMap::new();

    for line in input.split_terminator("\n") {
        let key_val: Vec<&str> = line.split(" -> ").collect();

        rules.insert(
            key_val[0],
            key_val[1].chars().nth(0).unwrap()
        );
    }

    let mut template = String::from("SCSCSKKVVBKVFKSCCSOV");
    for _ in 0..10 {
        parse(&mut template, &rules);
    }

    println!("Part 1: {}", quantify(template));
}

fn parse(template: &mut String, rules: &HashMap<&str, char>) {
    let mut insertions = vec![];

    for i in 0..template.len() - 1 {
        let key = &template[i..i + 2];

        match rules.get(key) {
            Some(c) => insertions.insert(0, (i + 1, c)),
            None => ()
        }
    }

    for (i, c) in insertions {
        template.insert(i, *c);
    }
}

fn quantify(template: String) -> usize {
    let mut counts = HashMap::new();
    for c in template.chars() {
        let p = counts.entry(c).or_insert(0);
        *p += 1
    }

    let min = counts.values().min().unwrap();
    let max = counts.values().max().unwrap();

    max - min
}

#[test]
fn test_rules() {
    let mut start = String::from("NNCB");
    let parse_rules = HashMap::from([
        ("CH", 'B'),
        ("HH", 'N'),
        ("CB", 'H'),
        ("NH", 'C'),
        ("HB", 'C'),
        ("HC", 'B'),
        ("HN", 'C'),
        ("NN", 'C'),
        ("BH", 'H'),
        ("NC", 'B'),
        ("NB", 'B'),
        ("BN", 'B'),
        ("BB", 'N'),
        ("BC", 'B'),
        ("CC", 'N'),
        ("CN", 'C')
    ]);

    for _ in 0..10 {
        parse(&mut start, &parse_rules);
    }
    assert_eq!(start.len(), 3073);
    assert_eq!(quantify(start), 1588);
}
