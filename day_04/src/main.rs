use regex::Regex;
use std::collections::HashMap;
use std::fs;




fn first_task<const N: usize>(boards: &[Vec<Vec<u16>>], crossing_numbers: &[u16]) -> u16{
    let mut num_to_pos: HashMap<u16, Vec<(usize, usize, usize)>> = HashMap::new();
    for (i, board) in boards.iter().enumerate() {
        for (j, row) in board.iter().enumerate() {
            for (k, &value) in row.iter().enumerate() {
                num_to_pos.entry(value).or_insert_with(|| vec![]).push((i,j,k))
            }
        }
    }
    let mut board_counters:  Vec<([usize;N],[usize;N])> = vec![([N;N],[N;N]);boards.len()];
    for (e,crossing_number) in crossing_numbers.iter().enumerate() {
        match num_to_pos.get(crossing_number) {
            Some(coords) => {
                for (i,j,k) in coords{

                    board_counters[*i].0[*j]-=1;
                    board_counters[*i].1[*k]-=1;
                    if board_counters[*i].0[*j] == 0
                    || board_counters[*i].1[*k] == 0{
                        return boards[*i]
                            .iter()
                            .flatten()
                            .filter(|elem| !crossing_numbers[0..=e].contains(elem))
                            .sum::<u16>()
                            *
                            *crossing_number as u16
                             
                    }
                }
            },
            None => continue,
        }
    }
    0
}

fn second_task<const N: usize>(boards: &[Vec<Vec<u16>>], crossing_numbers: &[u16]) -> u16{
    let mut num_to_pos: HashMap<u16, Vec<(usize, usize, usize)>> = HashMap::new();
    for (i, board) in boards.iter().enumerate() {
        for (j, row) in board.iter().enumerate() {
            for (k, &value) in row.iter().enumerate() {
                num_to_pos.entry(value).or_insert_with(|| vec![]).push((i,j,k))
            }
        }
    }
    let mut board_counters:  Vec<([usize;N],[usize;N])> = vec![([N;N],[N;N]);boards.len()];
    let mut boards_left: Vec<usize> = (0..boards.len()).collect::<Vec<_>>();
    for (e,crossing_number) in crossing_numbers.iter().enumerate() {
        match num_to_pos.get(crossing_number) {
            Some(coords) => {
                for (i,j,k) in coords{
                    if !boards_left.contains(i){
                        continue
                    }
                    board_counters[*i].0[*j]-=1;
                    board_counters[*i].1[*k]-=1;
                    if board_counters[*i].0[*j] == 0
                    || board_counters[*i].1[*k] == 0{
                        if boards_left.len() == 1{
                            
                            let last_idx = boards_left[0];
                            println!("{:?}",boards[last_idx]);
                            println!("{:?}",(e,i,j,k,crossing_number,board_counters[*i].0[*j]));
                            return boards[last_idx]
                            .iter()
                            .flatten()
                            .filter(|elem| !crossing_numbers[0..=e].contains(elem))
                            .sum::<u16>()
                            *
                            *crossing_number as u16
                            
                        }
                        boards_left.retain(|&x| x != *i);
                        

                    }
                }
            },
            None => continue,
        }
    }
    0
}


fn main() {
    const BOARD_DIM: usize = 5;
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.lines().collect();
    let crossing_numbers: Vec<u16> = lines[0]
        .split(",")
        .map(|x| x.parse::<u16>().unwrap())
        .collect();
    let boards: Vec<Vec<Vec<u16>>> = Regex::new(r"\r\n\r\n")
        .unwrap()
        .split(&contents)
        .skip(1)
        .map(|board| {
            board
                .split("\r\n")
                .map(|line| {
                    line.split_whitespace()
                        .map(|number| number.parse::<u16>().unwrap())
                        .collect::<Vec<u16>>()
                })
                .collect()
        })
        .collect();
    println!("{:?}", boards);


    let first_result = first_task::<BOARD_DIM>(&boards, &crossing_numbers);
    println!("{:?}", first_result);
    let second_result = second_task::<BOARD_DIM>(&boards, &crossing_numbers);
    println!("{:?}", second_result);
}