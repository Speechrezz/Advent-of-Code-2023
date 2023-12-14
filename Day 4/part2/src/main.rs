use std::fs;

fn get_win_count(line: &str) -> i32 {
    let start_idx = line.chars().position(|c| c == ':').unwrap();
    let separator_idx = line.chars().position(|c| c == '|').unwrap();
    let winning_numbers: Vec<i32> = line[start_idx + 1..separator_idx]
        .split_whitespace()
        .map(|num| num.parse::<i32>().unwrap())
        .collect();

    let mut sum = 0;

    line[separator_idx + 1..]
        .split_whitespace()
        .for_each(|num| {
            let num = num.parse::<i32>().unwrap();
            if winning_numbers.contains(&num) {
                sum += 1;
            }
        });

    sum
}

fn main() {
    let input = fs::read_to_string("../input.txt").unwrap();
    let mut card_counts: Vec<i32> = Vec::new();
    card_counts.resize(input.lines().count(), 1);

    for (i, line) in input.lines().enumerate() {
        let win_count = get_win_count(line);
        let end_range = (i + 1 + win_count as usize).min(card_counts.len());
        let current_count = card_counts[i];
        for count in card_counts[i + 1..end_range].iter_mut() {
            *count += current_count;
        }
    }

    println!("Sum: {}", card_counts.iter().sum::<i32>());
}
