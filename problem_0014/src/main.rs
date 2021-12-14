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

fn parse(
    template: &String,
    rules: &HashMap<&str, char>,
    count: usize) -> u128 {

    let mut counts: HashMap<char, u128> = HashMap::new();
    let mut cycle_counts: HashMap<&str, u128> = HashMap::new();

    for key in rules.keys() {
        cycle_counts.insert(*key, 0);
    }

    let mut prev_counts = cycle_counts.clone();

    // Setup
    for c in template.chars() {
        let p = counts.entry(c).or_insert(0);
        *p += 1
    }

    for i in 0..template.len() - 1 {
        let key = &template[i..i + 2];

        match cycle_counts.get_mut(key) {
            Some(p) => *p += 1,
            None => ()
        }
    }

    let keys: Vec<&&str> = rules.keys().collect();

    for _ in 0..count-1 {
        let mut diff = HashMap::new();

        for i in 0..keys.len() {
            let k = keys[i];
            let v = cycle_counts.get(k).unwrap();
            let prev_v = prev_counts.get(k).unwrap();

            if v > prev_v {
                diff.insert(k, v - prev_v);
            }
        }

        prev_counts = cycle_counts.clone();

        for (k, v) in &diff {
            let p = rules.get(&k as &str).unwrap();
            let l = format!("{}{}", k.chars().nth(0).unwrap(), p);
            let r = format!("{}{}", p, k.chars().nth(1).unwrap());

            match cycle_counts.get_mut(&l as &str) {
                Some(p) => *p += v,
                None => ()
            }

            match cycle_counts.get_mut(&r as &str) {
                Some(p) => *p += v,
                None => ()
            }
        }
    }

    for (k, v) in &cycle_counts {
        match rules.get(k) {
            Some(c) => {
                match counts.get_mut(c) {
                    Some(n) => *n += v,
                    None => { counts.insert(*c, *v); }
                }
            },
            None => ()
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
