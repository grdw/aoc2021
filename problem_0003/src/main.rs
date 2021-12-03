use std::fs;

fn main() {
    let contents = fs::read_to_string("input")
                      .unwrap_or("".to_string());

    let mut inputs: Vec<&str> = contents
        .split("\n")
        .collect();

    // Drop of the last newline, bad rust
    inputs.pop();

    println!("Diagnostic {:?}", binary_diagnostic(&inputs))
}

fn binary_diagnostic(binaries: &Vec<&str>) -> u32 {
    let len = binaries[0].len();
    let threshold = binaries.len() / 2;

    let mut total_ones = vec![0; len];
    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;

    for bin in binaries {
        for i in 0..len {
            let c = bin.chars().nth(i).unwrap();

            if c == '1' {
                total_ones[i] += 1;
            }
        }
    }

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
