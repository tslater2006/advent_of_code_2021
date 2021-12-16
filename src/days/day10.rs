use std::{collections::HashMap, mem::discriminant};

const INPUT: &str = include_str!("..\\..\\inputs\\day10.txt");

lazy_static! {
    static ref OPEN_CHARS: HashMap<char, Chunk> = vec![
        ('(', Chunk::Paren),
        ('[', Chunk::Square),
        ('{', Chunk::Curly),
        ('<', Chunk::Angle),
    ]
    .into_iter()
    .collect();
    static ref CLOSE_CHARS: HashMap<char, Chunk> = vec![
        (')', Chunk::Paren),
        (']', Chunk::Square),
        ('}', Chunk::Curly),
        ('>', Chunk::Angle),
    ]
    .into_iter()
    .collect();
}

pub fn solve_part_1() {
    let lines: Vec<&str> = INPUT.lines().into_iter().collect();

    let results: Vec<ValidationResult> = lines.iter().map(|l| validate_line(l)).collect();
    let mut error_score: usize = 0;

    for r in results {
        match r {
            ValidationResult::OK => {}
            ValidationResult::Incomplete(_) => {}
            ValidationResult::Corrupt(_, c) => {
                error_score += match c {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => unreachable!(),
                }
            }
        }
    }

    println!("Day #10 Part 1: {} ", error_score);
}

#[derive(Debug, Clone, Copy)]
enum Chunk {
    Paren,
    Square,
    Curly,
    Angle,
}

#[derive(Debug)]
enum ValidationResult {
    OK,
    Incomplete(Vec<Chunk>),
    Corrupt(Chunk, char),
}

fn validate_line(line: &str) -> ValidationResult {
    let mut chunk_stack: Vec<Chunk> = Vec::new();

    for x in line.chars() {
        match OPEN_CHARS.get(&x) {
            Some(c) => {
                chunk_stack.push(*c);
            }
            None => {
                if let Some(c) = CLOSE_CHARS.get(&x) {
                    let cur_chunk = chunk_stack.pop().unwrap();

                    if discriminant(c) != discriminant(&cur_chunk) {
                        return ValidationResult::Corrupt(cur_chunk, x);
                    }
                }
            }
        }
    }

    if !chunk_stack.is_empty() {
        ValidationResult::Incomplete(chunk_stack)
    } else {
        ValidationResult::OK
    }
}

pub fn solve_part_2() {
    let lines: Vec<&str> = INPUT.lines().into_iter().collect();

    let results: Vec<ValidationResult> = lines.iter().map(|l| validate_line(l)).collect();
    let mut autocomplete_scores: Vec<usize> = Vec::new();

    for r in results {
        if let ValidationResult::Incomplete(mut chunks) = r {
            let mut cur_score = 0;
            while !chunks.is_empty() {
                cur_score *= 5;

                let cur_chunk = chunks.pop().unwrap();
                match cur_chunk {
                    Chunk::Angle => cur_score += 4,
                    Chunk::Curly => cur_score += 3,
                    Chunk::Paren => cur_score += 1,
                    Chunk::Square => cur_score += 2,
                }
            }
            autocomplete_scores.push(cur_score);
        }
    }

    autocomplete_scores.sort_unstable();
    let ans: usize = autocomplete_scores[autocomplete_scores.len() / 2];
    println!("Day #10 Part 2: {} ", ans);
}
