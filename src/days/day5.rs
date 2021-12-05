use std::collections::HashMap;

const INPUT: &str = include_str!("..\\..\\inputs\\day5.txt");

#[derive(Debug)]
struct Line {
    point_1: (i16, i16),
    point_2: (i16, i16),
    travel_delta: (i16, i16),
}

impl Line {
    fn new(line_data: &str) -> Line {
        let coords = line_data.split(" -> ").collect::<Vec<&str>>();

        let p1 = coords[0]
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect::<Vec<i16>>();
        let p2 = coords[1]
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect::<Vec<i16>>();

        /* determine appropriate delta to get from p1 to p2 */
        let travel: (i16, i16);
        if p1[0] == p2[0] {
            /* X coords are the same */
            if p1[1] < p2[1] {
                travel = (0, 1);
            } else {
                travel = (0, -1);
            }
        } else if p1[1] == p2[1] {
            /* Y coords are the same */
            if p1[0] < p2[0] {
                travel = (1, 0);
            } else {
                travel = (-1, 0);
            }
        } else {
            /* we have a diagonal! */
            let x_delta = if p1[0] < p2[0] { 1 } else { -1 };
            let y_delta = if p1[1] < p2[1] { 1 } else { -1 };

            travel = (x_delta, y_delta);
        }

        Line {
            point_1: (p1[0], p1[1]),
            point_2: (p2[0], p2[1]),
            travel_delta: travel,
        }
    }

    fn is_at_angle(&self) -> bool {
        self.point_1.0 != self.point_2.0 && self.point_1.1 != self.point_2.1
    }
}

pub fn solve_part_1() {
    let input_lines: Vec<Line> = INPUT
        .lines()
        .map(|v| Line::new(v))
        .filter(|line| !line.is_at_angle())
        .collect();

    let mut points: HashMap<(i16, i16), u16> = HashMap::new();

    for l in input_lines {
        let mut cur_point = l.point_1.clone();

        while cur_point != l.point_2 {
            *points.entry(cur_point).or_insert(0) += 1;
            cur_point.0 += l.travel_delta.0;
            cur_point.1 += l.travel_delta.1;
        }
        *points.entry(cur_point).or_insert(0) += 1;
    }

    let answer = points.into_values().filter(|v| *v > 1).count();

    println!("Day 5 Part 1: {}", answer);
}

pub fn solve_part_2() {
    let input_lines: Vec<Line> = INPUT
        .lines()
        .map(|v| Line::new(v))
        .collect();

    let mut points: HashMap<(i16, i16), u16> = HashMap::new();

    for l in input_lines {
        let mut cur_point = l.point_1.clone();

        while cur_point != l.point_2 {
            *points.entry(cur_point).or_insert(0) += 1;
            cur_point.0 += l.travel_delta.0;
            cur_point.1 += l.travel_delta.1;
        }
        *points.entry(cur_point).or_insert(0) += 1;
    }

    let answer = points.into_values().filter(|v| *v > 1).count();

    println!("Day 5 Part 2: {}", answer);


}
