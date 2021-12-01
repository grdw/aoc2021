use std::fs;

fn main() {
    let contents = fs::read_to_string("input")
                      .unwrap_or("".to_string());

    let depths: Vec<u16> = contents
        .split("\n")
        .map(|t| t.parse::<u16>().unwrap_or(0))
        .collect();

    println!("{}", depth_increases(&depths));
    println!("{}", sliding_depth_increases(&depths));
}

fn depth_increases(depths: &Vec<u16>) -> usize {
    (0..depths.len() - 1)
        .filter(|&i| depths[i + 1] > depths[i])
        .count()
}

#[test]
fn test_depth_increases() {
    assert_eq!(
        depth_increases(&vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
        7
    );
}

fn sliding_depth_increases(depths: &Vec<u16>) -> usize {
    let group_size = 3;

    (0..depths.len() - group_size)
        .filter(|&i| {
            let sum: u16 = depths[i..i + 3].iter().sum();
            let next_sum: u16 = depths[i + 1..i + 4].iter().sum();

            next_sum > sum
        }).count()
}

#[test]
fn test_sliding_depth_increases() {
    assert_eq!(
        sliding_depth_increases(&vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
        5
    );
}

