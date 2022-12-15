use std::env;
use std::fmt;
use std::fs;

fn main() {
    let file_name = env::args().nth(1).unwrap();
    let section_pairs = get_file_data(file_name);

    let pairs_that_contain_each_other = num_pairs_that_contain_each_other(&section_pairs);
    let pairs_that_overlap_each_other = num_pairs_that_overlap_each_other(section_pairs);

    println!(
        "The number of pairs where one range fully contains the other is: {}",
        pairs_that_contain_each_other
    );
    println!(
        "The number of pairs where one range overlaps the other is: {}",
        pairs_that_overlap_each_other
    );
}

type SectionPairs = Vec<(SectionRange, SectionRange)>;

fn get_file_data(file_name: String) -> SectionPairs {
    fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|pair| {
            let (range_1, range_2) = pair.split_once(',').unwrap();
            (
                string_to_section_range(range_1),
                string_to_section_range(range_2),
            )
        })
        .collect()
}

fn string_to_section_range(string: &str) -> SectionRange {
    let (start, end) = string.split_once('-').unwrap();

    SectionRange::new(start.parse().unwrap(), end.parse().unwrap())
}

fn num_pairs_that_contain_each_other(section_pairs: &SectionPairs) -> i32 {
    section_pairs
        .into_iter()
        .fold(0, |total, (range_1, range_2)| {
            if range_1.contains(&range_2) || range_2.contains(&range_1) {
                total + 1
            } else {
                total
            }
        })
}

fn num_pairs_that_overlap_each_other(section_pairs: SectionPairs) -> i32 {
    section_pairs
        .into_iter()
        .fold(0, |total, (range_1, range_2)| {
            if range_1.overlaps(&range_2) {
                total + 1
            } else {
                total
            }
        })
}

#[derive(Debug)]
struct SectionRange {
    start: i32,
    end: i32,
}

impl SectionRange {
    fn new(start: i32, end: i32) -> SectionRange {
        SectionRange { start, end }
    }

    fn contains(&self, other: &SectionRange) -> bool {
        other.start >= self.start && other.end <= self.end
    }

    fn overlaps(&self, other: &SectionRange) -> bool {
        (other.start..=other.end).contains(&self.start)
            || (other.start..=other.end).contains(&self.end)
            || (self.start..=self.end).contains(&other.start)
            || (self.start..=self.end).contains(&other.end)
    }
}

impl fmt::Display for SectionRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}
