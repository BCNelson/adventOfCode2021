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
            let input_numbers: Vec<i32> = lines.iter()
                .map(|x| x.parse::<i32>().unwrap()).collect();
            let increases = count_increases(&input_numbers);
            println!("Increases {}", increases);
            let rolling_sum_increases = count_increases(&rolling_sum(&input_numbers));
            println!("Rolling sum increases {}", rolling_sum_increases);
        }
        Err(error) => {
            println!("Error: {}", error)
        }
    }
}

fn rolling_sum(numbers: &Vec<i32>) -> Vec<i32> {
    let mut sums: Vec<i32> = Vec::with_capacity(numbers.len()-2);
    let mut current_sum = numbers[0] + numbers[1] + numbers[2];
    sums.push(current_sum);
    for i in 3..numbers.len(){
        current_sum = current_sum - numbers[i-3] + numbers[i];
        sums.push(current_sum);
    }
    sums
}

fn count_increases(input: &Vec<i32>) -> i32 {
    // println!("{:?}", input_numbers);
    let mut count = 0;
    let mut last = input[0];
    for i in 1..input.len() {
        if input[i] > last {
            count += 1;
        }
        last = input[i];
    }
    count
}
