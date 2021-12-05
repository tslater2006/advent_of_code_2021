mod days;

use std::{time::Instant, ops::Sub};

macro_rules! run_day {
    ($($a:ident),*) => {
        $(
            days::$a::solve_part_1();
            days::$a::solve_part_2();
        )*
    };
}


fn main() {

    /* silly hack to stop "dead code" warnings */

    if 1 == 0 {
        run_day!(day1,day2, day3, day4);
    }

    let start = Instant::now();
    run_day!(day5);
    let stop = Instant::now();

    println!("Ran in {:?}", stop.sub(start));
}

