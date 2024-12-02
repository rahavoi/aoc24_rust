use std::fs::read_to_string;

pub fn solve(path : &str) {
    let file = read_to_string(path).unwrap();
    let elements : Vec<Vec<&str>> = file.lines().map(|line| line.split("   ").collect()).collect();

    let mut left : Vec<i32> = elements.iter().map(|parts| parts.get(0).unwrap().parse::<i32>().unwrap()).collect();
    let mut right : Vec<i32> = elements.iter().map(|parts| parts.get(1).unwrap().parse::<i32>().unwrap()).collect();
    left.sort();
    right.sort();

    part1(&left, &right);
    part2(&left, &right);
}

fn part1(left : &Vec<i32>, right : &Vec<i32>) {
    let mut sum = 0;

    for(i, el) in left.iter().enumerate() {
        let diff = el - right.get(i).unwrap();
        sum = sum + i32::abs(diff);
    }

    println!("{:?}", sum)
}

fn part2(left : &Vec<i32>, right : &Vec<i32>) {
    let mut sum = 0;

    for l in left {
        sum += right.iter().filter(|r| *r == l).count() as i32 * l;
    }

    println!("{:?}", sum)
}


