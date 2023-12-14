use std::fs;

fn get_points_in_line(line: &str) -> i32 {
    let start_idx = line.chars().position(|c| c == ':').unwrap();
    let separator_idx = line.chars().position(|c| c == '|').unwrap();
    let winning_numbers: Vec<i32> = line[start_idx + 1..separator_idx]
        .split_whitespace()
        .map(|num| num.parse::<i32>().unwrap())
        .collect();

    let mut score = 1;

    line[separator_idx + 1..]
        .split_whitespace()
        .for_each(|num| {
            let num = num.parse::<i32>().unwrap();
            if winning_numbers.contains(&num) {
                score *= 2;
            }
        });

    score / 2
}

fn main() {
    let input = fs::read_to_string("../input.txt").unwrap();

    let mut sum = 0;
    for line in input.lines() {
        sum += get_points_in_line(line);
    }

    println!("Sum: {}", sum);
}