use std::collections::HashMap;

const INPUT: &str = include_str!("..\\..\\inputs\\day11.txt");

pub fn solve_part_1() {
    let mut grid: Vec<Vec<u8>> = INPUT
        .lines()
        .map(|l| l.as_bytes().iter().map(|b| *b - b'0').collect())
        .collect();

    let mut neighbor_map: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let neighbors = neighbor_points(x, y, grid[y].len() as i8, grid.len() as i8);
            neighbor_map.insert((x, y), neighbors);
        }
    }

    let mut total_flashes = 0;

    for _ in 0..100 {
        total_flashes += run_step(&mut grid, &neighbor_map);
    }

    println!("Day #11 Part 1: {}", total_flashes);
}

pub fn solve_part_2() {
    let mut grid: Vec<Vec<u8>> = INPUT
        .lines()
        .map(|l| l.as_bytes().iter().map(|b| *b - b'0').collect())
        .collect();

    let mut neighbor_map: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    let step_count = run_until_sync_or_step(&mut grid, &mut neighbor_map, -1);

    println!("Day #11 Part 2: {}", step_count);
}

fn run_until_sync_or_step(
    grid: &mut Vec<Vec<u8>>,
    neighbor_map: &mut HashMap<(usize, usize), Vec<(usize, usize)>>,
    count: i32,
) -> u32 {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let neighbors = neighbor_points(x, y, grid[y].len() as i8, grid.len() as i8);
            neighbor_map.insert((x, y), neighbors);
        }
    }

    let mut total_flashes = 0;
    let mut step_count = 0;
    while total_flashes != grid.len() * grid[0].len() {
        total_flashes = run_step(grid, &neighbor_map);
        step_count += 1;

        if count >= 0 && step_count > count as u32 {
            return 0;
        }
    }

    step_count
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<u8>>) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            print!("{}", (grid[y][x] + b'0') as char);
        }
        println!();
    }
    println!();
}

fn run_step(
    grid: &mut Vec<Vec<u8>>,
    point_map: &HashMap<(usize, usize), Vec<(usize, usize)>>,
) -> usize {
    let mut flashers: Vec<(usize, usize)> = Vec::new();
    let mut processed_flashers: Vec<(usize, usize)> = Vec::new();
    /* add 1 everywhere */
    for p in point_map.keys() {
        grid[p.1][p.0] += 1;

        if grid[p.1][p.0] > 9 {
            flashers.push(*p);
        }
    }

    while flashers.len() > 0 {
        let p = flashers.pop().unwrap();

        for n in point_map.get(&p).unwrap() {
            grid[n.1][n.0] += 1;
            if grid[n.1][n.0] > 9 {
                if !flashers.contains(n) && !processed_flashers.contains(n) {
                    flashers.push(*n);
                }
            }
        }

        processed_flashers.push(p);
    }

    let flash_count = processed_flashers.len();

    for p in processed_flashers {
        grid[p.1][p.0] = 0;
    }

    flash_count
}

fn neighbor_points(x: usize, y: usize, width: i8, height: i8) -> Vec<(usize, usize)> {
    let directions: Vec<(i8, i8)> = vec![
        (1, 0),
        (0, 1),
        (-1, 0),
        (0, -1),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];

    let mut results: Vec<(usize, usize)> = Vec::new();

    for dir in directions {
        let test_point: (i8, i8) = (x as i8 + dir.0, y as i8 + dir.1);

        if test_point.0 < 0 || test_point.0 >= width || test_point.1 < 0 || test_point.1 >= height {
            continue;
        }

        results.push((test_point.0 as usize, test_point.1 as usize));
    }

    results
}
