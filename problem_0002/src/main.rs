use std::fs;

fn main() {
    let contents = fs::read_to_string("input")
                      .unwrap_or("".to_string());

    let mut inputs: Vec<&str> = contents
        .split("\n")
        .collect();

    // Drop of the last newline, bad rust
    inputs.pop();

    println!("Ends at depth {:?}", move_submarine(&inputs))
}

fn move_submarine(coords: &Vec<&str>) -> u32 {
    let mut depth = 0;
    let mut position = 0;
    let mut aim = 0;

    for coord in coords {
        let d: Vec<&str> = coord.split(" ").collect();
        let x: u32 = d[1].parse().unwrap();

        match d[0] {
            "forward" => {
                position += x;
                depth += aim * x;
            },
            "down" => aim += x,
            "up" => aim -= x,
            _ => panic!("invalid direction")
        }
    }

    depth * position
}

#[test]
fn test_movement() {
    let final_pos = move_submarine(
        &vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2"
        ]
    );

    assert_eq!(final_pos, 900)
}
