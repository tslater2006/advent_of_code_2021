use std::fmt;

const INPUT: &str = include_str!("..\\..\\inputs\\day4.txt");

#[derive(Clone, Copy)]
enum BoardResult {
    Win(u16),
    NoWin,
}

struct BingoBoard {
    board: [[u16; 5]; 5],
    mask: [[u16; 5]; 5],
    result: BoardResult,
}

impl fmt::Display for BingoBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.

        for y in 0..5 {
            for x in 0..5 {
                if self.board[y][x] < 10 {
                    write!(f, " {} ", self.board[y][x]).unwrap();
                } else {
                    write!(f, "{} ", self.board[y][x]).unwrap();
                }
            }
            writeln!(f).unwrap();
        }
        Ok(())
    }
}

impl BingoBoard {
    fn new(lines: &[&str]) -> BingoBoard {
        /* parse the 6 lines passed in */
        assert!(lines.len() == 6);

        let mut new_board = BingoBoard {
            board: [[0u16; 5]; 5],
            mask: [[0u16; 5]; 5],
            result: BoardResult::NoWin,
        };

        /* we need to go through the 5 last lines, parse out the numbers, set board up */

        for (y, line) in lines.iter().enumerate().take(6).skip(1) {
            let line_numbers: Vec<u16> = line
                .split(' ')
                .filter(|l| !l.is_empty())
                .map(|n| n.parse().unwrap())
                .collect();
            new_board.board[y - 1][..5].clone_from_slice(&line_numbers[..5]);
        }

        new_board
    }

    fn get_unused_sum(&mut self) -> u16 {
        let mut sum: u16 = 0;
        for y in 0..5 {
            for x in 0..5 {
                if self.mask[y][x] == 0 {
                    sum += self.board[y][x];
                }
            }
        }

        sum
    }

    fn play_number(&mut self, num: u16) -> BoardResult {
        let mut result = BoardResult::NoWin;

        for y in 0..5 {
            for x in 0..5 {
                if self.board[y][x] == num {
                    self.mask[y][x] = 1;
                }
            }
        }

        for y in 0..5 {
            let row_sum: u16 = self.mask[y].iter().sum();
            if row_sum == 5 {
                result = BoardResult::Win(num);
                break;
            }
        }
        match result {
            BoardResult::NoWin => {
                for x in 0..5 {
                    let col_sum: u16 = self.mask.iter().map(|row| row[x]).sum();
                    if col_sum == 5 {
                        result = BoardResult::Win(num);
                    }
                }
            }
            BoardResult::Win(_) => {}
        }

        /* update ourselves with the result, so we can stop playing games that have won already */
        self.result = result;

        result
    }
}

pub fn solve_part_1() {
    let lines: Vec<&str> = INPUT.lines().collect();
    let board_lines: Vec<&str> = INPUT.lines().into_iter().skip(1).collect();
    let drawn_numbers: Vec<u16> = lines[0].split(',').map(|v| v.parse().unwrap()).collect();

    //let boards: Vec<BingoBoard> = lines.iter().skip(1).

    let mut boards: Vec<BingoBoard> = board_lines.chunks(6).map(|l| BingoBoard::new(l)).collect();

    'num_loop: for num in drawn_numbers {
        for board in boards.iter_mut() {
            let result = board.play_number(num);
            match result {
                BoardResult::Win(winning_num) => {
                    println!("Day #4 Part 1: {}", board.get_unused_sum() * winning_num);
                    break 'num_loop;
                }
                BoardResult::NoWin => {}
            }
        }
    }
}

pub fn solve_part_2() {
    let lines: Vec<&str> = INPUT.lines().collect();
    let board_lines: Vec<&str> = INPUT.lines().into_iter().skip(1).collect();
    let drawn_numbers: Vec<u16> = lines[0].split(',').map(|v| v.parse().unwrap()).collect();

    //let boards: Vec<BingoBoard> = lines.iter().skip(1).

    let mut boards: Vec<BingoBoard> = board_lines.chunks(6).map(|l| BingoBoard::new(l)).collect();
    let board_count = boards.len();
    let mut winning_board_count = 0;
    'num_loop: for num in drawn_numbers {
        for board in boards.iter_mut() {
            //match board.result {
            if let BoardResult::NoWin = board.result {
                let result = board.play_number(num);
                if let BoardResult::Win(winning_num) = result {
                    //BoardResult::Win(winning_num) => {
                    winning_board_count += 1;
                    if winning_board_count == board_count {
                        println!("Day #4 Part 2: {}", board.get_unused_sum() * winning_num);
                        break 'num_loop;
                    }
                    /* }
                    BoardResult::NoWin => {}*/
                }
            }
            /*    _ => {}
            }*/
        }
    }
}
