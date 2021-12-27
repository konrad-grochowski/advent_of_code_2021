use itertools::Itertools;
use std::fs;

fn first_task(tuples: &Vec<(&str, i32)>) -> i32 {
    let results = tuples.into_iter().fold(
        (0, 0),
        |(depth, horizontal_position): (i32, i32),
         &(direction, value): &(&str, i32)|
         -> (i32, i32) {
            match direction {
                "forward" => (depth, horizontal_position + value),
                "up" => (depth - value, horizontal_position),
                "down" => (depth + value, horizontal_position),
                _ => unimplemented!(),
            }
        },
    );

    results.0 * results.1
}

fn second_task(tuples: &Vec<(&str, i32)>) -> i32 {
    let results = tuples.into_iter().fold(
        (0, 0, 0),
        |(aim, depth, horizontal_position): (i32, i32, i32),
         &(direction, value): &(&str, i32)|
         -> (i32, i32, i32) {
            match direction {
                "forward" => (aim, depth + (aim * value), horizontal_position + value),
                "up" => (aim - value, depth, horizontal_position),
                "down" => (aim + value, depth, horizontal_position),
                _ => unimplemented!(),
            }
        },
    );

    results.1 * results.2
}

fn main() {
    let filename = "input.txt";
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let lines = content.lines();
    let tuples: Vec<(&str, i32)> = lines
        .map(|line| -> (&str, i32) {
            let partial: (&str, &str) = line.split(" ").collect_tuple().unwrap();
            (partial.0, partial.1.parse::<i32>().unwrap())
        })
        .collect();

    let first_result = first_task(&tuples);
    println!("{:?}", first_result);

    let second_result = second_task(&tuples);
    println!("{:?}", second_result);
}
