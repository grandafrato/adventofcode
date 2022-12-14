use std::env;
use std::fs;

use crate::rucksack::Rucksack;

mod rucksack;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_data = get_file_data(&args[1]);

    let rucksacks = split_into_rucksacks(file_data);

    let total_value_per_rucksack = value_per_rucksack(&rucksacks);
    let total_value_per_group = value_per_group(rucksacks);

    println!(
        "The total value of the common items in one rucksack is: {}",
        total_value_per_rucksack
    );

    println!(
        "The total value of the common items in every group is: {}",
        total_value_per_group
    );
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

fn value_per_rucksack(rucksacks: &Vec<Rucksack>) -> u32 {
    rucksacks.iter().fold(0, |total, rucksack| {
        rucksack.item_in_both_compartments().value() + total
    })
}

fn value_per_group(rucksacks: Vec<Rucksack>) -> u32 {
    rucksacks
        .chunks_exact(3)
        .map(|group| group[0].item_shared(&[&group[1], &group[2]]).value())
        .reduce(|total, val| total + val)
        .unwrap()
}
