use crate::file_utils::read_file_line_by_line;

pub fn part_one() -> i32 {
    let mut calculated_sum = 0;
    let lines = read_file_line_by_line("inputs\\day1").unwrap();
    for line in lines {
        let mut first_digit = -1;
        let mut last_digit = -1;
        for char in line.chars() {
            if char.is_numeric() {
                if first_digit == -1 {
                    first_digit = char.to_digit(10).unwrap() as i32;
                } else {
                    last_digit = char.to_digit(10).unwrap() as i32;
                }
            }
        }
        if last_digit == -1 {
            last_digit = first_digit;
        }
        let number = (first_digit * 10) + last_digit;
        calculated_sum += number;
    }
    calculated_sum
}


pub fn part_two() -> i32 {
    let mut calculated_sum = 0;
    let lines = read_file_line_by_line("inputs\\day1").unwrap();
    let mut digit_count = std::collections::HashMap::new();
    digit_count.insert("one", 1);
    digit_count.insert("two", 2);
    digit_count.insert("three", 3);
    digit_count.insert("four", 4);
    digit_count.insert("five", 5);
    digit_count.insert("six", 6);
    digit_count.insert("seven", 7);
    digit_count.insert("eight", 8);
    digit_count.insert("nine", 9);
    
    digit_count.insert("1", 1);
    digit_count.insert("2", 2);
    digit_count.insert("3", 3);
    digit_count.insert("4", 4);
    digit_count.insert("5", 5);
    digit_count.insert("6", 6);
    digit_count.insert("7", 7);
    digit_count.insert("8", 8);
    digit_count.insert("9", 9);
    
    for line in lines {
        let mut first_digit = -1;
        let mut last_digit = -1;
        for start_index in 0..line.len()+1{
            for end_index in start_index..line.len()+1{
                let current_substring = line[start_index..end_index].to_string();
                if digit_count.contains_key(current_substring.as_str()) {
                    if first_digit == -1 {
                        first_digit = digit_count.get(current_substring.as_str()).unwrap().clone();
                    } else {
                        last_digit = digit_count.get(current_substring.as_str()).unwrap().clone();
                    }
                }
            }
        }

        if last_digit == -1 {
            last_digit = first_digit;
        };
        let number = (first_digit * 10) + last_digit;
        println!("{} : {}", line, number);
        calculated_sum += number;
    }
    calculated_sum
}