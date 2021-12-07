use std::fs;

fn main() {
	let crabs_string = fs::read_to_string("input")
                           .unwrap_or("".to_string());

    let crabs: Vec<isize> = crabs_string
        .split_terminator(",")
        .map(|n| str::replace(n, "\n", "").parse::<isize>().unwrap())
        .collect();

    let count = cheapest_crab_move(&crabs);
    println!("The cheapest move is: {:?}", count);

    let count = cheapest_crab_move_with_tax(&crabs);
    println!("The cheapest move (with tax) is: {:?}", count);
}

fn cheapest_crab_move(crabs: &Vec<isize>) -> isize {
    let max = crabs.iter().max().unwrap();

    (0..*max)
        .map(|m|
            crabs
                .iter()
                .map(|n| (n - m).abs())
                .fold(0, |acc, b| acc + b))
        .min().unwrap()
}


#[test]
fn test_cheapest_move() {
    let moves = vec![16,1,2,0,4,2,7,1,2,14];

    assert_eq!(cheapest_crab_move(&moves), 37);
}

fn cheapest_crab_move_with_tax(crabs: &Vec<isize>) -> isize {
    let mut tax_rates = vec![0, 1];
    let max = crabs.iter().max().unwrap();

    for t in 2..=*max {
        tax_rates.push((1..=t).sum::<isize>());
    }

    (0..*max)
        .map(|m|
            crabs
                .iter()
                .map(|n| tax_rates[(n - m).abs() as usize])
                .fold(0, |acc, b| acc + b))
        .min().unwrap()
}

#[test]
fn test_cheapest_move_with_tax() {
    let moves = vec![16,1,2,0,4,2,7,1,2,14];

    assert_eq!(cheapest_crab_move_with_tax(&moves), 168);
}
