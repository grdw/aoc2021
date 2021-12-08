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

    assert_eq!(unique_segments(&input), 26)
}
