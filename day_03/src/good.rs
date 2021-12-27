use std::fs;

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

fn count_occurrences(binaries: &Vec<&str>, idx: usize) -> (u32, u32) {
    binaries.into_iter().fold(
        (0, 0),
        |(zeros, ones): (u32, u32), binary: &&str| -> (u32, u32) {
            match binary.chars().nth(idx).unwrap() {
                '1' => (zeros, ones + 1),
                '0' => (zeros + 1, ones),
                _ => unimplemented!(),
            }
        },
    )
}
fn filter_binaries(binaries: Vec<&str>, idx: usize, chosen_digit: char) -> Vec<&str> {
    binaries
        .into_iter()
        .filter(|binary| binary.chars().nth(idx).unwrap() == chosen_digit)
        .collect()
}

fn calculate_ogr(binaries: Vec<&str>, idx: usize) -> Vec<&str> {
    if binaries.len() <= 1 {
        return binaries;
    }
    let (zeros, ones) = count_occurrences(&binaries, idx);
    let chosen_digit = if zeros > ones { '0' } else { '1' };
    let filtered_binaries = filter_binaries(binaries, idx, chosen_digit);
    calculate_ogr(filtered_binaries, idx + 1)
}

fn calculate_csr(binaries: Vec<&str>, idx: usize) -> Vec<&str> {
    if binaries.len() <= 1 {
        return binaries;
    }
    let (zeros, ones) = count_occurrences(&binaries, idx);
    let chosen_digit = if zeros > ones { '1' } else { '0' };
    let filtered_binaries = filter_binaries(binaries, idx, chosen_digit);
    calculate_csr(filtered_binaries, idx + 1)
}
fn second_task(content: &String) -> u32 {
    const INITIAL_IDX: usize = 0;
    let binaries: Vec<&str> = content.lines().collect();
    let ogr = (isize::from_str_radix(calculate_ogr(binaries.to_owned(), INITIAL_IDX)[0], 2)
        .unwrap()) as u32;
    let csr = (isize::from_str_radix(calculate_csr(binaries.to_owned(), INITIAL_IDX)[0], 2)
        .unwrap()) as u32;
    println!("{:?},",ogr);
    println!("{:?},",csr);
    ogr * csr
}

fn main() {
    let filename = "input.txt";
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let first_result = first_task(&content);
    println!("{:?}", first_result);
    let second_result = second_task(&content);
    println!("{:?}", second_result);
}
