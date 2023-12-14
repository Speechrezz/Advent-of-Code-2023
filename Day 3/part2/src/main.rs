use std::fs;
use std::ops::Range;

#[derive(Debug)]
struct NumberBounds {
    rows: Range<usize>,
    cols: Range<usize>,
    value: i32
}

impl NumberBounds {
    fn contains(&self, row: usize, col: usize) -> bool {
        self.rows.contains(&row) && self.cols.contains(&col)
    }
}

#[derive(Debug)]
struct NumberFinder {
    positions: Vec<NumberBounds>,
    start_index: Option<usize>,
    current_number: String,
    row_count: i32,
    col_count: i32
}

impl NumberFinder {
    fn new(input: &str) -> NumberFinder {
        let row_count = input.lines().count();
        let col_count = input.lines().next().unwrap_or("").len();

        NumberFinder {
            positions: Vec::new(),
            start_index: None,
            current_number: String::new(),
            row_count: row_count as i32,
            col_count: col_count as i32
        }
    }

    fn find(input: &str) -> Vec<NumberBounds> {
        let mut number_finder = NumberFinder::new(input);

        for (line_index, line) in input.lines().enumerate() {
            for (char_index, char) in line.chars().enumerate() {
                number_finder.process_char(&line_index, &char_index, &char)
            }

            number_finder.end_number(&line_index, &line.len());
        }

        number_finder.positions
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
        if let Some(start_col) = self.start_index {
            self.positions.push(self.new_number_bounds(
                row, 
                &(start_col..*col), 
                self.current_number.parse::<i32>().unwrap_or(0))
            );

            self.start_index = None;
            self.current_number.clear();
        }
    }

    fn new_number_bounds(&self, row: &usize, col_range: &Range<usize>, value: i32) -> NumberBounds {
        NumberBounds{
            rows: create_limited_range(*row as i32 - 1, *row as i32 + 2, self.row_count),
            cols: create_limited_range(col_range.start as i32 - 1, col_range.end as i32 + 1, self.col_count),
            value
        }
    }
}

fn create_limited_range(start: i32, end: i32, max_value: i32) -> Range<usize> {
    start.max(0) as usize..end.min(max_value) as usize
}

fn find_gear_number(row: usize, col: usize, number_bounds: &Vec<NumberBounds>) ->Option<i32> {
    let mut numbers: Vec<i32> = Vec::new();
    for bounds in number_bounds.iter() {
        if bounds.contains(row, col) {
            numbers.push(bounds.value);
        }
    }

    if numbers.len() == 2 {
        return Some(numbers[0] * numbers[1]);
    }

    None
}

fn main() {
    let input = fs::read_to_string("../input.txt").unwrap();

    // Find numbers and all their positions
    let number_bounds = NumberFinder::find(&input);

    // Find gear part numbers
    let mut sum: i32 = 0;
    for (row, line) in input.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            if char == '*' {
                if let Some(number) = find_gear_number(row, col, &number_bounds) {
                    sum += number;
                }
            }
        }
    }

    println!("Sum: {}", sum);
}
