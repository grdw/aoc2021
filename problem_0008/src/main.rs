use std::fs;

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
    let map: Vec<Vec<usize>> = vec![
        vec![1],
        vec![7],
        vec![4],
        vec![2,3,5],
        vec![0,6,9],
    ];

    let ind = val.len() - 2;

    // in case an 8 is present
    if ind == 5 {
        return true
    }

    match map.get(ind) {
        Some(positions) => {
            positions.iter().any(|p| {
                let perms = heap(POS[*p].to_vec());

                perms.iter().any(|c| {
                    let string: String = c
                        .into_iter()
                        .map(|n| chars[*n])
                        .collect();

                    &string == val
                })
            })
        },
        None => false
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
    assert_eq!(valid_perm(&perm, "acedgfb"), true); //cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"
}

fn heap<T: Clone>(mut vector: Vec<T>) -> Vec<Vec<T>> {
    let mut result: Vec<usize> = vec![0; vector.len()];
    let mut total = vec![];
    let mut i = 0;

    total.push(vector.clone());

    while i < vector.len() {
        if result[i] < i {
            if i % 2 == 0 {
                vector.swap(0, i);
            } else {
                vector.swap(result[i], i);
            }
            total.push(vector.clone());
            result[i] += 1;
            i = 0;
        } else {
            result[i] = 0;
            i += 1
        }
    }

    total
}

// IDEA is
// Get all perms of a till g
// filter out all perms where:
//   0
//  1 2
//   3
//  4 5
//   6
//
// 1 (2, 5) or (5, 2)
// 7 (2, 5, 0) or (5, 2, 0)
// etc. etc.
fn sum_digit_values(input: &Vec<&str>) -> u64 {
    let mut sum = 0;
    let heap_perms = heap("abcdefg".chars().collect());

    for measurement in input {
        let parsed: Vec<&str> = measurement.split(" | ").collect();
        let digits: Vec<&str> = parsed[1].split(" ").collect();
        let mut perms = heap_perms.clone();
        let mut tens: Vec<&str> = parsed[0].split(" ").collect();
        tens.sort_by_key(|t| t.len());

        for t in tens {
            perms.retain(|perm| valid_perm(perm, t));
        }

        if perms.len() > 1 {
            panic!("BUG!");
        }

        let final_perm = &perms[0];

        for (i, d) in digits.iter().enumerate() {
            let mut pos: Vec<usize> = d
                .chars()
                .map(|n| final_perm.iter().position(|t| *t == n).unwrap()).
                collect();

            pos.sort();

            match POS.iter().position(|t| *t == pos) {
                Some(n) => sum += (n as u64 * 10_u64.pow((3 - i) as u32)),
                None => panic!("Also a bug")
            }
        }
    }

    sum
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
