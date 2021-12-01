use std::fs;

fn main() {
    let contents = fs::read_to_string("input")
                      .unwrap_or("".to_string());

    let depths: Vec<u16> = contents
        .split("\n")
        .map(|t| t.parse::<u16>().unwrap_or(0))
        .collect();

    println!("{}", depth_increases(&depths));
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
