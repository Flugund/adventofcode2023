/**
 * --- Day 3: Gear Ratios ---
 * You and the Elf eventually reach a gondola lift station; he says the gondola lift will take you up to the water source, but this is as far as he can bring you. You go inside.
 *
 * It doesn't take long to find the gondolas, but there seems to be a problem: they're not moving.
 *
 * "Aaah!"
 *
 * You turn around to see a slightly-greasy Elf with a wrench and a look of surprise. "Sorry, I wasn't expecting anyone! The gondola lift isn't working right now; it'll still be a while before I can fix it." You offer to help.
 *
 * The engineer explains that an engine part seems to be missing from the engine, but nobody can figure out which one. If you can add up all the part numbers in the engine schematic, it should be easy to work out which part is missing.
 *
 * The engine schematic (your puzzle input) consists of a visual representation of the engine. There are lots of numbers and symbols you don't really understand, but apparently any number adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum. (Periods (.) do not count as a symbol.)
 *
 * Here is an example engine schematic:
 *
 * 467..114..
 * ...*......
 * ..35..633.
 * ......#...
 * 617*......
 * .....+.58.
 * ..592.....
 * ......755.
 * ...$.*....
 * .664.598..
 * In this schematic, two numbers are not part numbers because they are not adjacent to a symbol: 114 (top right) and 58 (middle right). Every other number is adjacent to a symbol and so is a part number; their sum is 4361.
 *
 * Of course, the actual engine schematic is much larger. What is the sum of all of the part numbers in the engine schematic?
 */
use std::fs;

fn main() {
    let file_path = "./data/input.txt";
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file"); // Add the full input data here

    println!("Input: {}", input);

    let sum = sum_parts(input);

    println!("Sum: {}", sum);
}

fn sum_parts(input: String) -> i32 {
    let schematic: Vec<_> = input
        .lines() // Split by lines
        .map(|line| line.trim()) // Trim whitespace from each line
        .collect();

    let sum = sum_part_numbers(&schematic);

    sum
}

fn sum_part_numbers(schematic: &[&str]) -> i32 {
    let mut total_sum = 0;

    let rows = schematic.len();
    let cols = schematic[0].len();

    let mut y = 0;
    while y < rows {
        let mut x = 0;
        while x < cols {
            let current_char = schematic[y].as_bytes()[x] as char;
            if current_char.is_digit(10) {
                // Extract the full number
                let mut num_str = String::new();
                let original_x = x;
                while x < cols && (schematic[y].as_bytes()[x] as char).is_digit(10) {
                    num_str.push(schematic[y].as_bytes()[x] as char);
                    x += 1;
                }

                if is_adjacent_to_symbol(&schematic, y, original_x, x - 1) {
                    total_sum += num_str.parse::<i32>().unwrap_or(0);
                }
            } else {
                x += 1;
            }
        }
        y += 1;
    }

    total_sum
}

fn is_adjacent_to_symbol(schematic: &[&str], y: usize, x_start: usize, x_end: usize) -> bool {
    let y_start = (y as i32 - 1).max(0) as usize
    let y_end = (y as i32 + 1).min(schematic.len() as i32 - 1) as usize

    for ny in y_start..=y_end
    {
        for nx in (x_start as i32 - 1).max(0) as usize
            ..=(x_end as i32 + 1).min(schematic[0].len() as i32 - 1) as usize
        {
            let coordinate_value = schematic[ny].as_bytes()[nx] as char;
            if coordinate_value != '.' && !coordinate_value.is_numeric() {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::sum_parts;

    #[test]
    fn it_works() {
        let input = "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..";

        let result = sum_parts(input.to_string());

        assert_eq!(result, 4361);
    }
}
