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

    let mut instructions = vec![];
    let mut values = vec![];
    let mut depth = 0;
    cursor.set_position(0);
    p2::parse(&mut cursor, &mut instructions, &mut values, &mut depth);
    println!("Part 2: {:?}", p2::unwind(&instructions));
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

    #[derive(Debug)]
    pub enum Instruction<'a> {
        Number(u64),
        Op(&'a str, Option<u64>)
    }

    pub fn unwind(instructions: &Vec<Instruction>) -> u64 {
        let mut values = vec![];
        let mut operations = vec![];

        for i in 0..instructions.len() {
            let inst = &instructions[i];

            match inst {
                Instruction::Op(op, default) => {
                    operations.push(*op);
                    match default {
                        Some(val) => values.push(*val),
                        None => ()
                    }
                },
                Instruction::Number(n) => {
                    let op = operations.pop().unwrap();
                    let val = values.pop().unwrap();

                    println!("{} {} {}", val, op, n);
                    match op {
                        "+" => {
                            values.push(val + n);
                            operations.push("+");
                        },
                        "*" => {
                            values.push(val * n);
                            operations.push("*");
                        },
                        "max" => {
                            if *n > val {
                                values.push(*n);
                            } else {
                                values.push(val);
                            }
                            operations.push("max");
                        },
                        "min" => {
                            if *n < val {
                                values.push(*n);
                            } else {
                                values.push(val);
                            }
                            operations.push("min");
                        }
                        _ => panic!("INVALID")
                    }
                }
            }
        }

        println!("{:?}", values);
        values[0]
    }

    #[test]
    fn test_unwind_sum() {
        let instructions = vec![
            Instruction::Op("+", Some(0)),
            Instruction::Number(25),
            Instruction::Number(10),
            Instruction::Number(11)
        ];

        assert_eq!(unwind(&instructions), 46);
    }

    #[test]
    fn test_unwind_multiply() {
        let instructions = vec![
            Instruction::Op("*", Some(1)),
            Instruction::Number(25),
            Instruction::Number(10),
            Instruction::Number(1)
        ];

        assert_eq!(unwind(&instructions), 250);
    }

    #[test]
    fn test_unwind_max() {
        let instructions = vec![
            Instruction::Op("max", Some(0)),
            Instruction::Number(25),
            Instruction::Number(10),
            Instruction::Number(1)
        ];

        assert_eq!(unwind(&instructions), 25);
    }

    #[test]
    fn test_unwind_min() {
        let instructions = vec![
            Instruction::Op("min", Some(u64::MAX)),
            Instruction::Number(25),
            Instruction::Number(10),
            Instruction::Number(1)
        ];

        assert_eq!(unwind(&instructions), 1);
    }

    #[test]
    fn test_unwind_recurse() {
        let instructions = vec![
            Instruction::Op("multiply", Some(1)),
            Instruction::Op("min", Some(u64::MAX)),
            Instruction::Number(25),
            Instruction::Number(10),
            Instruction::Number(1),
            Instruction::Op("min", Some(u64::MAX)),
            Instruction::Number(25),
            Instruction::Number(10),
            Instruction::Number(2)
        ];

        assert_eq!(unwind(&instructions), 2);
    }

    pub fn parse(
        cursor: &mut Cursor<String>,
        instructions: &mut Vec<Instruction>,
        values: &mut Vec<u64>,
        depth: &mut usize
    ) {
        let _version = read_ahead(cursor, 3);
        let type_id = read_ahead(cursor, 3);

        if type_id == 4 {
            let value = read_literal_value(cursor);

            instructions.push(Instruction::Number(value));
        } else {
            let type_length_id = read_ahead(cursor, 1);

            let (instruction, default) = match type_id {
                0 => ("+", Some(0)),
                1 => ("*", Some(1)),
                2 => ("min", None),
                3 => ("max", Some(0)),
                5 => (">", None),
                6 => ("<", None),
                7 => ("=", None),
                _ => panic!("invalid type_id")
            };

            instructions.push(Instruction::Op(instruction, default));

            if type_length_id == 0 {
                let total_length = read_ahead(cursor, 15);

                parse_with_read_limit(
                    cursor,
                    total_length,
                    instructions,
                    values,
                    depth
                );
            } else if type_length_id == 1 {
                let number_of_packs = read_ahead(cursor, 11);

                parse_with_packet_limit(
                    cursor,
                    number_of_packs,
                    instructions,
                    values,
                    depth
                );
            }
        }
    }

    fn parse_with_read_limit(
        cursor: &mut Cursor<String>,
        limit: u64,
        instructions: &mut Vec<Instruction>,
        values: &mut Vec<u64>,
        depth: &mut usize
    ) {
        let curr_poss = cursor.position();
        let limit = curr_poss + limit;

        while cursor.position() < limit {
            parse(cursor, instructions, values, depth);
        }
    }

    fn parse_with_packet_limit(
        cursor: &mut Cursor<String>,
        limit: u64,
        instructions: &mut Vec<Instruction>,
        values: &mut Vec<u64>,
        depth: &mut usize
    ) {
        let mut count = 0;

        while count < limit {
            parse(cursor, instructions, values, depth);
            count += 1;
        }
    }

    #[test]
    fn test_parse_complex_1() {
        let bytes = "C200B40A82".as_bytes();
        let mut cursor = bytes_to_bin(&bytes);
        let mut values: Vec<u64> = vec![];
        let mut instructions: Vec<Instruction> = vec![];
        parse(&mut cursor, &mut instructions, &mut values, &mut 0);

        assert_eq!(unwind(&instructions), 3);
    }

    #[test]
    fn test_parse_complex_2() {
        let bytes = "9C0141080250320F1802104A08".as_bytes();
        let mut cursor = bytes_to_bin(&bytes);
        let mut values: Vec<u64> = vec![];
        let mut instructions: Vec<Instruction> = vec![];
        parse(&mut cursor, &mut instructions, &mut values, &mut 0);

        assert_eq!(unwind(&instructions), 1);
    }

    //#[test]
    //fn test_parse_complex_3() {
    //    let bytes = "CE00C43D881120".as_bytes();
    //    let mut cursor = bytes_to_bin(&bytes);
    //    let mut values: Vec<u64> = vec![];
    //    let mut instructions: Vec<Instruction> = vec![];
    //    parse(&mut cursor, &mut instructions, &mut values, &mut 0);

    //    assert_eq!(unwind(&mut instructions, &mut vec![], &mut 0), 9);
    //}
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
