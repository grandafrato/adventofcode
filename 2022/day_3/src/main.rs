use std::env;
use std::fs;

use crate::rucksack::Rucksack;

mod rucksack;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_data = get_file_data(&args[1]);

    let rucksacks = split_into_rucksacks(file_data);
    let total_value = value_of_common_items(rucksacks);

    println!("The total value of the common items is: {}", total_value);
}

fn get_file_data(filename: &str) -> String {
    fs::read_to_string(filename).unwrap()
}

fn split_into_rucksacks(file_data: String) -> Vec<Rucksack> {
    file_data
        .split_whitespace()
        .map(|rucksack_string| Rucksack::new(rucksack_string))
        .collect()
}

fn value_of_common_items(rucksacks: Vec<Rucksack>) -> u32 {
    rucksacks.iter().fold(0, |total, rucksack| {
        rucksack.item_in_both_compartments().value() + total
    })
}
