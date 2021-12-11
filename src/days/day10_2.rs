use std::collections::HashMap;

const INPUT: &str = include_str!("..\\..\\inputs\\day10.txt");
extern crate peg;

peg::parser!( grammar chunks() for str {

    rule chunk() =  "(" chunk()* ")" /
                    "[" chunk()* "]" /
                    "{" chunk()* "}" /
                    "<" chunk()* ">"

    pub rule chunks() = chunk()+

});

pub fn solve_part_1() {
    let lines: Vec<&str> = INPUT.lines().collect();
    let mut ans = 0;
    for x in lines {
        let result = chunks::chunks(x);

        match result {
            Err(e) if e.location.column != x.len() + 1 => {
                //println!("Found corrupt at {}: {}", e.location.column, x);
                match x.as_bytes()[e.location.column - 1] {
                    b')' => ans += 3,
                    b']' => ans += 57,
                    b'}' => ans += 1197,
                    b'>' => ans += 25137,
                    _ => {}
                }
            }
            _ => {}
        }
    }

    println!("Day #10 Part 1: {}", ans);
}

pub fn solve_part_2() {
    let lines: Vec<&str> = INPUT.lines().collect();
    let mut autocomplete_scores: Vec<usize> = Vec::new();

    for x in lines {
        let result = chunks::chunks(x);

        match result {
            Err(e) if e.location.column == x.len() + 1 => {
                autocomplete_scores.push(get_completion_score(x));
            }
            _ => {}
        }
    }

    autocomplete_scores.sort();
    let ans: usize = autocomplete_scores[autocomplete_scores.len() / 2];
    println!("Day #10 Part 2: {} ", ans);
}

fn get_completion_score(line: &str) -> usize {
    let mut bytes: Vec<u8> = line.chars().map(|f| f as u8).collect();

    let mut close_counts = [0u8; 4];

    let mut paren_map: HashMap<u8, usize> = HashMap::new();
    paren_map.insert(b'(', 0);
    paren_map.insert(b'[', 1);
    paren_map.insert(b'{', 2);
    paren_map.insert(b'<', 3);
    paren_map.insert(b')', 0);
    paren_map.insert(b']', 1);
    paren_map.insert(b'}', 2);
    paren_map.insert(b'>', 3);

    for x in (0..bytes.len()).rev() {
        match bytes[x] {
            b')' | b']' | b'}' | b'>' => {
                close_counts[*paren_map.get(&bytes[x]).unwrap()] += 1;
                bytes[x] = 0
            }

            b'(' | b'[' | b'{' | b'<' => {
                let index = paren_map.get(&bytes[x]).unwrap();
                if close_counts[*index] > 0 {
                    close_counts[*index] -= 1;
                    bytes[x] = 0
                }
            }
            _ => {}
        }
    }

    let mut ans = 0;
    for x in (0..bytes.len()).rev() {
        if bytes[x] != 0 {
            ans *= 5;
            ans += match bytes[x] {
                b'(' => 1,
                b'[' => 2,
                b'{' => 3,
                b'<' => 4,
                _ => unreachable!(),
            };
        }
    }
    ans
}
