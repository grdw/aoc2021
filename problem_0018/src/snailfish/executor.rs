use crate::snailfish::{Snailfish, Action};
use core::ops::Range;

fn digit_split(input: &str) -> Vec<u8> {
    input.split(",").map(|n| n.parse::<u8>().unwrap()).collect()
}

impl Snailfish {
    fn explode(&self,
               pair: Range<usize>,
               left: Option<Range<usize>>,
               right: Option<Range<usize>>) -> Snailfish {

        let mut result = self.input.clone();
        let slice = &self.input[pair.start..pair.end];
        let to_explode: Vec<u8> = digit_split(slice);

        match (left, right) {
            (Some(lran), Some(rran)) => {
                let right_t = self.input[rran.start..rran.end].parse::<u8>().unwrap();
                let left_t = self.input[lran.start..lran.end].parse::<u8>().unwrap();

                let l_sum = format!("{}", to_explode[0] + left_t);
                let r_sum = format!("{}", to_explode[1] + right_t);

                result.replace_range(rran.start..rran.end, &r_sum);
                result.replace_range(lran.start..lran.end, &l_sum);

                if left_t < 10 && l_sum.len() == 2 {
                    result.replace_range(pair.start..pair.end + 2, "0");
                } else {
                    result.replace_range(pair.start-1..pair.end + 1, "0");
                }
            },
            (None, Some(ran)) => {
                let right_t = self.input[ran.start..ran.end].parse::<u8>().unwrap();
                let sum = format!("{}", to_explode[1] + right_t);

                result.replace_range(ran.start..ran.end, &sum);
                result.replace_range(pair.start-1..pair.end+1, "0");
            },
            (Some(ran), None) => {
                let left_t = self.input[ran.start..ran.end].parse::<u8>().unwrap();
                let sum = format!("{}", to_explode[0] + left_t);

                result.replace_range(ran.start..ran.end, &sum);
                if left_t < 10 && sum.len() == 2 {
                    result.replace_range(pair.start..pair.end+2, "0");
                } else {
                    result.replace_range(pair.start-1..pair.end + 1, "0");
                }
            },
            _ => panic!("Invalid action"),
        }

        Snailfish::new(&result)
    }

    fn split(&self, range: Range<usize>) -> Snailfish {
        let mut result = self.input.clone();
        let number = self.input[range.start..range.end].parse::<u8>().unwrap();
        let div = number / 2;
        let pair = format!("[{},{}]", div, number - div);

        result.replace_range(range.start..range.end, &pair);
        Snailfish::new(&result)
    }

    pub fn execute(&self) -> Option<Snailfish> {
        match self.action() {
            Action::Explode { pair, left, right } =>
                Some(self.explode(pair, left, right)),
            Action::Split { range } => Some(self.split(range)),
            _ => None
        }
    }
}

#[test]
fn test_execute() {
    let snailfish = Snailfish::new("[[[[[9,8],1],2],3],4]");
    let exploded = snailfish.execute().unwrap();

    assert_eq!(exploded.input, String::from("[[[[0,9],2],3],4]"));
}

#[test]
fn test_explode_2() {
    let snailfish = Snailfish::new("[7,[6,[5,[4,[3,2]]]]]");
    let exploded = snailfish.execute().unwrap();

    assert_eq!(exploded.input, String::from("[7,[6,[5,[7,0]]]]"))
}

#[test]
fn test_explode_3() {
    let snailfish = Snailfish::new("[[6,[5,[4,[3,2]]]],1]");
    let exploded = snailfish.execute().unwrap();

    assert_eq!(exploded.input, String::from("[[6,[5,[7,0]]],3]"))
}

#[test]
fn test_explode_4() {
    let snailfish = Snailfish::new("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
    let exploded = snailfish.execute().unwrap();

    assert_eq!(exploded.input, String::from("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"));
}

#[test]
fn test_explode_5() {
    let snailfish = Snailfish::new("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
    let exploded = snailfish.execute().unwrap();

    assert_eq!(exploded.input, String::from("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"));
}

#[test]
fn test_explode_6() {
    let snailfish = Snailfish::new("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]");
    let exploded = snailfish.execute().unwrap();

    assert_eq!(exploded.input, String::from("[[[[0,7],4],[15,[0,13]]],[1,1]]"));
}

#[test]
fn test_explode_7() {
    let snailfish = Snailfish::new("[[[[[1,1],[2,2]],[3,3]],[4,4]],[5,5]]");
    let exploded = snailfish.execute().unwrap();

    assert_eq!(exploded.input, String::from("[[[[0,[3,2]],[3,3]],[4,4]],[5,5]]"));

    let second_exploded = exploded.execute().unwrap();
    assert_eq!(second_exploded.input, String::from("[[[[3,0],[5,3]],[4,4]],[5,5]]"));
}

#[test]
fn test_explode_8() {
    let snailfish_1 = Snailfish::new("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]");
    let snailfish_2 = Snailfish::new("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]");
    let sum = snailfish_1 + snailfish_2;

    let result = sum.execute().unwrap();
    assert_eq!(
        result.input,
        String::from("[[[[4,0],[5,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]")
    );
}

#[test]
fn test_explode_9() {
    let snailfish = Snailfish::new("[[[[0,7],4],[7,[[8,4],4]]],[1,1]]");
    let result = snailfish.execute().unwrap();

    assert_eq!(result.input, String::from("[[[[0,7],4],[15,[0,8]]],[1,1]]"));
}

#[test]
fn test_explode_10() {
    let snailfish = Snailfish::new("[[[[0,7],4],[7,[[1,4],9]]],[1,1]]");
    let result = snailfish.execute().unwrap();

    assert_eq!(result.input, String::from("[[[[0,7],4],[8,[0,13]]],[1,1]]"));
}

#[test]
fn test_explode_11() {
    let snailfish = Snailfish::new("[[[[4,0],[5,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]");
    let result = snailfish.execute().unwrap();

    assert_eq!(result.input, String::from("[[[[4,0],[5,4]],[[0,[7,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]"));
}

#[test]
fn test_split() {
    let snailfish = Snailfish::new("[[[[0,7],4],[15,[0,13]]],[1,1]]");
    let result = snailfish.execute().unwrap();
    assert_eq!(result.input, String::from("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]"));
}
