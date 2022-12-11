use std::cmp::max;
use std::env;
use std::fs;
use std::num::ParseIntError;

fn main() {
    let args: Vec<String> = env::args().collect();

    let calorie_numbers = parse_numbers(get_file_data(&args[1]));

    let largest_amount_of_calories = most_calories(&calorie_numbers);

    let total_top_three_calories = top_3(calorie_numbers);

    println!(
        "The largest number of calories: {}",
        largest_amount_of_calories
    );
    println!(
        "The combined calories of the top three: {}",
        total_top_three_calories
    );
}

fn get_file_data(filename: &str) -> String {
    fs::read_to_string(filename).unwrap()
}

fn parse_numbers(numbers_string: String) -> Vec<i32> {
    numbers_string
        .split("\n\n")
        .map(|string_section| {
            string_section
                .split("\n")
                .map(|number_string| number_string.parse())
                .fold(0, |total, number_result: Result<i32, ParseIntError>| {
                    if let Ok(number) = number_result {
                        total + number
                    } else {
                        total
                    }
                })
        })
        .collect()
}

fn most_calories(calorie_numbers: &Vec<i32>) -> i32 {
    calorie_numbers
        .iter()
        .fold(0, |number, largest_so_far| max(number, *largest_so_far))
}

fn top_3(mut calorie_numbers: Vec<i32>) -> i32 {
    let mut the_largest_three = [0; 3];

    for i in 0..3 {
        the_largest_three[i] = most_calories(&calorie_numbers);

        calorie_numbers = calorie_numbers
            .into_iter()
            .filter(|&number| number != the_largest_three[i])
            .collect();
    }

    the_largest_three
        .iter()
        .fold(0, |number, total| number + total)
}
