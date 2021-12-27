use rayon::prelude::*;
use std::time::Instant;




fn asyncio() -> () {

    let list:Vec<i128> = (2..200).collect::<Vec<_>>();
    let result:Vec<_> = list.into_par_iter().map(|item| inner(item)).collect();
    
}


fn inner(base:i128) -> i128 {
    let mut result:i128 = 0;
    for i in 0..2024967i128 {
        result+=i128::pow(base,i as u32);
    }
    result
}

fn syncio() -> () {

    let list:Vec<i128> = (2..200).collect::<Vec<_>>();

    let result:Vec<_> = list.into_iter().map(|item| inner(item)).collect();
}


fn main() {
    let before = Instant::now();

    let second_result = syncio();
    println!("Elapsed time: {:.2?}", before.elapsed());
    println!("{:#?}", second_result);

    let before = Instant::now();

    let first_result = asyncio();
    println!("Elapsed time: {:.2?}", before.elapsed());
    println!("{:#?}", first_result);
}
