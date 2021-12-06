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
    //let mut fish_count = lantern_fish.len() as u128;

    for d in 0..days {

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

fn fish_count(start: u16, days: u16) -> u128 {
    //let nine = 2.0_f64.powf((days / 9) as f64) as u128;
    //println!("{}", nine);
    // Assuming they all go each 7th day
    2.0_f64.powf((days / 7) as f64) as u128
}

fn actual_fish_count(i: u16, days: u16) -> u128 {
    let mut fishes = vec![i];
    for d in 0..days {
        for i in 0..fishes.len() {
            if fishes[i] == 0 {
                fishes[i] = 6;
                fishes.push(8);
            } else {
                fishes[i] -= 1;
            }
        }
    }

    fishes.len() as u128
}

#[test]
fn test_fish_count() {
    //assert_eq!(actual_fish_count(6, 28), 9);
    //assert_eq!(fish_count(6, 28), 9);
    //assert_eq!(fish_count(1, 80), 1401);
    assert_eq!(actual_fish_count(6, 80), 905);
    assert_eq!(fish_count(6, 80), 905);
    //assert_eq!(fish_count(6, 18), 4);
    //assert_eq!(fish_count(0, 80), 1421);
    //assert_eq!(fish_count(1, 80), 1401);
}
