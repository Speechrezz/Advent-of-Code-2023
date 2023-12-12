use std::{fs, str::SplitWhitespace};

#[derive(Debug)]
struct Cubes {
    red: i32,
    green: i32,
    blue: i32
}

impl Cubes {
    fn new() -> Cubes {
        Cubes{
            red: 0,
            green: 0,
            blue: 0
        }
    }

    fn get_power(&self) -> i32 {
        self.red * self.green * self.blue
    }
}

fn add_cubes(number_str: &str, word_iter: &mut SplitWhitespace, cubes: &mut Cubes) {
    if let Ok(number) = number_str.parse::<i32>() {
        if let Some(color) = word_iter.next() {
            let color = color.trim_end_matches([',', ';']);
            match color {
                "red"   => cubes.red   = number.max(cubes.red),
                "green" => cubes.green = number.max(cubes.green),
                "blue"  => cubes.blue  = number.max(cubes.blue),
                _ => ()
            }
        }
    }
}

fn get_current_cubes(word_iter: &mut SplitWhitespace) -> Cubes {
    let mut cubes = Cubes::new();

    loop {
        match word_iter.next() {
            Some(number_str) => {
                add_cubes(number_str, word_iter, &mut cubes)
            },
            None => break
        }
    }

    cubes
}

fn main() {
    let mut sum = 0;
    let input = fs::read_to_string("../input.txt").unwrap();

    for line in input.lines() {
        sum += get_current_cubes(&mut line.split_whitespace()).get_power();
    }

    println!("Sum: {}", sum);
}