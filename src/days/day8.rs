const INPUT: &str = include_str!("..\\..\\inputs\\day8.txt");

pub fn solve_part_1() {
    let data: Vec<Vec<&str>> = INPUT
        .lines()
        .into_iter()
        .map(|l| {
            let mut sp = l.split('|').map(|s| s.trim());
            let out: Vec<&str> = sp.nth(1).unwrap().split(' ').collect();

            out
        })
        .collect();

    let ans: usize = data
        .iter()
        .map(|i| i.iter().filter(|f| 
            // 2 4 3 7
            matches!(f.len(), 2 | 4 | 3 | 7)
        ).count())
        .sum();

    println!("Day #8 Part 1: {}", ans);
}

pub fn solve_part_2() {
    

    let data: Vec<(Vec<&str>, Vec<&str>)> = INPUT
        .lines()
        .into_iter()
        .map(|l| {
            let mut sp = l.split('|').map(|s| s.trim());
            let obs: Vec<&str> = sp.next().unwrap().split(' ').collect();
            let out: Vec<&str> = sp.next().unwrap().split(' ').collect();

            (obs, out)
        })
        .collect();

    let ans: usize = data.iter().map(|d| {
        resolve_line(&d.0, &d.1)
    }).sum();

    println!("Day #8 Part 2: {}", ans);

}

fn resolve_line(observation: &[&str], outputs: &[&str]) -> usize {

    let mut numbers = ["";10];

    numbers[1] = observation.iter().find(|f| f.len() == 2).unwrap();
    
    numbers[4] = observation.iter().find(|f| f.len() == 4).unwrap();
    numbers[7] = observation.iter().find(|f| f.len() == 3).unwrap();
    numbers[8] = observation.iter().find(|f| f.len() == 7).unwrap();

    numbers[6] = observation.iter().find(|i| {
        i.len() == 6 && !all_of_target_in_source(i, numbers[1])
    }).unwrap();

    numbers[9] = observation.iter().find(|i| {
        i.len() == 6 && all_of_target_in_source(i, numbers[4])
    }).unwrap();

    numbers[3] = observation.iter().find(|i| {
        i.len() == 5 && all_of_target_in_source(i, numbers[1])
    }).unwrap();

    numbers[0] = observation.iter().find(|i| {
        i.len() == 6 && all_of_target_in_source(i, numbers[1]) && **i != numbers[9]
    }).unwrap();

    numbers[5] = observation.iter().find(|i| {
        i.len() == 5 && partial_overlap_count(i, numbers[6]) == 5
    }).unwrap();

    numbers[2] = observation.iter().find(|i| {
        i.len() == 5 && **i != numbers[3] && partial_overlap_count(i, numbers[6]) == 4
    }).unwrap();

    let mut displayed_value = 0;
    for item in outputs {

        displayed_value *= 10;

        for (y,n) in numbers.iter().enumerate() {
            
            if complete_overlap(item, n) {
                displayed_value += y;
                break;
            }
        }
    }
    displayed_value
}

/* returns true if source contains all of the letters of target, position doesn't matter */
fn all_of_target_in_source(source:&str, target:&str) -> bool {

    for l in target.chars() {
        if !source.contains(l) {
            return false;
        }
    }

    true
}

fn complete_overlap (source:&str, target:&str) -> bool {
    if source.len() == target.len() {
        all_of_target_in_source(source, target)
    } else {
        false
    }
}

fn partial_overlap_count(source:&str, target:&str) -> usize {
    target.chars().map(|c| if source.contains(c) { 1} else {0}).sum()
}