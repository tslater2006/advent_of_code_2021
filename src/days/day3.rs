const INPUT: &str = include_str!("..\\..\\inputs\\day3.txt");

pub fn solve_part_1() {
    let input_lines: Vec<&str> = INPUT.lines().collect();
    let frequent_bits = get_most_frequent_bits(&input_lines);

    let mut gamma = 0;
    let mut epsilon = 0;

    for count in frequent_bits {
        gamma <<= 1;
        epsilon <<= 1;

        match count {
            1 => {
                gamma += 1;
            }
            _ => {
                epsilon += 1;
            }
        }
    }
    println!("Day 3 Part 1: {}", gamma * epsilon);
}

pub fn solve_part_2() {
    let input_lines: Vec<&str> = INPUT.lines().collect();

    let mut frequent_bits = get_most_frequent_bits(&input_lines);
    let mut oxygen_lines: Vec<&str> = Vec::new();
    let mut co2_lines: Vec<&str> = Vec::new();

    for line in input_lines.iter() {
        if line.as_bytes()[0] - 0x30 == frequent_bits[0] {
            oxygen_lines.push(line);
        } else {
            co2_lines.push(line);
        }
    }
    let mut cur_bit = 1;
    while oxygen_lines.len() > 1 {
        frequent_bits = get_most_frequent_bits(&oxygen_lines);
        oxygen_lines.retain(|l| l.as_bytes()[cur_bit] - 0x30 == frequent_bits[cur_bit]);
        cur_bit += 1;
    }

    cur_bit = 1;
    while co2_lines.len() > 1 {
        frequent_bits = get_most_frequent_bits(&co2_lines);
        co2_lines.retain(|l| l.as_bytes()[cur_bit] - 0x30 != frequent_bits[cur_bit]);
        cur_bit += 1;
    }

    let oxygen_answer = oxygen_lines[0].as_bytes().iter().fold(0 as u32, |a,b | (a << 1) + (*b-0x30) as u32);
    let co2_answer = co2_lines[0].as_bytes().iter().fold(0 as u32, |a,b | (a << 1) + (*b-0x30) as u32);
    
    println!("Day 3 Part 2: {:?}", oxygen_answer * co2_answer);
}

fn get_most_frequent_bits(list: &Vec<&str>) -> Vec<u8> {
    let line_count = list.len() as u32;
    let line_width:u32 = list[0].len() as u32;

    let bit_threshold: u32 = match line_count % 2 {
        1 => (line_count + 1) / 2,
        0 => line_count / 2,
        _ => panic!()
    };
    let mut bit_counts = vec![0 as u32; line_width as usize];

    for line in list {
        line.chars()
            .map(|c| c.to_digit(10).unwrap())
            .enumerate()
            .for_each(|(i, x)| {
                bit_counts[i] += x;
            });
    }

    let mut most_frequent = Vec::new();
    for count in bit_counts {
        most_frequent.push(match count >= bit_threshold {
            true => 1,
            false => 0,
        });
    }
    return most_frequent;
}
