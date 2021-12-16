use std::fs;

fn main() {
    let display_string = fs::read_to_string("input")
                            .unwrap_or("".to_string());
    let bytes: Vec<&str> = display_string.trim().split_terminator("").collect();

    println!("{:?}", bytes);
}

fn bytes_to_bin(bytes: &Vec<&str>) -> String {
    let mut start = String::from("");
    for b in bytes {
        let int = u64::from_str_radix(b, 16).unwrap();
        let binary = format!("{:04b}", int);
        start.push_str(&binary);
    }
    start
}

#[test]
fn test_bytes_to_bin() {
    let bytes = vec!["D", "2", "F", "E", "2", "8"];
    let bin = bytes_to_bin(&bytes);

    assert_eq!(bin, "110100101111111000101000");
}
