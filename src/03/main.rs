use std::env;
use std::fs;

fn main() {
    // Prints each argument on a separate line
    let arguments: Vec<String> = env::args().collect();
    let filename = arguments[1].clone();
    println!("Opening File: {}", filename);
    let contents = fs::read_to_string(filename);
    match contents {
        Ok(contents) => {
            let lines: Vec<&str> = contents.split("\n").filter(|line| line.len() > 0).collect();
            let lines_len = lines.len();
            let binary_len = lines[0].len();
            let mut sums = vec![0; binary_len];
            for line in lines.clone() {
                for (i, c) in line.chars().enumerate() {
                    if c == '1' {
                        sums[i] += 1;
                    }
                }
            }

            let mut gamma_string = String::with_capacity(sums.len());
            let mut epsilon_string = String::with_capacity(sums.len());
            for sum in sums.clone() {
                if sum >= lines_len / 2 {
                    gamma_string.push('1');
                    epsilon_string.push('0');
                } else {
                    gamma_string.push('0');
                    epsilon_string.push('1');
                }
            }
            let gamma = isize::from_str_radix(gamma_string.as_str(), 2).unwrap();
            let epsilon = isize::from_str_radix(epsilon_string.as_str(), 2).unwrap();
            println!("Gamma: {}", gamma);
            println!("Epsilon: {}", epsilon);
            println!("Part 1 Answer: {}", gamma * epsilon);

            let mut oxygen = lines.clone();
            let mut co2 = lines.clone();
            for i in 0..binary_len {
                if oxygen.len() > 1 {
                    let mut count = 0;
                    for line in oxygen.clone() {
                        if line.chars().nth(i).unwrap() == '1' {
                            count += 1;
                        } else {
                            count -= 1;
                        }
                    }
                    let letter;
                    if count >= 0 {
                        letter = '1';
                    } else {
                        letter = '0';
                    }
                    oxygen = oxygen.into_iter().filter(|line| line.chars().nth(i) == Some(letter)).collect();
                }

                if co2.len() > 1 {
                    let mut count = 0;
                    for line in co2.clone() {
                        if line.chars().nth(i).unwrap() == '1' {
                            count += 1;
                        } else {
                            count -= 1;
                        }
                    }
                    let letter;
                    if count >= 0 {
                        letter = '0';
                    } else {
                        letter = '1';
                    }
                    co2 = co2.into_iter().filter(|line| line.chars().nth(i) == Some(letter)).collect();
                }
            }
            let oxygen_value = isize::from_str_radix(oxygen[0], 2).unwrap();
            println!("Oxygen: {}", oxygen_value);
            let co2_value = isize::from_str_radix(co2[0], 2).unwrap();
            println!("CO2: {}", co2_value);
            println!("Part 2 Answer: {}", oxygen_value * co2_value); // 1079645 to low
        }
        Err(error) => {
            println!("Error: {}", error)
        }
    }
}
