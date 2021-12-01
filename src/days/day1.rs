const INPUT: &str = include_str!("..\\..\\inputs\\day1.txt");

pub fn solve_part_1() {
    let list_of_ints: Vec<i32> = INPUT.lines().map(|f| f.parse::<i32>().unwrap()).collect();

    //println!("Part 1 answer: {:?}", list_of_ints);
    let mut increase_count = 0;
    for x in 1..list_of_ints.len() {
        if list_of_ints[x] > list_of_ints[x-1] {
            increase_count += 1;
        }
    }

    println!("Part 1 Answer: {}", increase_count);
}


pub fn solve_part_2() {
    let list_of_ints: Vec<i32> = INPUT.lines().map(|f| f.parse::<i32>().unwrap()).collect();
    let mut increases = 0;
    for x in 3..list_of_ints.len() {
        let a = list_of_ints[x-3] + list_of_ints[x-2] + list_of_ints[x-1];
        let b = list_of_ints[x] + list_of_ints[x-1] + list_of_ints[x-2];

        if b > a {
            increases += 1;
        }
    }
    println!("Part 2 Answer: {}", increases);
}
