use std::fs::read_to_string;

pub fn solve(path : &str) {
    let file = read_to_string(path).unwrap();
    let levels : Vec<Vec<i32>> = file.lines().map(|line| line.split(" ").map(|el| el.parse::<i32>().unwrap()).collect()).collect();

    part1(&levels);
    part2(&levels);

}

fn part1(levels : &Vec<Vec<i32>>) {
    let count = levels.iter().filter(|level| (is_all_increasing(level) || is_all_decreasing(level)) && is_within_range(level)).count();
    println!("{}", count);
}

fn part2(levels : &Vec<Vec<i32>>) {
    let count : i32 = levels.iter().map(|level| {
        if (is_all_increasing(level) || is_all_decreasing(level)) && is_within_range(level) {
            return 1;
        }

        for i in 0..level.len() {
            let mut copy = level.clone();
            copy.remove(i);

            if (is_all_increasing(&copy) || is_all_decreasing(&copy)) && is_within_range(&copy) {
                return 1;
            }
        }

        return 0;
    }).sum();

    println!("{}", count);
}

fn is_all_increasing(level : &Vec<i32>) -> bool {
    level.windows(2).all(|w| w[0] < w[1])
}

fn is_all_decreasing(level : &Vec<i32>) -> bool {
    level.windows(2).all(|w| w[0] > w[1])
}

fn is_within_range(level : &Vec<i32>) -> bool {
    level.windows(2).all(|w| i32::abs(w[0] - w[1]) >= 1 && i32::abs(w[0] - w[1]) <= 3)
}
