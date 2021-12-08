use std::fs;

// Assume the display is the format of:
//   0
//  1 2
//   3
//  4 5
//   6
const POS: &'static [&'static[usize]] = &[
    &[0,1,2,4,5,6],   // 0
    &[2,5],           // 1
    &[0,2,3,4,6],     // 2
    &[0,2,3,5,6],     // 3
    &[1,2,3,5],       // 4
    &[0,1,3,5,6],     // 5
    &[0,1,3,4,5,6],   // 6
    &[0,2,5],         // 7
    &[0,1,2,3,4,5,6], // 8
    &[0,1,2,3,5,6]    // 9
];

const LENGTH_MAP: &'static [&'static[usize]] = &[
    &[1],
    &[7],
    &[4],
    &[2,3,5],
    &[0,6,9]
];

fn main() {
	let display_string = fs::read_to_string("input")
                            .unwrap_or("".to_string());

    let measurements: Vec<&str> = display_string
        .split_terminator("\n")
        .collect();

    let count = unique_segments(&measurements);
    println!("The amount of unique segments: {:?}", count);

    let count = sum_digit_values(&measurements);
    println!("The amount of unique segments: {:?}", count);
}

fn unique_segments(input: &Vec<&str>) -> usize {
    let valid_lengths = vec![2, 3, 4, 7];

    input.iter().fold(0, |acc, measurement| {
        let parsed: Vec<&str> = measurement.split(" | ").collect();

        acc + parsed[1]
            .split(" ")
            .filter(|d| valid_lengths.contains(&d.len()))
            .count()
    })
}

fn valid_perm(chars: &Vec<char>, val: &str) -> bool {
    let ind = val.len() - 2;
    let mut sorted_val: Vec<char> = val.chars().collect();
    sorted_val.sort();

    match LENGTH_MAP.get(ind) {
        Some(positions) => {
            positions.iter().any(|p| {
                let perms = POS[*p];
                let mut sorted_string: Vec<char> = perms
                    .into_iter()
                    .map(|n| chars[*n])
                    .collect();

                sorted_string.sort();
                sorted_val == sorted_string
            })
        },
        None => true
    }
}

#[test]
fn test_valid_perm() {
    let perm = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    assert_eq!(valid_perm(&perm, "be"), false);
    assert_eq!(valid_perm(&perm, "cf"), true);
    assert_eq!(valid_perm(&perm, "cbdgef"), false);
    assert_eq!(valid_perm(&perm, "acdeg"), true);
    assert_eq!(valid_perm(&perm, "acdfg"), true);
    assert_eq!(valid_perm(&perm, "abdfg"), true);
    assert_eq!(valid_perm(&perm, "acedgfb"), true);
}

fn heap_with_prefix(mut vector: Vec<char>, prefix: char) -> Vec<Vec<char>> {
    let mut result: Vec<usize> = vec![0; vector.len()];
    let mut total = vec![];
    let mut i = 0;

    let mut copy = vector.clone();
    copy.insert(0, prefix);
    total.push(copy);

    while i < vector.len() {
        if result[i] < i {
            if i % 2 == 0 {
                vector.swap(0, i);
            } else {
                vector.swap(result[i], i);
            }

            let mut copy = vector.clone();
            copy.insert(0, prefix);
            total.push(copy);

            result[i] += 1;
            i = 0;
        } else {
            result[i] = 0;
            i += 1
        }
    }

    total
}

fn display_formation<'a>(tens: &'a Vec<&str>) -> Option<Vec<char>> {
    let mut mask: Vec<char> = "abcdefg".chars().collect();
    let mut prefix = ' ';

    for c in tens[1].chars() {
        if !tens[0].chars().any(|l| l == c) {
            mask.retain(|&x| x != c);
            prefix = c;
            break;
        }
    }

    let mut perms = heap_with_prefix(mask, prefix);

    for t in tens {
        perms.retain(|perm| valid_perm(perm, t));
    }

    if perms.len() != 1 {
        panic!("There can only be 1 formation!");
    }

    perms.first().cloned()
}

fn sum_digit_values(input: &Vec<&str>) -> u64 {
    input.iter().fold(0, |total_acc, measurement| {
        let parsed: Vec<&str> = measurement.split(" | ").collect();
        let digits: Vec<&str> = parsed[1].split(" ").collect();
        let mut tens: Vec<&str> = parsed[0].split(" ").collect();
        tens.sort_by_key(|t| t.len());

        let final_perm = display_formation(&tens).unwrap();
        let four_digit_num = digits.iter().enumerate().fold(0, |acc, (i, d)| {
            let mut pos: Vec<usize> = d
                .chars()
                .map(|n| final_perm.iter().position(|t| *t == n).unwrap()).
                collect();

            pos.sort();

            let pos_in = POS
                .iter()
                .position(|t| *t == pos)
                .unwrap();

            acc + (pos_in as u64 * 10_u64.pow((3 - i) as u32))
        });

        total_acc + four_digit_num
    })
}

#[test]
fn test_broken_seven_segment_display() {
    let input = vec![
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"
    ];

    assert_eq!(sum_digit_values(&input), 5353);

    let input = vec![
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
        "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
        "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
        "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
        "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
        "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
        "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
        "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
        "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
        "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
    ];

    assert_eq!(unique_segments(&input), 26);
    assert_eq!(sum_digit_values(&input), 61229)
}
