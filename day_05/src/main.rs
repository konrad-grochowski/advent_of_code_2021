use std::fs;

fn first_task(tuples_pairs: Vec<Vec<Vec<i16>>>, N: usize) -> usize {
    let mut board: Vec<Vec<i16>> = vec![vec![0; N]; N];
    for tuple_pair in tuples_pairs {
        let (mut x, mut y) = (tuple_pair[0][0], tuple_pair[0][1]);
        let (end_x, end_y) = (tuple_pair[1][0], tuple_pair[1][1]);
        // if x != end_x && y != end_y {
        //     continue;
        // }
        let mut delta_x = end_x - x;
        let mut delta_y = end_y - y;
        if delta_x != 0 {
            delta_x /= delta_x.abs();
        }
        if delta_y != 0 {
            delta_y /= delta_y.abs();
        }
        
        board[x as usize][y as usize] += 1;
        while x != end_x || y != end_y {
            x += delta_x;
            y += delta_y;
            board[x as usize][y as usize] += 1;
        }
       
    }

    board
        .iter()
        .flatten()
        .filter(|&&x| x >= 2)
        .collect::<Vec<&i16>>()
        .len()
}

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.lines().collect();
    let tuples_pairs: Vec<Vec<Vec<i16>>> = lines
        .iter()
        .map(|line| {
            line.split(" -> ")
                .map(|tuple| {
                    tuple
                        .split(",")
                        .map(|number| number.parse::<i16>().unwrap())
                        .collect::<Vec<i16>>()
                })
                .collect()
        })
        .collect();
    // println!("{:#?}",tuples_pairs);
    const N: usize = 1000;

    let first_result = first_task(tuples_pairs, N);
    println!("{:?}", first_result)
}
