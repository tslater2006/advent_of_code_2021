const INPUT: &str = include_str!("..\\..\\inputs\\day1.txt");

pub fn solve_part_1() {
    let list_of_ints: Vec<i32> = INPUT.lines().map(|f| f.parse::<i32>().unwrap()).collect();

    let part1_answer = list_of_ints.windows(2).filter(|f| f[0] < f[1]).count();

    println!("Day #1 Part 1 Answer: {}", part1_answer);
    /* let mut increase_count = 0;
    for x in 1..list_of_ints.len() {
        if list_of_ints[x] > list_of_ints[x-1] {
            increase_count += 1;
        }
    }
    println!("Part 1 Answer: {}", increase_count);
    */
}

pub fn solve_part_2() {
    let list_of_ints: Vec<i32> = INPUT.lines().map(|f| f.parse::<i32>().unwrap()).collect();

    let groups_of_three: Vec<i32> = list_of_ints.windows(3).map(|w| w.iter().sum()).collect();

    let part2_answer = groups_of_three.windows(2).filter(|f| f[0] < f[1]).count();
    println!("Day #1 Part 2 Answer: {}", part2_answer);

    /* let mut increases = 0;
    for x in 3..list_of_ints.len() {
        let a = list_of_ints[x - 3] + list_of_ints[x - 2] + list_of_ints[x - 1];
        let b = list_of_ints[x] + list_of_ints[x - 1] + list_of_ints[x - 2];

        if b > a {
            increases += 1;
        }

    }
    //println!("Part 2 Answer: {}", increases);
    */
}
