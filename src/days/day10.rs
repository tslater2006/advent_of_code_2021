const INPUT: &str = include_str!("..\\..\\inputs\\day10.txt");

pub fn solve_part_1() {
    let lines: Vec<&str> = INPUT.lines().into_iter().collect();

    let results: Vec<ValidationResult> = lines.iter().map(|l| validate_line(l)).collect();
    let mut error_score: usize = 0;

    for r in results {
        match r {
            ValidationResult::OK => {}
            ValidationResult::INCOMPLETE(chunks) => {}
            ValidationResult::CORRUPT(chunk, c) => {
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

#[derive(Debug)]
enum Chunk {
    PAREN,
    SQUARE,
    CURLY,
    ANGLE,
}

#[derive(Debug)]
enum ValidationResult {
    OK,
    INCOMPLETE(Vec<Chunk>),
    CORRUPT(Chunk, char),
}

fn validate_line(line: &str) -> ValidationResult {
    let mut chunk_stack: Vec<Chunk> = Vec::new();

    for x in line.chars() {
        match x {
            '(' => {
                chunk_stack.push(Chunk::PAREN);
            }
            '[' => {
                chunk_stack.push(Chunk::SQUARE);
            }
            '{' => {
                chunk_stack.push(Chunk::CURLY);
            }
            '<' => {
                chunk_stack.push(Chunk::ANGLE);
            }

            ')' => match chunk_stack.pop() {
                Some(chunk) => match chunk {
                    Chunk::ANGLE => {
                        return ValidationResult::CORRUPT(chunk, x);
                    }
                    Chunk::SQUARE => {
                        return ValidationResult::CORRUPT(chunk, x);
                    }
                    Chunk::CURLY => {
                        return ValidationResult::CORRUPT(chunk, x);
                    }
                    _ => {}
                },
                None => return ValidationResult::INCOMPLETE(chunk_stack),
            },
            ']' => match chunk_stack.pop() {
                Some(chunk) => match chunk {
                    Chunk::ANGLE => {
                        return ValidationResult::CORRUPT(chunk, x);
                    }
                    Chunk::PAREN => {
                        return ValidationResult::CORRUPT(chunk, x);
                    }
                    Chunk::CURLY => {
                        return ValidationResult::CORRUPT(chunk, x);
                    }
                    _ => {}
                },
                None => return ValidationResult::INCOMPLETE(chunk_stack),
            },
            '}' => match chunk_stack.pop() {
                Some(chunk) => match chunk {
                    Chunk::ANGLE => {
                        return ValidationResult::CORRUPT(chunk, x);
                    }
                    Chunk::SQUARE => {
                        return ValidationResult::CORRUPT(chunk, x);
                    }
                    Chunk::PAREN => {
                        return ValidationResult::CORRUPT(chunk, x);
                    }
                    _ => {}
                },
                None => return ValidationResult::INCOMPLETE(chunk_stack),
            },
            '>' => match chunk_stack.pop() {
                Some(chunk) => match chunk {
                    Chunk::PAREN => {
                        return ValidationResult::CORRUPT(chunk, x);
                    }
                    Chunk::SQUARE => {
                        return ValidationResult::CORRUPT(chunk, x);
                    }
                    Chunk::CURLY => {
                        return ValidationResult::CORRUPT(chunk, x);
                    }
                    _ => {}
                },
                None => return ValidationResult::INCOMPLETE(chunk_stack),
            },
            _ => unreachable!(),
        }
    }

    if chunk_stack.len() > 0 {
        ValidationResult::INCOMPLETE(chunk_stack)
    } else {
        ValidationResult::OK
    }
}

pub fn solve_part_2() {
    let lines: Vec<&str> = INPUT.lines().into_iter().collect();

    let results: Vec<ValidationResult> = lines.iter().map(|l| validate_line(l)).collect();
    let mut autocomplete_scores: Vec<usize> = Vec::new();

    for r in results {
        match r {
            ValidationResult::OK => {}
            ValidationResult::CORRUPT(chunk, c) => {}

            ValidationResult::INCOMPLETE(mut chunks) => {
                let mut cur_score = 0;
                while chunks.len() > 0 {
                    cur_score *= 5;

                    let cur_chunk = chunks.pop().unwrap();
                    match cur_chunk {
                        Chunk::ANGLE => cur_score += 4,
                        Chunk::CURLY => cur_score += 3,
                        Chunk::PAREN => cur_score += 1,
                        Chunk::SQUARE => cur_score += 2,
                    }
                }
                autocomplete_scores.push(cur_score);
            }
        }
    }

    autocomplete_scores.sort();
    let ans: usize = autocomplete_scores[autocomplete_scores.len() / 2];
    println!("Day #10 Part 2: {} ", ans);
}
