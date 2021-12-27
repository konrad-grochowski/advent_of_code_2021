use std::fs;
/*
    It can by proven that the result needs to be
    either at the median index if the number of elements is odd,
    or between the middle elements (inclusive) if the number of elements is even.
    Short summary of the proof by contradiction is: if you choose a potentially perfect position
    and the number of elements on the left is greater than the number of elements on the right,
    then you can try to reduce the number of steps they have to make,
    if you move the position one step in the direction of a bigger number of elements.
    The reason is: if you move in a chosen direction,
    the elements coming from this direction will have to make x fewer steps:
    x is equal to numbers of elements coming from this direction.
    Consequently, elements from the other direction will have to make y more steps,
    where y is the number of elemnents coming from the other direction. But since we know that x > y,
    then we decrease the number of steps if we move to the chosen direction. Following that logic we end up in a spot
    where number of elements on the left is equal to number of elements on the right;
    this is the spot where the elements will have to make the least number of steps.

*/
fn first_task(numbers: &[i64]) -> i64 {
    let median_idx = numbers.len() / 2;
    let median = numbers[median_idx];
    return numbers.iter().map(|n| (n - median).abs()).sum();
}

/*
    This can be proven analytically using derivatives in contrast to the first task,
    since in this example we can differentiate freely due to lack of absolute values in the equation.
    We have to use the fact that 1+2+3+...+n = n*(n+1)/2

*/

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
    let mut numbers: Vec<i64> = contents
        .strip_suffix("\r\n")
        .unwrap()
        .split(",")
        .map(|number| number.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    numbers.sort();
    println!("{:#?}", numbers[499]);
    let first_result = first_task(&numbers);
    println!("{:?}", first_result);
    let second_result = second_task(&numbers);
    println!("{:?}", second_result);
}
