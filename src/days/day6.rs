const INPUT: &str = include_str!("..\\..\\inputs\\day6.txt");

pub fn solve_part_1() {
    let mut fish: Vec<u8> = INPUT.split(",").map(|s| s.parse().unwrap()).collect();

    for day in 0..80 {
        let mut new_fish_count = 0;
        for f in fish.iter_mut() {
            match *f {
                0 => {
                    *f = 6;
                    new_fish_count += 1;
                }
                _ => {
                    *f -=1;
                }
            }
        }

        for n in 0..new_fish_count{
            fish.push(8);
        }

    }

    println!("{}", fish.len());
}

pub fn solve_part_2() {}
