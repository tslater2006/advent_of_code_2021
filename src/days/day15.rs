use std::collections::HashMap;

use pathfinding::prelude::dijkstra;

const INPUT: &str = include_str!("..\\..\\inputs\\day15.txt");

pub fn solve_part_1() {
    let grid: Vec<Vec<usize>> = INPUT
        .lines()
        .map(|l| l.as_bytes().iter().map(|b| (*b - b'0') as usize).collect())
        .collect();

    let mut neighbor_map: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let neighbors = neighbor_points(x, y, grid[y].len() as i16, grid.len() as i16);
            neighbor_map.insert((x as usize, y as usize), neighbors);
        }
    }

    let start = (0_usize, 0_usize);
    //let stop = (0 as usize, 0 as usize);
    let stop = ((grid.len() - 1) as usize, (grid[0].len() - 1) as usize);

    let result = dijkstra(
        &start,
        |p| {
            let r: Vec<((usize, usize), usize)> = neighbor_map
                .get(p)
                .unwrap()
                .iter()
                .map(|s| (*s, grid[p.1 as usize][p.0 as usize]))
                .collect();

            r
        },
        |p| *p == stop,
    );

    println!("Day #15 Part 1: {}", result.unwrap().1);
}

pub fn solve_part_2() {
    let grid: Vec<Vec<usize>> = INPUT
        .lines()
        .map(|l| l.as_bytes().iter().map(|b| (*b - b'0') as usize).collect())
        .collect();

    let mut neighbor_map: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();

    for y in 0..grid.len() * 5 {
        for x in 0..grid[0].len() * 5 {
            let neighbors =
                neighbor_points(x, y, (grid[0].len() * 5) as i16, (grid.len() * 5) as i16);
            neighbor_map.insert((x as usize, y as usize), neighbors);
        }
    }

    let start = (0_usize, 0_usize);
    let stop = (
        ((grid.len() * 5) - 1) as usize,
        ((grid[0].len() * 5) - 1) as usize,
    );

    let result = dijkstra(
        &start,
        |p| {
            let r: Vec<((usize, usize), usize)> = neighbor_map
                .get(p)
                .unwrap()
                .iter()
                .map(|s| {
                    let cost_x = s.0 % grid[0].len();
                    let cost_y = s.1 % grid.len();

                    let base_cost = grid[cost_y][cost_x];

                    let inc_count_x = s.0 / grid[0].len();
                    let inc_count_y = s.1 / grid.len();

                    let scaled_cost = base_cost + (inc_count_x + inc_count_y);

                    let correct_cost = scaled_cost - ((scaled_cost / 10) * 9);

                    (*s, correct_cost)
                })
                .collect();

            r
        },
        |p| *p == stop,
    );

    println!("Day #15 Part 2: {}", result.unwrap().1);
}

fn neighbor_points(x: usize, y: usize, width: i16, height: i16) -> Vec<(usize, usize)> {
    let directions: Vec<(i16, i16)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];

    let mut results: Vec<(usize, usize)> = Vec::new();

    for dir in directions {
        let test_point: (i16, i16) = (x as i16 + dir.0, y as i16 + dir.1);

        if test_point.0 < 0 || test_point.0 >= width || test_point.1 < 0 || test_point.1 >= height {
            continue;
        }

        results.push((test_point.0 as usize, test_point.1 as usize));
    }

    results
}
