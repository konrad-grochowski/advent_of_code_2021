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

    let joined_digits = dbg!(digits
        .iter()
        .map(|set| set.iter().copied().collect::<Vec<char>>().iter().join(""))
        .join(""));
    let chars: [char; 7] = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];

    let chars_counts = chars
        .iter()
        .map(|ch| {
            let count = joined_digits
                .chars()
                .filter(|character| ch == character)
                .count();
            (count, ch)
        })
        .collect::<HashMap<usize, &char>>();

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

    let char_that_two_is_missing = chars_counts
        .iter()
        .filter(|(&k, _)| k == 9 as usize)
        .next()
        .unwrap()
        .1;
    let two_hashset = digits
        .iter()
        .find(|set| !set.contains(char_that_two_is_missing))
        .unwrap();

    digit_map.insert(2, two_hashset.to_owned());
    let sets_with_five_chars_besides_two: Vec<HashSet<char>> = digits
        .iter()
        .cloned()
        .filter(|set| set.len() == 5 as usize && set != two_hashset)
        .collect();
    let three_and_five_apart: Vec<(bool, Vec<HashSet<char>>)> = sets_with_five_chars_besides_two
        .iter()
        .group_by(|set| digit_map.get(&1).unwrap().is_subset(set))
        .into_iter()
        .map(|(ge0, group)| (ge0, group.cloned().collect()))
        .collect();
    three_and_five_apart.iter().for_each(|(key,group)| {
        if *key == true {
            digit_map.insert(3,group.iter().next().unwrap().to_owned());
        } else {
            digit_map.insert(5,group.iter().next().unwrap().to_owned());
        }
    });
    let sets_with_six_chars:Vec<HashSet<char>> = digits
        .iter()
        .cloned()
        .filter(|set| set.len() == 6 as usize)
        .collect();
    let nine_hashset = sets_with_six_chars.iter().find(|set| digit_map.get(&4).unwrap().is_subset(set)).unwrap();
    let six_hashset = sets_with_six_chars.iter().find(|set| digit_map.get(&5).unwrap().is_subset(set)).unwrap();
        
        
        
        
}