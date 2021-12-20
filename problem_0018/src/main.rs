mod snailfish;

use std::fs;
use snailfish::Snailfish;

fn main() {
    let contents = fs::read_to_string("input")
                      .unwrap_or("".to_string());

    let readings: Vec<&str> = contents
        .split_terminator("\n")
        .collect();

    let snailfish = queue_sum(readings.clone());
    //println!("Part 1: {}", snailfish.magnitude());

    //let mut queue = VecDeque::from(readings.clone());
    //let result = queue_magnitude_sum(&mut queue);
    //println!("Part 2: {}", result);
}

fn queue_sum(mut queue: Vec<&str>) -> Snailfish {
    let start = queue.remove(0);
    let snailfish = Snailfish::new(start);

    queue.iter().fold(snailfish, |acc, next| {
        let sum = acc + Snailfish::new(next);
        sum.reduce()
    })
}

//fn magnitude(input: &str, index: &mut usize) -> String {
//    let parsed = parse_snailfish(input);
//    let mut p_input = String::from(input);
//
//    let (_, range) = &parsed[*index];
//    let (_, nrange) = &parsed[*index + 1];
//
//    if nrange.start - range.end == 1 {
//        let first = &input[range.start..range.end].parse::<u64>().unwrap();
//        let cons = &input[nrange.start..nrange.end].parse::<u64>().unwrap();
//        let sum = format!("{}", first * 3 + cons * 2);
//        p_input.replace_range(range.start-1..nrange.end+1, &sum);
//
//        if *index > 0 {
//            *index -= 1;
//        }
//    } else {
//        *index += 1;
//    }
//
//    //thread::sleep(Duration::from_millis(1000));
//    //println!("{:?} ---------------- {}", p_input, index);
//    if p_input.chars().nth(0).unwrap() == '[' {
//        magnitude(&p_input, index)
//    } else {
//        p_input
//    }
//}
//
//#[test]
//fn test_magnitude() {
//    let input = "[[1,2],[[3,4],5]]";
//    assert_eq!(magnitude(input, &mut 0), String::from("143"));
//
//    let input = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]";
//    assert_eq!(magnitude(input, &mut 0), String::from("1384"));
//}
//

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
