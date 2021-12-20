use crate::snailfish::Snailfish;
use core::ops::Range;

#[derive(Debug, Eq, PartialEq)]
pub enum Action {
    Explode {
        pair: Range<usize>,
        left: Option<Range<usize>>,
        right: Option<Range<usize>>
    },
    Split { range: Range<usize> },
    NonAction
}

pub fn get_action(snailfish: &Snailfish) -> Action {
    let parts = snailfish.parse();

    for i in 0..parts.len() - 1 {
        let part = &parts[i];
        let next_part = &parts[i + 1];
        let number = snailfish
            .input[part.range.clone()]
            .parse::<u8>()
            .unwrap();

        if part.depth > 4 {
            let left = if i > 0 {
                let l = &parts[i - 1];
                Some(l.range.clone())
            } else {
                None
            };

            let right_part = &parts[i + 2];

            return Action::Explode {
                pair: part.range.start..next_part.range.end,
                left: left,
                right: Some(right_part.range.clone())
            }
        }

        if number > 9 {
            return Action::Split {
                range: part.range.clone()
            }
        }
    }

    Action::NonAction
}

#[test]
fn test_action_explode() {
    let snailfish = Snailfish::new("[[[[[9,8],1],2],3],4]");
    let action = get_action(&snailfish);

    assert_eq!(action, Action::Explode {
        pair: 5..8,
        left: None,
        right: Some(10..11)
    });
}

#[test]
fn test_action_no_action() {
    let snailfish = Snailfish::new("[2,4]");
    let action = get_action(&snailfish);

    assert_eq!(action, Action::NonAction);
}

#[test]
fn test_action_split() {
    let snailfish = Snailfish::new("[[[[0,7],4],[15,[0,13]]],[1,1]]");
    let action = get_action(&snailfish);

    assert_eq!(action, Action::Split { range: 13..15 });
}
