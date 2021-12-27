use std::fs;
use std::time::Instant;

mod binary_try;

fn first_task(content: &String) -> u32 {
    const BITS_NUMBER: usize = 12;

    let lines: Vec<&str> = content.lines().collect();
    let occurrences = lines.iter().fold(
        vec![0; BITS_NUMBER],
        |array: Vec<u32>, bin: &&str| -> Vec<u32> {
            let parsed_digits = bin.chars().map(|digit| digit as u32 - '0' as u32);
            let res: Vec<_> = parsed_digits
                .zip(array.iter())
                .map(|(x, y)| x + y)
                .collect();
            res
        },
    );
    let partial_results: String = occurrences
        .iter()
        .map(|x| if 2 * x < lines.len() as u32 { '0' } else { '1' })
        .collect();
    println!("{:?}", partial_results);
    let results = (isize::from_str_radix(&partial_results, 2).unwrap()) as u32;
    let results_xor = ((2 as u32).pow(12) - 1) ^ results;
    println!("{:?}", results);
    println!("{:?}", results_xor);
    results * results_xor
}

// This is a little bit over the top, but I wanted to train some folds at the time :^)
fn count_occurrences(numbers: &Vec<u32>, idx: u8) -> (u32, u32) {
    numbers.into_iter().fold(
        (0, 0),
        |(zeros, ones): (u32, u32), &number: &u32| -> (u32, u32) {
            match number >> idx & 1 {
                1 => (zeros, ones + 1),
                0 => (zeros + 1, ones),
                _ => unimplemented!(),
            }
        },
    )
}
fn filter_numbers(numbers: &Vec<u32>, idx: u8, chosen_digit: u32) -> Vec<u32> {
    numbers
        .iter()
        .filter(|&&number| (number >> idx & 1) == chosen_digit)
        .copied()
        .collect()
}
fn calculate_ogr(numbers: &Vec<u32>, idx: u8) -> Vec<u32> {
    let (zeros, ones) = count_occurrences(&numbers, idx);
    let chosen_digit = if zeros > ones { 0u32 } else { 1u32 };
    let filtered_numbers = filter_numbers(&numbers, idx, chosen_digit);
    if filtered_numbers.len() <= 1 {
        return filtered_numbers.to_owned();
    }
    calculate_ogr(&filtered_numbers, idx - 1)
}
fn calculate_csr(numbers: &Vec<u32>, idx: u8) -> Vec<u32> {
    let (zeros, ones) = count_occurrences(&numbers, idx);
    let chosen_digit = if zeros > ones { 1u32 } else { 0u32 };
    let filtered_numbers = filter_numbers(&numbers, idx, chosen_digit);
    if filtered_numbers.len() <= 1 {
        return filtered_numbers.to_owned();
    }
    calculate_csr(&filtered_numbers, idx - 1)
}
fn second_task(content: &String) -> u32 {
    let lines: Vec<&str> = content.lines().collect();
    let initial_idx: u8 = lines[0].len() as u8 - 1;
    let numbers: Vec<u32> = lines
        .iter()
        .map(|binary_string| (isize::from_str_radix(binary_string, 2).unwrap()) as u32)
        .collect();
    let ogr = calculate_ogr(&numbers, initial_idx)[0];
    let csr = calculate_csr(&numbers, initial_idx)[0];
    ogr * csr
}

fn main() {
    let filename = "input.txt";
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let first_result = first_task(&content);
    println!("{:?}", first_result);

    let before = Instant::now();
    let second_result = second_task(&content);
    println!("Elapsed time: {:.2?}", before.elapsed());
    println!("{:?}", second_result);

    let before = Instant::now();
    let string_second_result = binary_try::second_task(&content);
    println!("Elapsed time: {:.2?}", before.elapsed());
    println!("{:?}", string_second_result);
}
