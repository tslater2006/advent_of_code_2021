const INPUT: &str = include_str!("..\\..\\inputs\\day2.txt");

struct Submarine {
    position: i32,
    depth: i32,
    aim: i32, /* for part 2 */
}

pub fn solve_part_1() {
    let directions: Vec<(&str, i32)> = INPUT
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(a, b)| (a, b.parse().unwrap()))
        .collect();

    let mut my_sub = Submarine {
        position: 0,
        depth: 0,
        aim: 0,
    };
    
    for x in directions {
        match x.0 {
            "down" => {
                my_sub.depth += x.1;
            }
            "up" => {
                my_sub.depth -= x.1;
            }
            "forward" => {
                my_sub.position += x.1;
            }
            _ => {
                panic!("Unknown instruction")
            }
        }
    }

    println!("Day #2 Part 1 answer: {}", my_sub.position * my_sub.depth);
}

pub fn solve_part_2() {
    let directions: Vec<(&str, i32)> = INPUT
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(a, b)| (a, b.parse().unwrap()))
        .collect();

    let mut my_sub = Submarine { position: 0, depth: 0, aim: 0 };

    for x in directions {
        match x.0 {
            "down" => {
                my_sub.aim += x.1;
            }
            "up" => {
                my_sub.aim -= x.1;
            }
            "forward" => {
                my_sub.position += x.1;
                my_sub.depth += my_sub.aim * x.1;
            }
            _ => {
                panic!("Unknown instruction")
            }
        }
    }

    println!("Day #2 Part 2 answer: {}", my_sub.position * my_sub.depth);
}
