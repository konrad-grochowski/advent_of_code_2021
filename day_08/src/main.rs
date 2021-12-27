use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn first_task(numbers: &[i64]) {}

fn second_task(numbers: &[i64]) -> i64 {
    let mean: i64 = numbers.iter().sum::<i64>() / numbers.len() as i64;
    println!("{}", mean);
    numbers
        .iter()
        .map(|n| {
            let dist = (n - mean).abs();
            (dist + 1) * dist / 2
        })
        .sum()
}

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.lines().collect::<Vec<&str>>();
    let digits: Vec<HashSet<char>> = lines[0].split("|").collect::<Vec<&str>>()[0]
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.chars().collect::<HashSet<char>>())
        .collect();


    let mut digit_map: HashMap<u8, HashSet<char>> = HashMap::new();
    let easy_recognition_digits: [(u8, usize); 4] = [(1, 2), (7, 3), (4, 4), (8, 7)];
    easy_recognition_digits
        .iter()
        .for_each(|(digit, len): &(u8, usize)| {
            let idx = digits
                .iter()
                .position(|r| r.len() == *len as usize)
                .unwrap();
            digit_map.insert(*digit, digits[idx].to_owned());
        });

    let sets_with_five_chars: Vec<HashSet<char>> = digits
        .iter()
        .cloned()
        .filter(|set| set.len() == 5 as usize)
        .collect();
    let three_hashset = sets_with_five_chars
        .iter()
        .find(|set| digit_map.get(&7).unwrap().is_subset(set))
        .unwrap();
    digit_map.insert(3,three_hashset.to_owned());
    let two_hashset = sets_with_five_chars
        .iter()
        .find(|set| {
            digit_map
                .get(&4)
                .unwrap()
                .intersection(set)
                .cloned()
                .collect::<HashSet<char>>()
                .len()
                == 2 as usize
        })
        .unwrap();
    digit_map.insert(2,two_hashset.to_owned());
    let five_hashset = sets_with_five_chars
        .iter()
        .find(|set| {
            digit_map
                .get(&4)
                .unwrap()
                .intersection(set)
                .cloned()
                .collect::<HashSet<char>>()
                .len()
                == 3 as usize
        })
        .unwrap();
    digit_map.insert(5,five_hashset.to_owned());
    let sets_with_six_chars: Vec<HashSet<char>> = digits
        .iter()
        .cloned()
        .filter(|set| set.len() == 6 as usize)
        .collect();
    let nine_hashset = sets_with_six_chars
        .iter()
        .find(|set| digit_map.get(&4).unwrap().is_subset(set))
        .unwrap();
    digit_map.insert(9,nine_hashset.to_owned());
    let six_hashset = sets_with_six_chars
        .iter()
        .find(|set| digit_map.get(&5).unwrap().is_subset(set))
        .unwrap();
    digit_map.insert(6,six_hashset.to_owned());
    let zero_hashset = sets_with_six_chars
        .iter()
        .find(|&set| set != six_hashset && set != nine_hashset)
        .unwrap();
    digit_map.insert(0,zero_hashset.to_owned());
}
