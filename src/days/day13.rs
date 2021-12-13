use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("..\\..\\inputs\\day13.txt");

pub fn solve_part_1() {
    let (mut points, instructions) = parse_input();
    for inst in instructions {
        match inst.0 {
            0 => {
                for i in 0..points.len() {
                    if points[i].0 < inst.1 {
                        points[i].0 += ((inst.1 - points[i].0).abs()) * 2;
                    }
                }
            }
            1 => {
                for i in 0..points.len() {
                    if points[i].1 > inst.1 {
                        points[i].1 -= (points[i].1 - inst.1).abs() * 2;
                    }
                }
            }
            _ => unreachable!(),
        }
        break;
    }
    let mut unique_points = HashSet::new();
    for i in 0..points.len() {
        unique_points.insert(points[i]);
    }

    println!("Day #13 Part 1: {}", unique_points.len());
}

#[allow(dead_code)]
fn print_debug_grid(points: &Vec<(i32, i32)>, width: usize, height: usize) {
    for y in 0..height {
        for x in 0..width {
            if points.contains(&(x as i32, y as i32)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn parse_input() -> (Vec<(i32, i32)>, Vec<(u8, i32)>) {
    let mut lines = INPUT.lines();

    let mut points: Vec<(i32, i32)> = Vec::new();
    let mut instrs: Vec<(u8, i32)> = Vec::new();

    /*  */
    loop {
        let line = lines.next().unwrap();
        if line == "" {
            break;
        } else {
            let parsed_points: Vec<i32> = line.split(",").map(|p| p.parse().unwrap()).collect();
            points.push((parsed_points[0], parsed_points[1]));
        }
    }

    loop {
        let line = lines.next();
        match line {
            None => break,
            Some(l) => {
                let instr_text: Vec<&str> = l.split(" ").collect();

                let mut instr_pieces = instr_text[2].split("=");

                let orientation: u8 = if instr_pieces.next().unwrap() == "x" {
                    0
                } else {
                    1
                };
                let amount: i32 = instr_pieces.next().unwrap().parse().unwrap();

                instrs.push((orientation, amount))
            }
        }
    }

    (points, instrs)
}

pub fn solve_part_2() {
    let (mut points, instructions) = parse_input();
    for inst in instructions {
        match inst.0 {
            0 => {
                for i in 0..points.len() {
                    if points[i].0 > inst.1 {
                        points[i].0 -= ((inst.1 - points[i].0).abs()) * 2;
                    }
                }
            }
            1 => {
                for i in 0..points.len() {
                    if points[i].1 > inst.1 {
                        points[i].1 -= (points[i].1 - inst.1).abs() * 2;
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    let mut unique_points = HashSet::new();
    let mut max = (0, 0);

    for i in 0..points.len() {
        if points[i].0 > max.0 {
            max.0 = points[i].0
        }
        if points[i].1 > max.1 {
            max.1 = points[i].1
        }
        unique_points.insert(points[i]);
    }

    max.0 += 1;
    max.1 += 1;

    let ans = ocr_letters(&unique_points, max);

    println!("Day #13 Part 2: {}", ans);
}

fn ocr_letters(points: &HashSet<(i32, i32)>, max: (i32, i32)) -> String {
    let mut found_string = String::new();
    let mut letter_lookup: HashMap<u32, char> = HashMap::new();
    letter_lookup.insert(0b011010011001111110011001u32, 'A');
    letter_lookup.insert(0b111010011110100110011110u32, 'B');
    letter_lookup.insert(0b011010011000100010010110u32, 'C');
    letter_lookup.insert(0b111110001110100010001111u32, 'E');
    letter_lookup.insert(0b111110001110100010001000u32, 'F');
    letter_lookup.insert(0b011010011000101110010111u32, 'G');
    letter_lookup.insert(0b100110011111100110011001u32, 'H');
    letter_lookup.insert(0b001100010001000110010110u32, 'J');
    letter_lookup.insert(0b100110101100101010101001u32, 'K');
    letter_lookup.insert(0b100010001000100010001111u32, 'L');
    letter_lookup.insert(0b111010011001111010001000u32, 'P');
    letter_lookup.insert(0b111010011001111010101001u32, 'R');
    letter_lookup.insert(0b100110011001100110010110u32, 'U');
    letter_lookup.insert(0b100010000101001000100010u32, 'Y');
    letter_lookup.insert(0b111100010010010010001111u32, 'Z');
    letter_lookup.insert(0b000000000000000000000000u32, ' ');

    let mut grid: Vec<Vec<u8>> = Vec::with_capacity(max.1 as usize);
    for y in 0..max.1 {
        let mut x_vec = Vec::with_capacity(max.0 as usize);
        for x in 0..max.0 {
            x_vec.push(points.contains(&(x, y)) as u8);
        }
        grid.push(x_vec)
    }

    let mut x_offset = 0;

    while x_offset < grid[0].len() {
        let mut letter_val: u32 = 0;
        /* read letter here */
        for y in 0..grid.len() {
            for x in 0..4 {
                letter_val <<= 1;
                letter_val += grid[y][x + x_offset] as u32
            }
        }

        if letter_lookup.contains_key(&letter_val) {
            found_string.push(*letter_lookup.get(&letter_val).unwrap());
        }

        x_offset += 5
    }

    found_string
}
