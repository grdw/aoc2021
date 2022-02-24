use std::fs;

fn main() {
    println!("Hello, world!");
}

fn reboot(input: &'static str) -> u64 {
    for line in input.split_terminator("\n") {
        let p: Vec<&'static str> = line.split(" ").collect();

        match p[0] {
            "on" => {
                println!("{:?}", p[1]);
            },
            "off" => {
                println!("{:?}", p[1]);
            },
            _ => panic!("LOGIC IS GONE")
        }
    }

    0
}

#[test]
fn test_example() {
    let input = "on x=10..12,y=10..12,z=10..12\n\
                 on x=11..13,y=11..13,z=11..13\n\
                 off x=9..11,y=9..11,z=9..11\n\
                 on x=10..10,y=10..10,z=10..10";

    assert_eq!(reboot(&input), 39)
}
