use futures::future::join_all;
use itertools::Itertools;
use std::collections::HashSet;
use std::fs;
use std::num::Wrapping;
use std::time::Instant;
use std::{thread, time};
#[derive(Debug, Copy, Clone)]
struct ArrayElement(usize, usize, u32);

fn is_valid(x: usize, size: usize) -> bool {
    x < size
}
fn get_neighbour_indices(x: usize, y: usize) -> [(Wrapping<usize>, Wrapping<usize>); 4] {
    [
        (Wrapping(x) - Wrapping(1), Wrapping(y)),
        (Wrapping(x) + Wrapping(1), Wrapping(y)),
        (Wrapping(x), Wrapping(y) - Wrapping(1)),
        (Wrapping(x), Wrapping(y) + Wrapping(1)),
    ]
}
fn is_smaller_than_neighbours(x: usize, y: usize, array: &Vec<Vec<u32>>, value: u32) -> bool {
    let size_a = array.len();
    let size_b = array[0].len();
    let neighbour_indices = get_neighbour_indices(x, y);
    neighbour_indices
        .iter()
        .filter(|(a, b)| is_valid(a.0, size_a) && is_valid(b.0, size_b))
        .all(|(a, b)| array[a.0][b.0] > value)
}

async fn risk_value(x: usize, y: usize, array: &Vec<Vec<u32>>, value: u32) -> u32 {
    if is_smaller_than_neighbours(x, y, array, value) {
        value + 1
    } else {
        0
    }
}

async fn first_task(array: &Vec<Vec<u32>>) -> u32 {
    let result: Vec<_> = array
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(move |(j, value)| risk_value(i, j, array, *value))
        })
        .flatten()
        .collect::<Vec<_>>();
    join_all(result).await.iter().sum()
}

fn extend_basin(
    x: usize,
    y: usize,
    mut basin: HashSet<(usize, usize)>,
    array: &Vec<Vec<u32>>,
) -> HashSet<(usize, usize)> {
    let neighbour_indices = get_neighbour_indices(x, y);
    let size_a = array.len();
    let size_b = array[0].len();
    let filtered_neighbour_indices: Vec<_> = neighbour_indices
        .iter()
        .filter(|&(a, b)| {
            is_valid(a.0, size_a)
                && is_valid(b.0, size_b)
                && array[a.0][b.0] != 9
                && !(&basin).contains(&(a.0, b.0))
        })
        .collect();
    basin.insert((x, y));
    for &(a, b) in filtered_neighbour_indices {
        basin = extend_basin(a.0, b.0, basin, array);
    }
    basin
}

async fn find_basin(sink: ArrayElement, array: &Vec<Vec<u32>>) -> usize {

    let mut basin = HashSet::new();
    let (x, y) = (sink.0, sink.1);
    basin = extend_basin(x, y, basin, array);
    basin.len()
}

async fn second_task(array: &Vec<Vec<u32>>) -> usize {
    let sinks = array
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(move |(j, value)| ArrayElement(i, j, *value))
        })
        .flatten()
        .filter(|arr_el| is_smaller_than_neighbours(arr_el.0, arr_el.1, array, arr_el.2))
        .collect::<Vec<ArrayElement>>();

    let futures = sinks
        .into_iter()
        .map(move |sink| find_basin(sink, array))
        .collect::<Vec<_>>();
    let results = join_all(futures)
        .await
        .into_iter()
        .sorted_by(|a, b| b.cmp(a))
        .collect::<Vec<_>>();
    dbg!(results[0..3].iter()).cloned().product()
}

#[tokio::main]
async fn main() {
    const RADIX: u32 = 10;
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.lines().collect::<Vec<&str>>();
    let array: Vec<Vec<u32>> = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(RADIX).unwrap() as u32)
                .collect()
        })
        .collect();
    let first_result = first_task(&array).await;
    println!("{:#?}", first_result);
    let before = Instant::now();

    let second_result = second_task(&array).await;
    println!("Elapsed time: {:.2?}", before.elapsed());
    println!("{:#?}", second_result);
}
