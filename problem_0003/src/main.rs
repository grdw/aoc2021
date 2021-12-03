use std::fs;

fn main() {
    let contents = fs::read_to_string("input")
                      .unwrap_or("".to_string());

    let mut inputs: Vec<&str> = contents
        .split("\n")
        .collect();

    // Drop of the last newline, bad rust
    inputs.pop();

    println!("Diagnostic {:?}", binary_diagnostic(&inputs));
    println!("Life support rating {:?}", life_support_rating(&inputs));
}

fn binary_diagnostic(binaries: &Vec<&str>) -> u32 {
    let threshold = binaries.len() / 2;
    let total_ones = totals(binaries);

    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;

    for (i, t) in total_ones.iter().enumerate() {
        let n = 2_u32.pow((total_ones.len() - 1 - i) as u32);

        if *t > threshold {
            gamma_rate += n;
        } else {
            epsilon_rate += n;
        }
    }

    gamma_rate * epsilon_rate
}

fn totals(binaries: &Vec<&str>) -> Vec<usize> {
    let len = binaries[0].len();
    let mut total_ones = vec![0; len];

    for bin in binaries {
        for i in 0..len {
            let c = bin.chars().nth(i);

            if c == Some('1') {
                total_ones[i] += 1;
            }
        }
    }

    total_ones
}

#[test]
fn test_binary_diagnostic() {
	assert_eq!(
        binary_diagnostic(
            &vec![
                "00100",
                "11110",
                "10110",
                "10111",
                "10101",
                "01111",
                "00111",
                "11100",
                "10000",
                "11001",
                "00010",
                "01010"
            ]
        ),
        198
    );
}

fn life_support_rating(binaries: &Vec<&str>) -> u32 {
    let oxygen_rating = binary_filter(&binaries, &['0', '1']);
    let co2_scrubber_rating = binary_filter(&binaries, &['1', '0']);

    oxygen_rating * co2_scrubber_rating
}

fn binary_filter(binaries: &Vec<&str>, search: &[char]) -> u32 {
    let mut bins = binaries.clone();
    let mut j = 0;

    while bins.len() > 1 {
        let total_ones = bins
            .iter()
            .filter(|&&b| b.chars().nth(j) == Some('1'))
            .count();

        let threshold = (bins.len() as f32 * 5.0) as usize;

        bins.retain(|&bin| {
            let bit = if total_ones * 10 >= threshold {
                search[1]
            } else {
                search[0]
            };

            bin.chars().nth(j) == Some(bit)
        });

        j += 1
    }

    u32::from_str_radix(bins[0], 2).unwrap()
}

#[test]
fn test_life_support_rating() {
	assert_eq!(
        life_support_rating(
            &vec![
                "00100",
                "11110",
                "10110",
                "10111",
                "10101",
                "01111",
                "00111",
                "11100",
                "10000",
                "11001",
                "00010",
                "01010"
            ]
        ),
        230
    );
}

