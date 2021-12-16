use std::{collections::HashMap};

const INPUT: &str = include_str!("..\\..\\inputs\\day6.txt");

fn get_spawn_count(cache: &mut HashMap<(usize, usize), usize>, age: usize, days: usize) -> usize {
    match cache.get(&(age, days)) {
        Some(v) => {
            *v
        }
        None => {
            let mut spawn_count = 0;
            if days < 7 || days < (age + 1) {
                return spawn_count;
            }
            let num_fish_made: usize = 1 + ((days - (age + 1)) / 7);

            for x in 0..num_fish_made {
                let fish_start_day = days - (age + 1) - (7 * x);
                spawn_count += 1 + get_spawn_count(cache, 8, fish_start_day);
            }

            cache.insert((age,days),spawn_count);
            spawn_count
        }
    }
}

pub fn solve_part_1() {
    let fish: Vec<u8> = INPUT.split(',').map(|s| s.parse().unwrap()).collect();
    let mut cache: HashMap<(usize, usize), usize> = HashMap::new();
    let ans: usize = fish
        .iter()
        .map(|f| 1 + get_spawn_count(&mut cache, *f as usize, 80))
        .sum();

    println!("Day #6 Part 1: {}", ans);

}

pub fn solve_part_2() {
    let fish: Vec<u8> = INPUT.split(',').map(|s| s.parse().unwrap()).collect();
    let mut cache: HashMap<(usize, usize), usize> = HashMap::new();
    let ans: usize = fish
        .iter()
        .map(|f| 1 + get_spawn_count(&mut cache, *f as usize, 256))
        .sum();

    println!("Day #6 Part 2: {}", ans);
}
