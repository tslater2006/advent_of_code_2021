use std::fs;

pub fn solve_part_1() {
    let sum_of_ints: i32 = fs::read_to_string("inputs\\day1.txt")
        .unwrap()
        .lines()
        .map(|f| f.parse::<i32>().unwrap())
        .sum();

    println!("Part 1 answer: {}", sum_of_ints);
}

pub fn solve_part_2() {
    println!("Part 2 answer: ");
}
