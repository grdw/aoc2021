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
}

fn t(d_left: u16, reducer: u16, count: &mut u128) {
    if reducer == 9 {
        *count += 1
    }

    if d_left > 9 {
        t(d_left - 9, 9, count);
        t(d_left - 7, 7, count);
    }
}

fn fast_conception(l: Vec<u16>, days: u16) -> u128 {
    let d: Vec<u16> = l.iter().map(|n| days - n).collect();

    let mut c = 1;
    t(d[0], l[0], &mut c);
    println!("{}", c);
    0
}

#[test]
fn test_fast_conception() {
    let lantern_fishes = vec![3, 4, 3, 1, 2];

    //let days = 18;
    //assert_eq!(immaculate_conception(lantern_fishes.clone(), days), 26);
    //assert_eq!(fast_conception(lantern_fishes.clone(), days), 26);

    //let days = 80;
    //assert_eq!(immaculate_conception(lantern_fishes.clone(), days), 5934);
    //assert_eq!(fast_conception(lantern_fishes.clone(), days), 5934);
    assert_eq!(fast_conception(lantern_fishes.clone(), 256), 26984457539);
}
