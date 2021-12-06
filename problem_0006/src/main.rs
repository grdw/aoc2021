use std::fs;

fn main() {
	let fishes_string = fs::read_to_string("input")
                           .unwrap_or("".to_string());

    let fishes: Vec<u16> = fishes_string
        .split_terminator(",")
        .map(|n| n[0..1].parse::<u16>().unwrap())
        .collect();

    let count = immaculate_conception(fishes.clone(), 80);
    println!("After 80 days there will be: {:?}", count);

    let count = fast_conception(fishes.clone(), 256);
    println!("After 80 days there will be: {:?}", count);
}

fn immaculate_conception(mut lantern_fish: Vec<u16>, days: u16) -> u128 {
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

    let lantern_fish = vec![3];
    let count = immaculate_conception(lantern_fish.clone(), 30);
    assert_eq!(count, 15);
}

fn fast_conception(data: Vec<u16>, days: u16) -> u128 {
    let fish = data.iter().fold([0u128; 9], |mut acc, val| {
        acc[*val as usize] += 1;
        acc
    });

    (0..days).fold(fish, |mut acc, _| {
        acc.rotate_left(1);
        acc[6] += acc[8];
        acc
    }).iter().sum()
}
#[test]
fn test_fast_conception() {
    let lantern_fishes = vec![3, 4, 3, 1, 2];

    let days = 18;
    assert_eq!(immaculate_conception(lantern_fishes.clone(), days), 26);
    assert_eq!(fast_conception(lantern_fishes.clone(), days), 26);

    let days = 80;
    assert_eq!(immaculate_conception(lantern_fishes.clone(), days), 5934);
    assert_eq!(fast_conception(lantern_fishes.clone(), days), 5934);
    assert_eq!(fast_conception(lantern_fishes.clone(), 256), 26984457539);
}
