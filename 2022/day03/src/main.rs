use std::fs;

use std::collections::HashSet;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Can't read the input file");

    let mistake_priority = count_mistake_priority(search_rucksacks(contents));
    println!("{:?}", mistake_priority);
    assert_eq!(mistake_priority, 7903);
}




fn find_item(drawer: String) -> char {
    let mut compartment0 = HashSet::new();
    let mut compartment1 = HashSet::new();

    let compartments = drawer.split_at(drawer.len() / 2);

    compartments.0.chars().for_each(|l| { compartment0.insert(l); });
    compartments.1.chars().for_each(|l| { compartment1.insert(l); });
    *compartment0.intersection(&compartment1).last().unwrap()
}


fn search_rucksacks(rucksacks: String) -> Vec<char> {
    let mut letters = vec![];
    rucksacks.lines().for_each(|line| letters.push(find_item(line.to_string())));
    letters
}

fn count_mistake_priority(letters: Vec<char>) -> usize {
    let mut count = 0;
    letters.iter().for_each(|letter| {
        let letter_value_in_ascii = *letter as u32;

        // Uppercase
        if letter_value_in_ascii < 'a' as u32 {
            count += letter_value_in_ascii - 'A' as u32 + 27;
        } else {
            count += letter_value_in_ascii - 'a' as u32 + 1;
        }
    });
    count.try_into().unwrap()

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_rucksack_item_duplicate() {
        let letter = find_item("vJrwpWtwJgWrhcsFMMfFFhFp".to_string());

        assert_eq!(letter, 'p');
    }

    #[test]
    fn should_find_when_each_compartment_has_two_of_same_chars() {
        let letter = find_item("PmmdzqPrVvPwwTWBwg".to_string());

        assert_eq!(letter, 'P');
    }

    #[test]
    fn should_find_a_container_of_rucksacks() {
        let letters = search_rucksacks("vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw".to_string());

        assert_eq!(letters, vec!['p', 'L', 'P', 'v', 't', 's']);
    }

    #[test]
    fn should_count_priorities() {
        let priority = count_mistake_priority(vec!['p', 'L', 'P', 'v', 't', 's']);

        assert_eq!(priority, 157);
    }

}
