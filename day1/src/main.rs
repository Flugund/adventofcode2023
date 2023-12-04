/**
 * --- Day 1: Trebuchet?! ---
 * 
 * Something is wrong with global snow production, and you've been selected to take a look. The Elves have even given you a map; on it, they've used stars to mark the top fifty locations that are likely to be having problems.
 * 
 * You've been doing this long enough to know that to restore snow operations, you need to check all fifty stars by December 25th.
 * 
 * Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!
 * 
 * You try to ask why they can't just use a weather machine ("not powerful enough") and where they're even sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of questions") and hang on did you just say the sky ("of course, where do you think snow comes from") when you realize that the Elves are already loading you into a trebuchet ("please hold still, we need to strap you in").
 * 
 * As they're making the final adjustments, they discover that their calibration document (your puzzle input) has been amended by a very young Elf who was apparently just excited to show off her art skills. Consequently, the Elves are having trouble reading the values on the document.
 * 
 * The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need to recover. On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.
 * 
 * For example:
 * 
 * 1abc2
 * pqr3stu8vwx
 * a1b2c3d4e5f
 * treb7uchet
 * 
 * In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.
 * 
 * Consider your entire calibration document. What is the sum of all of the calibration values?
 * 
 */
use std::fs;

fn main() {
    println!("Day1!");

    let file_path = String::from("./data/input.txt");

    let lines = load_input(file_path);

    println!("Size input {}", lines.len());

    let parsed_lines: Vec<String> = parse_lines(lines);
    
    println!("Size parsed input {}", parsed_lines.len());


    println!("Done, {}!", calculate_sum(parsed_lines));
}

fn load_input(file_path: String) -> Vec<String> {
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    return contents.split_whitespace()
        .map(|s| s.to_string()) 
        .collect::<Vec<String>>();
}

fn parse_lines(lines: Vec<String>) -> Vec<String> {
    let mut parsed_lines: Vec<String> = Vec::new();

    for mut s in lines {
        s = s.replace("one", "1");
        s = s.replace("two", "2");
        s = s.replace("three", "3");
        s = s.replace("four", "4");
        s = s.replace("five", "5");
        s = s.replace("six", "6");
        s = s.replace("seven", "7");
        s = s.replace("eight", "8");
        s = s.replace("nine", "9");

        parsed_lines.push(s);
    }

    parsed_lines
}

fn calculate_sum(lines: Vec<String>) -> i32 {
    let mut sum = 0;

    const RADIX: u32 = 10;

    for s in lines {
        println!("String: {}", s);

        let mut left: usize = 0;
        let mut right: usize = s.len() - 1;

        let mut ch_start_found = -1;
        let mut ch_end_found = -1;

        while left < s.len() {
            let ch_left = s.chars().nth(left).unwrap();
            let ch_right = s.chars().nth(right).unwrap();

            if ch_left.is_numeric() && ch_start_found == -1 {
                ch_start_found = ch_left.to_digit(RADIX).unwrap() as i32;
            }

            if ch_right.is_numeric() && ch_end_found == -1 {
                ch_end_found = ch_right.to_digit(RADIX).unwrap() as i32;
            }

            if (ch_end_found != -1 && ch_start_found != -1) {
                break;
            }

            left += 1;
            right -= 1;
        }

        let value = format!("{}{}", ch_start_found, ch_end_found);

        println!("Found value: {}", value);

        sum += value.parse::<i32>().unwrap();
    }

    sum
}