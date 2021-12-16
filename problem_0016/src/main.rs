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
    parse(&mut cursor, &mut counter);
    println!("Part 1: {}", counter);
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

fn parse(cursor: &mut Cursor<String>, counter: &mut u64) {
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
