use crate::snailfish::{Snailfish, Action};

impl Snailfish {
    pub fn action(&self) -> Action {
        let parts = self.parse();

        for i in 0..parts.len() - 1 {
            let part = &parts[i];
            let next_part = &parts[i + 1];

            if part.depth > 4 {
                let left = if i > 0 {
                    let part = &parts[i - 1];
                    Some(part.range.clone())
                } else {
                    None
                };

                let right = match parts.get(i + 2) {
                    Some(part) => Some(part.range.clone()),
                    None => None
                };

                return Action::Explode {
                    pair: part.range.start..next_part.range.end,
                    left: left,
                    right: right
                }
            }
        }

        for i in 0..parts.len() {
            let part = &parts[i];

            let number = self
                .input[part.range.clone()]
                .parse::<u8>()
                .unwrap();

            if number > 9 {
                return Action::Split {
                    range: part.range.clone()
                }
            }
        }

        Action::NonAction
    }
}

#[test]
fn test_action_explode() {
    let snailfish = Snailfish::new("[[[[[9,8],1],2],3],4]");
    let action = snailfish.action();

    assert_eq!(action, Action::Explode {
        pair: 5..8,
        left: None,
        right: Some(10..11)
    });
}

#[test]
fn test_action_no_action() {
    let snailfish = Snailfish::new("[2,4]");
    let action = snailfish.action();

    assert_eq!(action, Action::NonAction);
}

#[test]
fn test_action_split() {
    let snailfish = Snailfish::new("[[[[0,7],4],[15,[0,13]]],[1,1]]");
    let action = snailfish.action();

    assert_eq!(action, Action::Split { range: 13..15 });
}
