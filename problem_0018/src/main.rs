mod snailfish;

use std::fs;
use snailfish::Snailfish;
use snailfish::magnitude;

fn main() {
    let contents = fs::read_to_string("input")
                      .unwrap_or("".to_string());

    let readings: Vec<&str> = contents
        .split_terminator("\n")
        .collect();

    let snailfish = queue_sum(readings.clone());
    println!("Part 1: {}", magnitude::magnitude(&snailfish));

    let result = max_magnitude(readings.clone());
    println!("Part 2: {}", result);
}

fn queue_sum(mut queue: Vec<&str>) -> Snailfish {
    let start = queue.remove(0);
    let snailfish = Snailfish::new(start);

    queue.iter().fold(snailfish, |acc, next| {
        let sum = acc + Snailfish::new(next);
        sum.reduce()
    })
}

#[test]
fn test_queue_sum() {
    let mut snailfish_numbers = vec![
        "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
        "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
        "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
        "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
        "[7,[5,[[3,8],[1,4]]]]",
        "[[2,[2,2]],[8,[8,1]]]",
        "[2,9]",
        "[1,[[[9,3],9],[[9,0],[0,7]]]]",
        "[[[5,[7,4]],7],1]",
        "[[[[4,2],2],6],[8,7]]"
    ];

    let result = queue_sum(snailfish_numbers.clone());
    assert_eq!(
        &result.input,
        "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
    );
}

fn max_magnitude(queue: Vec<&str>) -> u64 {
    let mut max = 0;

    for p in &queue {
        for n in &queue {
            if p == n {
                continue
            }

            let snailfish_1 = Snailfish::new(p);
            let snailfish_2 = Snailfish::new(n);
            let sum = snailfish_1 + snailfish_2;
            let magnitude = magnitude::magnitude(&sum.reduce());

            if magnitude > max {
                max = magnitude
            }
        }
    }

    max
}

#[test]
fn test_max_magnitude() {
    let mut snailfish_numbers = vec![
		"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
		"[[[5,[2,8]],4],[5,[[9,9],0]]]",
		"[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
		"[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
		"[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
		"[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
		"[[[[5,4],[7,7]],8],[[8,3],8]]",
		"[[9,3],[[9,9],[6,[4,9]]]]",
		"[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
		"[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"
	];

    assert_eq!(max_magnitude(snailfish_numbers.clone()), 3993);
}
