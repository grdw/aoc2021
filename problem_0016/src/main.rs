use std::fs;
use std::str;
use std::io::prelude::*;
use std::io::Cursor;

fn main() {
    let display_string = fs::read_to_string("input")
                            .unwrap_or("".to_string());

    let bytes = display_string.as_bytes();
    let mut cursor = bytes_to_bin(&bytes);
    let mut counter = 0;
    p1::parse(&mut cursor, &mut counter);
    println!("Part 1: {}", counter);

    cursor.set_position(0);
    let node = p2::Node::rc_root();
    p2::parse(&mut cursor, node.clone());
    let mut result = p2::collapse(node.clone(), p2::Node::rc_root());
    result = p2::collapse(result, p2::Node::rc_root());
    result = p2::collapse(result, p2::Node::rc_root());
    result = p2::collapse(result, p2::Node::rc_root());
    result = p2::collapse(result, p2::Node::rc_root());
    result = p2::collapse(result, p2::Node::rc_root());
    result = p2::collapse(result, p2::Node::rc_root());
    result = p2::collapse(result, p2::Node::rc_root());
    result = p2::collapse(result, p2::Node::rc_root());
    result = p2::collapse(result, p2::Node::rc_root());
    result = p2::collapse(result, p2::Node::rc_root());
    result = p2::collapse(result, p2::Node::rc_root());
    result = p2::collapse(result, p2::Node::rc_root());
    result = p2::collapse(result, p2::Node::rc_root());
    result = p2::collapse(result, p2::Node::rc_root());
    result = p2::collapse(result, p2::Node::rc_root());
    result = p2::collapse(result, p2::Node::rc_root());
    result = p2::collapse(result, p2::Node::rc_root());
    result = p2::collapse(result, p2::Node::rc_root());
    result = p2::collapse(result, p2::Node::rc_root());
    result = p2::collapse(result, p2::Node::rc_root());
    println!("Part 2: {:?}", result.borrow().read_value());
}

fn bytes_to_bin(bytes: &[u8]) -> Cursor<String> {
    let mut start = String::from("");
    for b in bytes {
        if b < &11 { continue };
        let s = format!("{}", *b as char);
        let int = u64::from_str_radix(&s, 16).unwrap();
        let binary = format!("{:04b}", int);
        start.push_str(&binary);
    }
    Cursor::new(start)
}

#[test]
fn test_bytes_to_bin() {
    let bytes = "D2FE28".as_bytes();
    let mut cursor = bytes_to_bin(&bytes);
    let mut bin = String::new();
    let _read_bytes = cursor.read_to_string(&mut bin);

    assert_eq!(bin, "110100101111111000101000");
}

fn read_ahead(cursor: &mut Cursor<String>, bytes: usize) -> u64 {
    let mut buf = vec![0; bytes];
    let _c = cursor.read_exact(&mut buf);
    let version = str::from_utf8(&buf).unwrap();
    u64::from_str_radix(&version, 2).unwrap()
}

fn read_literal_value(cursor: &mut Cursor<String>) -> u64 {
    let mut total = String::new();
    loop {
        let mut buf = vec![0; 5];
        let _c = cursor.read_exact(&mut buf);
        let version = str::from_utf8(&buf).unwrap();

        total.push_str(&version[1..5]);

        if &version[..1] == "0" {
            break;
        }
    }
    u64::from_str_radix(&total, 2).unwrap()
}

mod p2 {
    use super::*;
    use std::rc::{Rc, Weak};
    use std::cell::RefCell;

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub enum Instruction {
        No,
        Number(u64),
        Op(u64)
    }

    #[derive(Debug)]
    pub struct Node {
        parent: Option<Weak<RefCell<Node>>>,
        children: Vec<Rc<RefCell<Node>>>,
        instruction: Instruction
    }

    impl Node {
        pub fn rc_root() -> Rc<RefCell<Node>> {
            Rc::new(
                RefCell::new(
                    Node::node(Instruction::No, None)
                )
            )
        }

        pub fn add_child(
            &mut self,
            instruction: Instruction,
            parent: &Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {

            let rc = Rc::new(
                RefCell::new(
                    Node::node(
                        instruction,
                        Some(Rc::downgrade(parent))
                    )
                )
            );

            self.children.push(rc.clone());
            rc
        }

        fn to_vec(&self) -> Vec<u64> {
            let mut result = vec![];
            for child in &self.children {
                match child.borrow().instruction {
                    Instruction::Number(n) => result.push(n),
                    _ => ()
                }
            }
            result
        }

        fn node(
            instruction: Instruction,
            parent: Option<Weak<RefCell<Node>>>
        ) -> Node {
            Node {
                children: vec![],
                parent: parent,
                instruction: instruction
            }
        }

        fn all_leafs(&self) -> bool {
            self.children.iter().all(|n| n.borrow().is_leaf())
        }

        fn is_leaf(&self) -> bool {
            self.children.is_empty()
        }

        pub fn read_value(&self) -> Option<u64> {
            match &self.children[0].as_ref().borrow().instruction {
                Instruction::Number(n) => Some(*n),
                _ => None
            }
        }
    }

    pub fn collapse(
        rc_node: Rc<RefCell<Node>>,
        result: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        let node = rc_node.borrow();

        if node.all_leafs() {
            let nums = node.to_vec();
            let val = match node.instruction {
                Instruction::Op(0) => nums.iter().fold(0, |a, n| a + n),
                Instruction::Op(1) => nums.iter().fold(1, |a, n| a * n),
                Instruction::Op(2) => *nums.iter().min().unwrap(),
                Instruction::Op(3) => *nums.iter().max().unwrap(),
                Instruction::Op(5) => if nums[0] == nums[1] { 1 } else { 0 },
                Instruction::Op(6) => if nums[0] > nums[1] { 1 } else { 0 },
                Instruction::Op(7) => if nums[0] < nums[1] { 1 } else { 0 },
                Instruction::Number(val) => val,
                _ => panic!("Invalid")
            };

            result.borrow_mut().add_child(
                Instruction::Number(val), &result
            );
            result
        } else {
            let n = match node.instruction {
                Instruction::Op(n) => {
                    result.borrow_mut().add_child(Instruction::Op(n), &result)
                },
                _ => result.clone()
            };

            for i in 0..node.children.len() {
                collapse(node.children[i].clone(), n.clone());
            }
            result
        }
    }

    fn recurse_collapse(rc_node: Rc<RefCell<Node>>) -> u64 {
        if let Some(n) = rc_node.borrow().read_value() {
            n
        } else {
            recurse_collapse(
                collapse(rc_node.clone(), Node::rc_root())
            )
        }
    }

    #[test]
    fn test_unwind_sum() {
        let root = Node::rc_root();
        let add_root = root.borrow_mut().add_child(Instruction::Op(0), &root);
        add_root.borrow_mut().add_child(Instruction::Number(25), &add_root);
        add_root.borrow_mut().add_child(Instruction::Number(10), &add_root);
        add_root.borrow_mut().add_child(Instruction::Number(1), &add_root);

        let new_node = Node::rc_root();
        collapse(root, new_node.clone());
        let number = new_node.borrow().read_value();

        assert_eq!(number, Some(36));
    }

    #[test]
    fn test_unwind_multiply() {
        let root = Node::rc_root();
        let mul1 = root.borrow_mut().add_child(Instruction::Op(1), &root);
        let add1 = mul1.borrow_mut().add_child(Instruction::Op(0), &mul1);
        let add2 = mul1.borrow_mut().add_child(Instruction::Op(0), &mul1);
        add1.borrow_mut().add_child(Instruction::Number(5), &add1);
        add1.borrow_mut().add_child(Instruction::Number(2), &add1);
        add2.borrow_mut().add_child(Instruction::Number(6), &add2);
        add2.borrow_mut().add_child(Instruction::Number(4), &add2);

        assert_eq!(recurse_collapse(root), 70);
    }

    #[test]
    fn test_unwind_max() {
        let root = Node::rc_root();
        let max = root.borrow_mut().add_child(Instruction::Op(3), &root);
        max.borrow_mut().add_child(Instruction::Number(25), &max);
        max.borrow_mut().add_child(Instruction::Number(10), &max);
        max.borrow_mut().add_child(Instruction::Number(1), &max);

        assert_eq!(recurse_collapse(root), 25);
    }

    #[test]
    fn test_unwind_min() {
        let root = Node::rc_root();
        let max = root.borrow_mut().add_child(Instruction::Op(2), &root);
        max.borrow_mut().add_child(Instruction::Number(25), &max);
        max.borrow_mut().add_child(Instruction::Number(10), &max);
        max.borrow_mut().add_child(Instruction::Number(1), &max);

        assert_eq!(recurse_collapse(root), 1);
    }

    #[test]
    fn test_unwind_gt() {
        let root = Node::rc_root();
        let max = root.borrow_mut().add_child(Instruction::Op(5), &root);
        max.borrow_mut().add_child(Instruction::Number(25), &max);
        max.borrow_mut().add_child(Instruction::Number(10), &max);

        assert_eq!(recurse_collapse(root), 0);
    }

    #[test]
    fn test_unwind_lt() {
        let root = Node::rc_root();
        let max = root.borrow_mut().add_child(Instruction::Op(6), &root);
        max.borrow_mut().add_child(Instruction::Number(25), &max);
        max.borrow_mut().add_child(Instruction::Number(10), &max);

        assert_eq!(recurse_collapse(root), 1);
    }

    #[test]
    fn test_unwind_eq() {
        let root = Node::rc_root();
        let max = root.borrow_mut().add_child(Instruction::Op(7), &root);
        max.borrow_mut().add_child(Instruction::Number(25), &max);
        max.borrow_mut().add_child(Instruction::Number(10), &max);

        assert_eq!(recurse_collapse(root), 0);
    }

    pub fn parse(cursor: &mut Cursor<String>, node: Rc<RefCell<Node>>) {
        let _version = read_ahead(cursor, 3);
        let type_id = read_ahead(cursor, 3);

        if type_id == 4 {
            let value = read_literal_value(cursor);
            let number = Instruction::Number(value);
            node.borrow_mut().add_child(number, &node);
        } else {
            let type_length_id = read_ahead(cursor, 1);

            let operation = Instruction::Op(type_id);
            let child = node.borrow_mut().add_child(operation, &node);

            if type_length_id == 0 {
                let total_length = read_ahead(cursor, 15);

                parse_with_read_limit(cursor, total_length, child);
            } else if type_length_id == 1 {
                let number_of_packs = read_ahead(cursor, 11);

                parse_with_packet_limit(cursor, number_of_packs, child);
            }
        }
    }

    fn parse_with_read_limit(
        cursor: &mut Cursor<String>,
        limit: u64,
        node: Rc<RefCell<Node>>
    ) {
        let curr_poss = cursor.position();
        let limit = curr_poss + limit;

        while cursor.position() < limit {
            parse(cursor, node.clone());
        }
    }

    fn parse_with_packet_limit(
        cursor: &mut Cursor<String>,
        limit: u64,
        node: Rc<RefCell<Node>>
    ) {
        let mut count = 0;

        while count < limit {
            parse(cursor, node.clone());
            count += 1;
        }
    }

    #[test]
    fn test_parse_complex_1() {
        let bytes = "C200B40A82".as_bytes();
        let mut cursor = bytes_to_bin(&bytes);
        let node = Node::rc_root();
        parse(&mut cursor, node.clone());

        let number = recurse_collapse(node);
        assert_eq!(number, 3);
    }

    #[test]
    fn test_parse_complex_2() {
        let bytes = "9C0141080250320F1802104A08".as_bytes();
        let mut cursor = bytes_to_bin(&bytes);
        let node = Node::rc_root();
        parse(&mut cursor, node.clone());

        let number = recurse_collapse(node);
        assert_eq!(number, 1);
    }

    #[test]
    fn test_parse_complex_3() {
        let bytes = "CE00C43D881120".as_bytes();
        let mut cursor = bytes_to_bin(&bytes);
        let node = Node::rc_root();
        parse(&mut cursor, node.clone());

        let number = recurse_collapse(node);
        assert_eq!(number, 9);
    }
}


mod p1 {
    use super::*;

    pub fn parse(cursor: &mut Cursor<String>, counter: &mut u64) {
        let version = read_ahead(cursor, 3);
        let type_id = read_ahead(cursor, 3);

        *counter += version;

        if type_id == 4 {
            let _value = read_literal_value(cursor);
        } else {
            let type_length_id = read_ahead(cursor, 1);

            if type_length_id == 0 {
                let total_length = read_ahead(cursor, 15);
                parse_with_read_limit(cursor, total_length, counter);
            } else if type_length_id == 1 {
                let number_of_packs = read_ahead(cursor, 11);
                parse_with_packet_limit(cursor, number_of_packs, counter);
            }
        }
    }

    fn parse_with_read_limit(
        cursor: &mut Cursor<String>,
        limit: u64,
        counter: &mut u64
    ) {
        let curr_poss = cursor.position();
        let limit = curr_poss + limit;

        while cursor.position() < limit {
            parse(cursor, counter);
        }
    }

    fn parse_with_packet_limit(
        cursor: &mut Cursor<String>,
        limit: u64,
        counter: &mut u64
    ) {
        let mut count = 0;

        while count < limit {
            parse(cursor, counter);
            count += 1;
        }
    }

    #[test]
    fn test_parse_bytes_literal_value() {
        let bytes = "D2FE28".as_bytes();
        let mut cursor = bytes_to_bin(&bytes);
        let mut counter = 0;
        parse(&mut cursor, &mut counter);

        cursor.set_position(0);
        let mut full_read = String::new();
        let _bytes_read = cursor.read_to_string(&mut full_read);

        assert_eq!(full_read, "110100101111111000101000");
    }


    #[test]
    fn test_parse_bytes_type_0() {
        let bytes = "38006F45291200".as_bytes();
        let mut cursor = bytes_to_bin(&bytes);
        let mut counter = 0;
        parse(&mut cursor, &mut counter);
    }

    #[test]
    fn test_parse_bytes_type_1() {
        let bytes = "EE00D40C823060".as_bytes();
        let mut cursor = bytes_to_bin(&bytes);
        let mut counter = 0;
        parse(&mut cursor, &mut counter);
    }

    #[test]
    fn test_parse_complex_1() {
        let bytes = "8A004A801A8002F478".as_bytes();
        let mut cursor = bytes_to_bin(&bytes);
        let mut counter = 0;
        parse(&mut cursor, &mut counter);
        assert_eq!(counter, 16);
    }

    #[test]
    fn test_parse_complex_2() {
        let bytes = "620080001611562C8802118E34".as_bytes();
        let mut cursor = bytes_to_bin(&bytes);
        let mut counter = 0;
        parse(&mut cursor, &mut counter);
        assert_eq!(counter, 12);
    }

    #[test]
    fn test_parse_complex_3() {
        let bytes = "C0015000016115A2E0802F182340".as_bytes();
        let mut cursor = bytes_to_bin(&bytes);
        let mut counter = 0;
        parse(&mut cursor, &mut counter);
        assert_eq!(counter, 23);
    }

    #[test]
    fn test_parse_complex_4() {
        let bytes = "A0016C880162017C3686B18A3D4780".as_bytes();
        let mut cursor = bytes_to_bin(&bytes);
        let mut counter = 0;
        parse(&mut cursor, &mut counter);
        assert_eq!(counter, 31);
    }
}
