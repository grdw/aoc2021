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
    let total_ones = totals(binaries, '1');

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

fn totals(binaries: &Vec<&str>, search: char) -> Vec<usize> {
    let len = binaries[0].len();
    let mut total_ones = vec![0; len];

    for bin in binaries {
        for i in 0..len {
            let c = bin.chars().nth(i);

            if c == Some(search) {
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
    let oxygen_rating = binary_filter(&binaries, '1');
    let co2_scrubber_rating = binary_filter(&binaries, '0');

    oxygen_rating * co2_scrubber_rating
}

fn binary_filter(binaries: &Vec<&str>, search: char) -> u32 {
    let mut filter = binaries.clone();

    let reverse = if search >= '1' {
                      '0'
                  } else {
                      '1'
                  };

    for i in 0..binaries[0].len() {
        let threshold = ((filter.len() as f32) * 5.0) as usize;
        let total_ones = totals(&filter, '1');

        filter = filter
            .iter()
            .filter(|&&n| {
                let total = total_ones[i] * 10;
                let bit = if total >= threshold {
                              search
                          } else {
                              reverse
                          };

                n.chars().nth(i) == Some(bit)
            })
            .map(|n| *n)
            .collect();

        if filter.len() == 1 {
            break;
        }
    }

    u32::from_str_radix(filter[0], 2).unwrap()
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

