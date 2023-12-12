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

    fn can_fit(&self, other: &Cubes) -> bool {
        return other.red <= self.red 
            && other.green <= self.green
            && other.blue <= self.blue;
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

fn get_current_game_number(word_iter: &mut SplitWhitespace) -> i32 {
    let mut game_iter = word_iter.skip_while(|&word| word != "Game");
    game_iter.next();
    
    if let Some(number_str) = game_iter.next() {
        let number_str = &number_str[..number_str.len() - 1];
        return number_str.parse::<i32>().unwrap_or(0);
    }

    return 0;
}


fn main() {
    let bag = Cubes {
        red: 12,
        green: 13,
        blue: 14
    };

    let mut sum = 0;

    let input = fs::read_to_string("../input.txt").unwrap();

    for line in input.lines() {
        let mut word_iter = line.split_whitespace();

        let game_number = get_current_game_number(&mut word_iter);
        let cubes = get_current_cubes(&mut word_iter);

        if bag.can_fit(&cubes) {
            sum += game_number;
        }
        //println!("game: {}, cubes: {:?}", game_number, cubes);
    }

    println!("Sum: {}", sum);
}
