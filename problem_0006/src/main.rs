use std::fs;

fn main() {
	let fishes_string = fs::read_to_string("input")
                           .unwrap_or("".to_string());

    let mut fishes: Vec<i16> = fishes_string
        .split_terminator(",")
        .map(|n| n[0..1].parse::<i16>().unwrap())
        .collect();

    let count = immaculate_conception(fishes.clone(), 80);
    println!("After 80 days there will be: {:?}", count);

    let count = immaculate_conception(fishes.clone(), 256);
    println!("After 256 days there will be: {:?}", count);
}

fn immaculate_conception(mut lantern_fish: Vec<i16>, days: i16) -> u128 {
    for i in 0..lantern_fish.len() {
        lantern_fish[i] = days - lantern_fish[i]
    }

    for l in lantern_fish {
        println!("{}", l);
    }

    0
}

#[test]
fn test_lantern_fish() {
    let mut lantern_fish = vec![3, 4, 3, 1, 2];
    let count = immaculate_conception(lantern_fish.clone(), 18);
    assert_eq!(count, 26);

    let count = immaculate_conception(lantern_fish.clone(), 80);
    assert_eq!(count, 5934);

    let count = immaculate_conception(lantern_fish.clone(), 256);
    assert_eq!(count, 26984457539);
}
