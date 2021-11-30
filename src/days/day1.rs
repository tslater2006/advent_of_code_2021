use std::fs;

pub fn solve_part_1() {
    let input_string: i32 = fs::read_to_string("inputs\\day1.txt")
        .unwrap()
        .lines()
        .map(|f| f.parse::<i32>().unwrap())
        .sum();

        println!("{}", input_string);
    
}

pub fn solve_part_2() {}
