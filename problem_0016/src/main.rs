use std::fs;

fn main() {
    let display_string = fs::read_to_string("input")
                            .unwrap_or("".to_string());

    let bytes = display_string.as_bytes();
    let binary = bytes_to_bin(&bytes);
    parse(&binary);
}

fn bytes_to_bin(bytes: &[u8]) -> String {
    let mut start = String::from("");
    for b in bytes {
        if b < &11 { continue };
        let s = format!("{}", *b as char);
        let int = u64::from_str_radix(&s, 16).unwrap();
        let binary = format!("{:04b}", int);
        start.push_str(&binary);
    }
    start
}

#[test]
fn test_bytes_to_bin() {
    let bytes = "D2FE28".as_bytes();
    let bin = bytes_to_bin(&bytes);

    assert_eq!(bin, "110100101111111000101000");
}

fn parse(binary: &String) {
    let version = u64::from_str_radix(&binary[0..3], 2).unwrap();
    let type_id = u64::from_str_radix(&binary[3..6], 2).unwrap();
    let type_len_id = binary.chars().nth(7).unwrap();
    let is_literal = type_id == 4;

    if is_literal {
        let mut count = 0;
        let group_size = 5;
        let mut binar_rep = String::from("");

        loop {
            let start = 6 + (count * group_size);
            let end = start + group_size;
            let blob = &binary[start..end];

            binar_rep.push_str(&blob[1..group_size]);
            count += 1;

            if &blob[..1] == "0" {
                break;
            }
        }

        println!(
            "{} {}",
             binar_rep,
             u64::from_str_radix(&binar_rep, 2).unwrap()
        );
    } else if type_len_id == '0' {
        let total_l = u64::from_str_radix(&binary[7..22], 2).unwrap();
        println!("{}", total_l);
    } else if type_len_id == '1' {
        let num_of_sub = u64::from_str_radix(&binary[6..17], 2).unwrap();
        println!("{}", num_of_sub);
    }
    println!("");
}

#[test]
fn test_parse_bytes() {
    let bytes = "D2FE28".as_bytes();
    let bin = bytes_to_bin(&bytes);
    parse(&bin);

    assert_eq!(bin, "110100101111111000101000");
}
