pub struct Rucksack {
    first_compartment: Vec<Item>,
    second_compartment: Vec<Item>,
    total_contents: Vec<Item>,
}

impl Rucksack {
    pub fn new(rucksack_string: &str) -> Rucksack {
        let rucksack_middle = rucksack_string.len() / 2;
        let total_contents: Vec<Item> = rucksack_string.chars().map(|c| Item(c)).collect();
        let mut first_compartment = total_contents.clone();
        let second_compartment = first_compartment.split_off(rucksack_middle);

        Rucksack {
            first_compartment,
            second_compartment,
            total_contents,
        }
    }

    pub fn item_in_both_compartments(&self) -> &Item {
        Self::vec_compare(&self.first_compartment, &[&self.second_compartment])
    }

    pub fn item_shared(&self, other_rucksacks: &[&Rucksack]) -> &Item {
        let other_items: Vec<&Vec<Item>> = other_rucksacks
            .iter()
            .map(|rucksack| &rucksack.total_contents)
            .collect();

        Self::vec_compare(&self.total_contents, &other_items)
    }

    fn vec_compare<'a>(first_items: &'a Vec<Item>, other_items: &[&Vec<Item>]) -> &'a Item {
        for first_item in first_items.iter() {
            if other_items
                .iter()
                .all(|item_vec| item_vec.iter().any(|item| item == first_item))
            {
                return first_item;
            }
        }

        panic!(
            "No matching item in the rucksack! {:?} {:?}",
            first_items, other_items
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
