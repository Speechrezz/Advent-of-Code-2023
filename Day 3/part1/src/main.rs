use std::fs;
use std::ops::Range;

#[derive(Debug)]
struct NumberPosition {
    row: i32,
    cols: Range<i32>,
    value: i32
}

struct NumberFinder {
    positions: Vec<NumberPosition>,
    start_index: Option<usize>,
    current_number: String
}

impl NumberFinder {
    fn new() -> NumberFinder {
        NumberFinder{
            positions: Vec::new(),
            start_index: None,
            current_number: String::new()
        }
    }

    fn from(input: &str) -> NumberFinder {
        let mut number_finder = NumberFinder::new();
        number_finder.find(&input);

        number_finder
    }

    fn find(&mut self, input: &str) {
        for (line_index, line) in input.lines().enumerate() {
            for (char_index, char) in line.chars().enumerate() {
                self.process_char(&line_index, &char_index, &char)
            }

            self.end_number(&line_index, &line.len());
        }
    }

    fn process_char(&mut self, line_index: &usize, char_index: &usize, char: &char) {
        if char.is_digit(10) {
            self.current_number.push(*char);
            if self.start_index == None {
                self.start_index = Some(*char_index);
            }
        }
        else {
            self.end_number(&line_index, &char_index);
        }
    }

    fn end_number(&mut self, row: &usize, col: &usize) {
        if let Some(start_index) = self.start_index {
            self.positions.push(NumberPosition {
                row: *row as i32,
                cols: start_index as i32..*col as i32,
                value: self.current_number.parse::<i32>().unwrap_or(0)
            });

            self.start_index = None;
            self.current_number.clear();
        }
    }
}

fn create_limited_range(start: i32, end: i32, max_value: i32) -> Range<usize> {
    start.max(0) as usize..end.min(max_value) as usize
}

fn is_part_number(input: &str, number_position: &NumberPosition) -> bool {
    let lines: Vec<&str> = input.lines().collect();

    let row = &number_position.row;
    let row_range = create_limited_range(row - 1, row + 2, lines.len() as i32);
    let cols = &number_position.cols;
    let col_range = create_limited_range(cols.start - 1, cols.end + 1, lines[0].len() as i32);

    for line in &lines[row_range] {
        for char in line[col_range.clone()].chars() {
            if !char.is_digit(10) && char != '.' {
                return true;
            }
        }
    }

    return false;
}

fn main() {
    let input = fs::read_to_string("../input.txt").unwrap();

    // Find numbers and all their positions
    let number_positions = NumberFinder::from(&input).positions;

    // Sum all numbers near symbols
    let mut sum: i32 = 0;
    for number_position in number_positions {
        if is_part_number(&input, &number_position) {
            sum += number_position.value;
        }
    }

    println!("Sum: {}", sum);
}
