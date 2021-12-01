const INPUT: &str = include_str!("..\\..\\inputs\\day1.txt");

pub fn solve_part_1() {
    let list_of_ints: Vec<i32> = INPUT.lines().map(|f| f.parse::<i32>().unwrap()).collect();

    //println!("Part 1 answer: {:?}", list_of_ints);
    let increase_count = count_increases(&list_of_ints);
    println!("Part 1 Answer: {}", increase_count);
}

fn count_increases(vec: &Vec<i32>) -> i32 {
    let mut last_num = vec[0];
    let mut increase_count = 0;
    for x in 1..vec.len() {
        if vec[x] > last_num {
            increase_count += 1;
        }
        last_num = vec[x];
    }

    increase_count
}

pub fn solve_part_2() {
    let list_of_ints: Vec<i32> = INPUT.lines().map(|f| f.parse::<i32>().unwrap()).collect();

    let mut sums_of_three: Vec<i32> = Vec::new();
    let mut current_sum = 0;
    let mut index = 0;
    let mut nums_added = 0;
    while index < list_of_ints.len() {
        current_sum += list_of_ints[index];
        nums_added += 1;
        if nums_added == 3 {
            sums_of_three.push(current_sum);
            current_sum = 0;
            index -= 1;
            nums_added = 0;
        } else {
            index += 1;
        }
    }

    let answer = count_increases(&sums_of_three);
    println!("Part 2 Answer: {}", answer);
}
