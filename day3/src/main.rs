use std::error;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn binary_to_decimal(binary: &[u8]) -> u32 {
    let powers: Vec<u32> = vec![2048, 1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1];
    let mut sum = 0;
    for i in 0..binary.len() {
        sum += binary[i] as u32 * powers[i];
    }

    sum
}

fn get_counts(lines: &[Result<String, Error>]) -> (Vec<u32>, Vec<u32>) {
    let mut one_count: Vec<u32> = vec![0; 12];
    let mut zero_count: Vec<u32> = vec![0; 12];

    for line in lines {
        let line = line.as_ref().unwrap();

        let row_strs: Vec<&str> = line.trim().split("").collect();

        for i in 1..13 {
            let bit: u8 = row_strs[i].parse().unwrap();

            if bit == 1 {
                one_count[i - 1] += 1;
            } else {
                zero_count[i - 1] += 1;
            }
        }
    }

    (one_count, zero_count)
}

fn part_one(lines: &[Result<String, Error>]) -> (u32, u32) {
    let (one_count, zero_count) = get_counts(lines);

    let mut gamma_vec: Vec<u8> = Vec::new();
    let mut epsilon_vec: Vec<u8> = Vec::new();


    for i in 0..one_count.len() {
        if one_count[i] > zero_count[i] as u32 {
            gamma_vec.push(1);
            epsilon_vec.push(0);
        } else {
            gamma_vec.push(0);
            epsilon_vec.push(1);
        }
    }

    println!("Gamma binary: {:?}", gamma_vec);
    println!("Epsilon binary: {:?}", epsilon_vec);
    println!("One count: {:?}", one_count);

    let gamma_rate = binary_to_decimal(&gamma_vec);
    let epsilon_rate = binary_to_decimal(&epsilon_vec);

    (gamma_rate, epsilon_rate)
}

fn calculate_oxygen_gen_rating(lines: &[Result<String, Error>]) -> u32 {
    let (one_count, zero_count) = get_counts(lines);

    let mut lines_left: Vec<usize> = Vec::new();

    for i in 0..1000 {
        lines_left.push(i);
    }

    let mut bit_position: usize = 0;

    while lines_left.len() > 1 {
        let bit = match one_count >= zero_count {
            true => 1,
            false => 0,
        };

        for i in 0..lines.len() {
            let line = &lines[i];
            let chars: Vec<char> = line.as_ref().unwrap().chars().collect();
            let num = chars[bit_position].to_digit(10).unwrap();

            if num == bit {
                lines_left.retain(|&x| x != i);
            }
        }

        bit_position += 1;
    }

    let last_index = lines_left[0];
    let binary_str = lines[last_index].as_ref().unwrap();
    let binary_split: Vec<&str> = binary_str.split("").collect();
    let mut binary_vec: Vec<u8> = Vec::new();

    for i in 1..(binary_split.len() - 1) {
        let num: u8 = binary_split[i].parse().unwrap();

        binary_vec.push(num);
    }

    binary_to_decimal(&binary_vec)
}

fn calculate_co2_scrubber_rating(lines: &[Result<String, Error>]) -> u32 {
    let (one_count, zero_count) = get_counts(lines);

    let mut lines_left: Vec<usize> = Vec::new();

    for i in 0..1000 {
        lines_left.push(i);
    }

    let mut bit_position: usize = 0;

    while lines_left.len() > 1 {
        let bit = match one_count >= zero_count {
            true => 0,
            false => 1,
        };

        for i in 0..lines.len() {
            let line = &lines[i];
            let chars: Vec<char> = line.as_ref().unwrap().chars().collect();
            let num = chars[bit_position].to_digit(10).unwrap();

            if num == bit {
                lines_left.retain(|&x| x != i);
            }
        }

        bit_position += 1;
    }

    let last_index = lines_left[0];
    let binary_str = lines[last_index].as_ref().unwrap();
    let binary_split: Vec<&str> = binary_str.split("").collect();
    let mut binary_vec: Vec<u8> = Vec::new();

    for i in 1..(binary_split.len() - 1) {
        let num: u8 = binary_split[i].parse().unwrap();

        binary_vec.push(num);
    }

    binary_to_decimal(&binary_vec)
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let path = "input.txt";

    let input = File::open(path)?;
    let lines: Vec<Result<_, _>> = BufReader::new(input).lines().collect();

    let (gamma_rate, epsilon_rate) = part_one(&lines);

    let oxygen_gen_rating = calculate_oxygen_gen_rating(&lines);
    let co2_scrubber_rating = calculate_co2_scrubber_rating(&lines);

    println!("Part one solution: {}", gamma_rate * epsilon_rate);

    println!("Oxygen generator rating: {}", oxygen_gen_rating);
    println!("CO2 scrubber rating: {}", co2_scrubber_rating);

    println!("Part two solution: {}", oxygen_gen_rating * co2_scrubber_rating);

    Ok(())
}
