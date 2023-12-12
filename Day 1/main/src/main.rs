use std::fs;
use std::cmp;
use std::char;

fn get_word_digit(char_index: usize, line: &str) -> Option<char> {
    let number_list = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    for (i, number) in number_list.iter().enumerate() {
        let end_index = cmp::min( char_index + number.len(), line.len());
        let word = &line[char_index..end_index];
        if word.to_string() == number.to_string() {
            if let Some(digit) = char::from_digit((i + 1) as u32, 10) {
                return Some(digit);
            }
        }
    }

    return None;
}

fn main() {
    let contents = fs::read_to_string("../input.txt").unwrap();

    let mut vec: Vec<char> = Vec::new();
    let mut sum: i32 = 0;

    for line in contents.lines() {
        vec.clear();
        for (i, char) in line.chars().enumerate() {
            if char.is_digit(10) {
                vec.push(char);
            }
            else if let Some(digit) = get_word_digit(i, line) {
                vec.push(digit);
            }
        }

        sum += format!("{}{}", vec[0], vec[vec.len() - 1]).parse::<i32>().unwrap();
    }

    println!("Sum: {}", sum);
}