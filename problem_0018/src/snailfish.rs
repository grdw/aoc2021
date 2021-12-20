mod action;
mod parser;

use std::ops::Add;

pub struct Snailfish {
    input: String
}

impl Snailfish {
    fn new(input: &str) -> Snailfish {
        Snailfish { input: String::from(input) }
    }
}

impl Add for Snailfish {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut result = String::new();
        result.push('[');
        result.push_str(&self.input);
        result.push(',');
        result.push_str(&other.input);
        result.push(']');

        Self {
            input: result
        }
    }
}

#[test]
fn test_add() {
    let snailfish_1 = Snailfish::new("[1,2]");
    let snailfish_2 = Snailfish::new("[[3,4],5]");
    let sum = snailfish_1 + snailfish_2;

    assert_eq!(
        sum.input,
        String::from("[[1,2],[[3,4],5]]")
    )
}
