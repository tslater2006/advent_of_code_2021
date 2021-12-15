mod days;

use std::{ops::Sub, time::Instant};
#[macro_use]
extern crate lazy_static;

macro_rules! run_day {
    ($($a:ident),*) => {
        $(
            let day_start = Instant::now();
            days::$a::solve_part_1();
            days::$a::solve_part_2();
            let day_stop = Instant::now();
            println!("Day ran in: {}ms\n", day_stop.sub(day_start).as_secs_f64() * 1000.0);
        )*
    };
}

fn main() {
    let start = Instant::now();

    /* silly hack to stop "dead code" warnings */
    if 1 == 0 {
        run_day!(day1, day2, day3, day4, day5, day6, day7, day8, day9, day10, day10_2);
        run_day!(day11, day12, day13, day14);
    }

    run_day!(day15);
    let stop = Instant::now();

    println!("Ran in {:?}ms", stop.sub(start).as_secs_f64() * 1000.0);
}
