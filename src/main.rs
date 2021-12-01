mod days;

macro_rules! run_day {
    ($a:ident) => {
        days::$a::solve_part_1();
        days::$a::solve_part_2();
    };
}


fn main() {
    run_day!(day1);
}

