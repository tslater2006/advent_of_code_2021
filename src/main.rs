mod days;

use std::{time::Instant, ops::Sub};

macro_rules! run_day {
    ($a:ident) => {
        days::$a::solve_part_1();
        days::$a::solve_part_2();
    };
}


fn main() {
    let start = Instant::now();
    run_day!(day1);
    let stop = Instant::now();

    let difference = stop.sub(start);
    println!("Ran in {:?}", difference);
}

