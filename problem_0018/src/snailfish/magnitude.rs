use crate::snailfish::{Snailfish, Action};

pub fn magnitude(snailfish: &Snailfish) -> u64 {
    match magnitude_action(snailfish) {
        Action::Magnitude { range } => {
            let slice: Vec<u64> = snailfish.input[range.start..range.end]
                .split(",")
                .map(|n| n.parse::<u64>().unwrap())
                .collect();

            let sum = format!("{}", slice[0] * 3 + slice[1] * 2);
            let mut result = snailfish.input.clone();
            result.replace_range(
                range.start - 1..range.end + 1,
                &sum
            );

            magnitude(&Snailfish::new(&result))
        },
        _ => snailfish.input.parse::<u64>().unwrap()
    }
}

fn magnitude_action(snailfish: &Snailfish) -> Action {
    let parts = snailfish.parse();

    for i in 0..parts.len() - 1 {
         let first = &parts[i];
         let next = &parts[i + 1];

         if next.range.start - first.range.end == 1 {
             return Action::Magnitude {
                 range: first.range.start..next.range.end
             }
         }
    }

    Action::NonAction
}

#[test]
fn test_magnitude() {
    let input = Snailfish::new("[[1,2],[[3,4],5]]");
    assert_eq!(magnitude(&input), 143);

    let input = Snailfish::new("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    assert_eq!(magnitude(&input), 1384);

    let input = Snailfish::new("[[[[7,8],[6,6]],[[6,0],[7,7]]],[[[7,8],[8,8]],[[7,9],[0,6]]]]");

    assert_eq!(magnitude(&input), 3993);
}
