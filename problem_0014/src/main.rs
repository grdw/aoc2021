use std::fs;
use std::collections::HashMap;

type Rules<'a> = HashMap<&'a str, char>;

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

fn parse(template: &String, rules: &Rules, count: usize) -> u128 {
    let keys: Vec<&&str> = rules.keys().collect();
    let mut prev_arr: Vec<u128> = vec![0; keys.len()];
    let mut curr_arr: Vec<u128> = vec![0; keys.len()];

    for i in 0..template.len() - 1 {
        let key = &template[i..i + 2];
        let pos = keys.iter().position(|&&k| k == key).unwrap();

        curr_arr[pos] += 1;
    }

    for _ in 0..count-1 {
        let diff: Vec<u128> = curr_arr
            .iter()
            .enumerate()
            .map(|(i, b)| b - prev_arr[i])
            .collect();

        prev_arr = curr_arr.clone();

        for i in 0..diff.len() {
            let key = keys[i];
            let change = rules.get(key).unwrap();

            let left = format!("{}{}", &key[0..1], change);
            let right = format!("{}{}", change, &key[1..2]);

            let left_pos = keys.iter().position(|&&k| k == left).unwrap();
            let right_pos = keys.iter().position(|&&k| k == right).unwrap();
            curr_arr[left_pos] += diff[i];
            curr_arr[right_pos] += diff[i];
        }
    }

    count_chars(template, rules, &curr_arr, &keys)
}

fn count_chars(
    template: &String,
    rules: &Rules,
    curr_arr: &Vec<u128>,
    keys: &Vec<&&str>) -> u128 {

    let mut counts: HashMap<char, u128> = HashMap::new();
    // Count the initial characters of "template"
    for c in template.chars() {
        let p = counts.entry(c).or_insert(0);
        *p += 1
    }

    // Add the characters from the amount of cycles
    for i in 0..curr_arr.len() {
        let key = keys[i];
        let cha = rules.get(key).unwrap();
        let p = counts.entry(*cha).or_insert(0);
        *p += curr_arr[i]
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
