pub struct Rucksack {
    first_compartment: Vec<Item>,
    second_compartment: Vec<Item>,
}

impl Rucksack {
    pub fn new(rucksack_string: &str) -> Rucksack {
        let rucksack_middle = rucksack_string.len() / 2;
        let mut first_compartment: Vec<Item> = rucksack_string.chars().map(|c| Item(c)).collect();
        let second_compartment = first_compartment.split_off(rucksack_middle);

        Rucksack {
            first_compartment,
            second_compartment,
        }
    }

    pub fn item_in_both_compartments(&self) -> Item {
        for first_item in self.first_compartment.iter() {
            for second_item in self.second_compartment.iter() {
                if first_item == second_item {
                    return first_item.clone();
                }
            }
        }

        panic!(
            "No matching item in the rucksack! {:?} {:?}",
            self.first_compartment, self.second_compartment
        );
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Item(char);

impl Item {
    pub fn value(&self) -> u32 {
        let item_char = self.0 as u32;

        // The value of "a" is 97 and the value of "z" is 122.
        if item_char >= 97 && item_char <= 122 {
            item_char - 96
        // The value of "A" is 65 and the value of "Z" is 90.
        } else if item_char >= 65 && item_char <= 90 {
            item_char - (64 - 26)
        } else {
            0
        }
    }
}
