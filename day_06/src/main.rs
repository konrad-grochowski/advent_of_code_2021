use std::fs;

fn first_task(mut numbers: Vec<u8>, iterations: u16) -> usize {
    for iteration in 0..iterations {
        println!("{}", iteration);
        let mut zeroes_count = 0;
        numbers.iter_mut().for_each(|x| {
            if *x != 0 {
                *x -= 1
            } else {
                *x = 6;
                zeroes_count += 1
            }
        });
        numbers.append(&mut vec![8; zeroes_count]);
    }
    return numbers.len();
}

fn second_task(numbers: Vec<u8>, iterations: u16) -> u64 {
    let mut smart_numbers = numbers
    .iter()
    .fold(vec![0;9], |mut acc:Vec<u64>, number:&u8| {
        acc[*number as usize] += 1;
        acc
    });
    for iteration in 0..iterations {
        println!("{}",iteration);
        let zeroes = smart_numbers[0];
        smart_numbers.drain(0..1);
        smart_numbers[6]+=zeroes;
        smart_numbers.push(zeroes);
    }
    return smart_numbers.into_iter().sum()
}

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let numbers: Vec<u8> = contents
        .strip_suffix("\r\n")
        .unwrap()
        .split(",")
        .map(|number| number.parse::<u8>().unwrap())
        .collect();

    const FIRST_TASK_ITER_NUM: u16 = 80;
    let first_result = first_task(numbers.to_owned(), FIRST_TASK_ITER_NUM);
    println!("{:?}", first_result);
    const SECOND_TASK_ITER_NUM: u16 = 256;
    let second_result = second_task(numbers.to_owned(), SECOND_TASK_ITER_NUM);
    println!("{:?}", second_result);
}
