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

    let template = String::from("SCSCSKKVVBKVFKSCCSOV");
    let count = parse(&template, &rules, 10);

    println!("Part 1: {}", count);

    let count = parse(&template, &rules, 40);
    println!("Part 2: {}", count);
}

fn parse(template: &String,
    rules: &HashMap<&str, char>,
    count: usize) -> u128 {

    let keys: Vec<&&str> = rules.keys().collect();
    let mut cycle_counts: HashMap<&str, u128> = HashMap::new();
    let mut prev_counts: HashMap<&str, u128> = HashMap::new();

    for key in &keys {
        cycle_counts.insert(*key, 0);
    }

    for i in 0..template.len() - 1 {
        let key = &template[i..i + 2];

        if let Some(p) = cycle_counts.get_mut(key) {
            *p += 1
        }
    }

    for _ in 0..count-1 {
        let mut diff = HashMap::new();

        for i in 0..keys.len() {
            let k = keys[i];
            let v = cycle_counts.get(k).unwrap_or(&0);
            let prev_v = prev_counts.get(k).unwrap_or(&0);

            if v > prev_v {
                diff.insert(k, v - prev_v);
            }
        }

        prev_counts = cycle_counts.clone();

        for (k, v) in &diff {
            let p = rules.get(&k as &str).unwrap();
            let l = format!("{}{}", k.chars().nth(0).unwrap(), p);
            let r = format!("{}{}", p, k.chars().nth(1).unwrap());

            if let Some(p) = cycle_counts.get_mut(&l as &str) {
                *p += v
            }

            if let Some(p) = cycle_counts.get_mut(&r as &str) {
                *p += v
            }
        }
    }

    let mut counts: HashMap<char, u128> = HashMap::new();
    // Count the initial characters of "template"
    for c in template.chars() {
        let p = counts.entry(c).or_insert(0);
        *p += 1
    }

    // Add the characters from the amount of cycles
    for (k, v) in &cycle_counts {
        if let Some(c) = rules.get(k) {
            let p = counts.entry(*c).or_insert(0);
            *p += v
        }
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

    let count = parse(&mut start, &parse_rules, 10);
    assert_eq!(count, 1588);

    let count = parse(&mut start, &parse_rules, 40);
    assert_eq!(count, 2188189693529);
}
