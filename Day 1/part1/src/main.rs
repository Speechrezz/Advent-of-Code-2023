use std::fs;

fn main() {
    let contents = fs::read_to_string("../input.txt").unwrap();

    let mut vec: Vec<char> = Vec::new();
    let mut sum: i32 = 0;

    for line in contents.lines() {
        vec.clear();
        for char in line.chars() {
            if char >= '0' && char <='9' {
                vec.push(char);
            }
        }

        sum += format!("{}{}", vec[0], vec[vec.len() - 1]).parse::<i32>().unwrap();
    }

    println!("Sum: {}", sum);
}