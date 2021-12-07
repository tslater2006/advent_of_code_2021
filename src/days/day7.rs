const INPUT: &str = include_str!("..\\..\\inputs\\day7sample.txt");

pub fn solve_part_1() {
    let mut crabs: Vec<i32> = INPUT.split(",").map(|s| s.parse().unwrap()).collect();
    crabs.sort();
    let median = crabs[crabs.len() / 2];
    let answer: i32 = crabs.iter().map(|c| (*c - median).abs()).sum();

    /* Original code for solve */
    /*
    let crabs: Vec<i32> = INPUT.split(",").map(|s| s.parse().unwrap()).collect();
    let largest = *crabs.iter().max().unwrap();
    let answer:i32 = (0..largest).map(|n| crabs.iter().map(|c| (*c - n).abs()).sum()).min().unwrap();
    */

    println!("Day #7 Part 1: {} ", answer);
}

fn fuel_cost_for_move(distance: i32) -> i32 {
    (distance * (distance + 1)) / 2
}

pub fn solve_part_2() {
    let crabs: Vec<i32> = INPUT.split(",").map(|s| s.parse().unwrap()).collect();
    let largest = *crabs.iter().max().unwrap();

    let answer: i32 = (0..largest)
        .map(|n| {
            crabs
                .iter()
                .map(|c| fuel_cost_for_move((*c - n).abs()))
                .sum()
        })
        .min()
        .unwrap();

    println!("Day #7 Part 2: {} ", answer);
}
