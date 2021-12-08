use std::fs;

fn main() {
	let display_string = fs::read_to_string("input")
                            .unwrap_or("".to_string());

    let measurements: Vec<&str> = display_string
        .split_terminator("\n")
        .collect();

    let count = unique_segments(&measurements);
    println!("The amount of unique segments: {:?}", count);
}

fn unique_segments(input: &Vec<&str>) -> u32 {
    let valid_lengths = vec![2, 3, 4, 7];
    let mut count = 0;

    for measurement in input {
        let parsed: Vec<&str> = measurement.split(" | ").collect();
        let digits: Vec<&str> = parsed[1].split(" ").collect();

        for d in &digits {
            if valid_lengths.contains(&d.len()) {
                count += 1
            }
        }
    }
    count
}

use std::collections::HashMap;

struct Display {
    measurements: Vec<String>
}

fn heap_char(mut vector: Vec<char>) -> Vec<Vec<char>> {
    let mut result: Vec<usize> = vec![0; vector.len()];
    let mut i = 0;

    println!("{:?}", result);
    while i < vector.len() {
        if result[i] < i {
            if i % 2 == 0 {
                vector.swap(0, i);
            } else {
                vector.swap(result[i], i);
            }

            println!("{:?}", result);
            result[i] += 1;
            i = 0;
        } else {
            result[i] = 0;
            i += 1
        }
    }

    vec![vec!['a']]
}

// IDEA is
// Get all perms of a till g
// filter out all perms where:
// 1 (2, 5) or (5, 2)
// 7 (2, 5, 0) or (5, 2, 0)
// etc. etc.
fn sum_digit_values(input: &Vec<&str>) -> u64 {
    let results = heap_char("abcdefg".chars().collect());
    println!("{:?}", results);

    for measurement in input {
        let parsed: Vec<Vec<String>> = measurement
            .split(" | ")
            .map(|t| t.split(" ").map(|n| String::from(n)).collect() )
            .collect();
    }

    0
}

#[test]
fn test_broken_seven_segment_display() {
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
