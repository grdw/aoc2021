use std::fs;

fn main() {
	let fishes_string = fs::read_to_string("input")
                           .unwrap_or("".to_string());

    let fishes: Vec<i16> = fishes_string
        .split_terminator(",")
        .map(|n| n[0..1].parse::<i16>().unwrap())
        .collect();

    let count = immaculate_conception(fishes.clone(), 80);
    println!("After 80 days there will be: {:?}", count);
}

fn immaculate_conception(mut lantern_fish: Vec<i16>, days: i16) -> u128 {
    for _ in 0..days {
        for i in 0..lantern_fish.len() {
            if lantern_fish[i] == 0 {
                lantern_fish[i] = 6;
                lantern_fish.push(8);
            } else {
                lantern_fish[i] -= 1;
            }
        }
    }

    lantern_fish.len() as u128
}

#[test]
fn test_lantern_fish() {
    let lantern_fish = vec![3, 4, 3, 1, 2];
    let count = immaculate_conception(lantern_fish.clone(), 18);
    assert_eq!(count, 26);

    let count = immaculate_conception(lantern_fish.clone(), 80);
    assert_eq!(count, 5934);
}

