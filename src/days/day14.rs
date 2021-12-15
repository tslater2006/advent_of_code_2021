use std::collections::HashMap;

const INPUT: &str = include_str!("..\\..\\inputs\\day14.txt");

pub fn solve_part_1() {
    let ans = build_polymers(10);

    println!("Day #14 Part 1: {}", ans);
}

pub fn solve_part_2() {
    let ans = build_polymers(40);

    println!("Day #14 Part 2: {:?}", ans);
}

fn parse_input() -> (Vec<u8>, HashMap<(u8, u8), u8>) {
    let mut lines = INPUT.lines();
    let start = String::from(lines.next().unwrap()).into_bytes();
    lines.next();

    let mut rule_map: HashMap<(u8, u8), u8> = HashMap::new();
    loop {
        match lines.next() {
            Some(l) => {
                let mut parts = l.split(" -> ");
                let key_bytes = parts.next().unwrap().as_bytes();
                let key = (key_bytes[0], key_bytes[1]);
                rule_map.insert(key, parts.next().unwrap().as_bytes()[0]);
            }

            None => {
                break;
            }
        }
    }

    (start, rule_map)
}

fn build_polymers(count: usize) -> u64 {
    let (state, rules) = parse_input();

    let mut pairs_count: HashMap<(u8, u8), u64> = HashMap::new();
    let start_letter = state[0] - b'A';
    let end_letter = state[state.len() - 1] - b'A';

    for n in state.windows(2) {
        pairs_count.insert((n[0], n[1]), 1);
    }

    for _ in 0..count {
        let mut new_pairs_count: HashMap<(u8, u8), u64> = HashMap::new();

        for k in pairs_count.keys() {
            match rules.get(k) {
                Some(v) => {
                    let pair1 = (k.0, *v);
                    let pair2 = (*v, k.1);
                    match new_pairs_count.get_mut(&pair1) {
                        Some(v) => {
                            *v += pairs_count.get(k).unwrap();
                        }
                        None => {
                            new_pairs_count.insert(pair1, *pairs_count.get(k).unwrap());
                        }
                    }

                    match new_pairs_count.get_mut(&pair2) {
                        Some(v) => {
                            *v += pairs_count.get(k).unwrap();
                        }
                        None => {
                            new_pairs_count.insert(pair2, *pairs_count.get(k).unwrap());
                        }
                    }
                }
                None => match new_pairs_count.get_mut(k) {
                    Some(v) => {
                        *v += pairs_count.get(k).unwrap();
                    }
                    None => {
                        new_pairs_count.insert(*k, *pairs_count.get(k).unwrap());
                    }
                },
            }
        }
        pairs_count.clear();
        pairs_count = new_pairs_count;
    }

    let mut letters_count = [0u64; 26];
    letters_count[start_letter as usize] = 1;
    letters_count[end_letter as usize] = 1;

    for k in pairs_count {
        letters_count[(k.0 .0 - b'A') as usize] += k.1;
        letters_count[(k.0 .1 - b'A') as usize] += k.1;
    }

    let letter_max = letters_count.iter().max().unwrap() / 2;

    let letter_min = letters_count.iter().filter(|a| **a > 0).min().unwrap() / 2;

    letter_max - letter_min
}
