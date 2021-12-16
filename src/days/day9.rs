use pathfinding::prelude::bfs_reach;
const INPUT: &str = include_str!("..\\..\\inputs\\day9.txt");

pub fn solve_part_1() {
    let map: Vec<Vec<u8>> = INPUT
        .lines()
        .into_iter()
        .map(|l| l.chars().map(|c| c as u8 - b'0').collect())
        .collect();

    let low_spots: Vec<(i32, i32)> = get_low_spots(&map);

    let ans: i32 = low_spots
        .into_iter()
        .map(|s| map[s.1 as usize][s.0 as usize] as i32 + 1)
        .sum();

    println!("Day #9 Part 1: {}", ans);
}

pub fn solve_part_2() {
    let map: Vec<Vec<u8>> = INPUT
        .lines()
        .into_iter()
        .map(|l| l.chars().map(|c| c as u8 - b'0').collect())
        .collect();

    let low_spots: Vec<(i32, i32)> = get_low_spots(&map);

    let mut basin_sizes: Vec<usize> = low_spots
        .into_iter()
        .map(|s| {
            bfs_reach(s, |p| {
                let next_items: Vec<(i32, i32)> = get_neighbors(*p, &map)
                    .into_iter()
                    .filter(|i| map[i.1 as usize][i.0 as usize] <= 8)
                    .collect();

                next_items
            })
            .count()
        })
        .collect();
    basin_sizes.sort_unstable();
    basin_sizes.reverse();

    let mut ans = 1;
    for size in basin_sizes.iter().take(3) {
        ans *= size;
    }

    println!("Day #9 Part 2: {} ", ans);
}

fn get_neighbors(point: (i32, i32), map: &[Vec<u8>]) -> Vec<(i32, i32)> {
    let neighbor_checks: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut list: Vec<(i32, i32)> = Vec::new();
    for check in neighbor_checks {
        let new_check_pos = (point.0 + check.0, point.1 + check.1);

        if new_check_pos.0 < 0 || new_check_pos.0 >= map[point.1 as usize].len() as i32 {
            continue;
        }

        if new_check_pos.1 < 0 || new_check_pos.1 >= map.len() as i32 {
            continue;
        }

        list.push((new_check_pos.0, new_check_pos.1));
    }
    list
}

fn get_low_spots(map: &[Vec<u8>]) -> Vec<(i32, i32)> {
    let mut low_spots: Vec<(i32, i32)> = Vec::new();
    for y in 0..map.len() as i32 {
        for x in 0..map[0].len() as i32 {
            let cur_val = map[y as usize][x as usize];

            let neighbors = get_neighbors((x, y), map);

            if neighbors.len()
                == neighbors
                    .iter()
                    .map(|n| {
                        if map[n.1 as usize][n.0 as usize] > cur_val {
                            1
                        } else {
                            0
                        }
                    })
                    .sum()
            {
                low_spots.push((x, y));
            }
        }
    }
    low_spots
}
