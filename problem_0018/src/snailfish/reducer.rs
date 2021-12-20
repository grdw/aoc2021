use crate::snailfish::Snailfish;

impl Snailfish {
    fn reduce(&self) -> Snailfish {
        let exec_action = self.action();

        match self.execute() {
            Some(snailfish) => snailfish.reduce(),
            None => Snailfish::new(&self.input)
        }
    }
}

#[test]
fn test_reduce_simple() {
    let snailfish_1 = Snailfish::new("[[[[4,3],4],4],[7,[[8,4],9]]]");
    let snailfish_2 = Snailfish::new("[1,1]");
    let sum = snailfish_1 + snailfish_2;
    let result = sum.reduce();
    assert_eq!(&result.input, "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
}

#[test]
fn test_reduce_reddit_help_1() {
    let snailfish = Snailfish::new("[[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]");
    let result = snailfish.reduce();
    assert_eq!(&result.input, "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]");
}

#[test]
fn test_reduce_reddit_help_2() {
    let snailfish = Snailfish::new("[[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]],[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]]");
    let result = snailfish.reduce();
    assert_eq!(&result.input, "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]");
}

#[test]
fn test_reduce_reddit_help_3() {
    let snailfish = Snailfish::new("[[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]],[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]]");
    let result = snailfish.reduce();
    assert_eq!(&result.input, "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]");
}

#[test]
fn test_reduce_reddit_help_4() {
    let snailfish = Snailfish::new("[[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]],[7,[5,[[3,8],[1,4]]]]]");
    let result = snailfish.reduce();
    assert_eq!(&result.input, "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]");
}

#[test]
fn test_reduce_reddit_help_5() {
    let snailfish = Snailfish::new("[[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]],[[2,[2,2]],[8,[8,1]]]]");
    let result = snailfish.reduce();
    assert_eq!(result.input, "[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]");
}
