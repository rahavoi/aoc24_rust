use std::fs::read_to_string;

use regex::Regex;

pub fn solve(path : &str) {
    let file = read_to_string(path).unwrap();
    let lines = file.lines();
    let re = Regex::new("(mul\\(\\d+,\\d+\\))|do\\(\\)|don't\\(\\)").unwrap();


    let mut enabled = true;
    let mut result = 0;

    for line in lines {
        let matches : Vec<&str> = re.find_iter(line).map(|m| m.as_str()).collect();

        for m in matches {
            if m == "do()" {
                enabled = true;
            } else if m == "don't()" {
                enabled = false;
            } else if enabled {
                let nums = m.replace("mul(", "").replace(")", "");
                let mut parts = nums.split(",");

                result += parts.next().unwrap().parse::<i32>().unwrap() * parts.next().unwrap().parse::<i32>().unwrap();
            }
        }
    }

    println!("{:?}", result);
}