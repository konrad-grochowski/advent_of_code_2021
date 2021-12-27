use itertools::Itertools;
use std::fs;

fn first_task(content: &String) -> u16{
    let mut values = content
        .split_whitespace()
        .map(|v| v.parse::<u16>()
        .unwrap());
    let first_value = values.next().unwrap();
    let increases: u16 = values.fold(
        (0,first_value), 
        |mut acc, value| {
            if value > acc.1{
                acc.0 += 1;
            }
            acc.1 = value;
            acc
        }
    ).0;
    increases
}

fn second_task(content: String) -> u16 {
    let values = content
        .split_whitespace()
        .map(|v| v.parse::<u16>()
        .unwrap());
    let mut windowed_values = values
        .into_iter()
        .tuple_windows::<(_, _, _)>()
        .map(|(a,b,c)| a+b+c );
    let first_value = windowed_values.next().unwrap();
    let increases: u16 = windowed_values.fold(
        (0,first_value), 
        |mut acc, value| {
            if value > acc.1{
                acc.0 += 1;
            }
            acc.1 = value;
            acc
        }
    ).0;
    increases
}

fn main() {
    let filename = "input.txt";
    let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");


    let first_result = first_task(&content);
    println!("{:?}",first_result);
    let second_result = second_task(content);
    println!("{:?}",second_result);
    
}
