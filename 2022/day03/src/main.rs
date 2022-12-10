use std::fs;

use std::collections::HashSet;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Can't read the input file");

    let mistake_priority = count_priorities(search_rucksacks(contents.clone()));
    println!("{:?}", mistake_priority);
    assert_eq!(mistake_priority, 7903);

    let badge_total = count_priorities(search_for_badges(contents));
    println!("{:?}", badge_total);
    assert_eq!(badge_total, 2548);
}


#[derive(Debug)]
struct RuckSack {
    letter: char,
    _contents: String,
}

fn find_item(drawer: String) -> RuckSack {
    let mut compartment0 = HashSet::new();
    let mut compartment1 = HashSet::new();

    let compartments = drawer.split_at(drawer.len() / 2);

    compartments.0.chars().for_each(|l| { compartment0.insert(l); });
    compartments.1.chars().for_each(|l| { compartment1.insert(l); });
    RuckSack {
        letter: *compartment0.intersection(&compartment1).last().unwrap(),
        _contents: drawer,
    }

}


fn search_rucksacks(rucksacks: String) -> Vec<char> {
    let mut letters = vec![];
    rucksacks.lines().for_each(|line| letters.push(find_item(line.to_string()).letter));
    letters
}

fn count_priorities(letters: Vec<char>) -> usize {
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

fn search_for_badges(rucksacks: String) -> Vec<char> {
    let mut badges = vec![];

    let mut lines = vec![];
    rucksacks.lines().for_each(|line| lines.push(line.to_string()));

    for group in lines.chunks(3) {
        let mut group0 = HashSet::new();
        let mut group1 = HashSet::new();
        let mut group2 = HashSet::new();

        for (i, line) in group.iter().enumerate() {
             if i == 0 {
                line.chars().for_each(|l| { group1.insert(l); })
            } else if i == 1 {
                line.chars().for_each(|l| { group2.insert(l); })
            } else {
                line.chars().for_each(|l| { group0.insert(l); })
            }
        }
        let first_intersection = group0.intersection(&group1).cloned().collect::<HashSet<char>>();
        let intersection = first_intersection.intersection(&group2);
        badges.push(*intersection.last().unwrap());
    }
    badges
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_rucksack_item_duplicate() {
        let rucksack = find_item("vJrwpWtwJgWrhcsFMMfFFhFp".to_string());

        assert_eq!(rucksack.letter, 'p');
    }

    #[test]
    fn should_find_when_each_compartment_has_two_of_same_chars() {
        let rucksack = find_item("PmmdzqPrVvPwwTWBwg".to_string());

        assert_eq!(rucksack.letter, 'P');
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
        let priority = count_priorities(vec!['p', 'L', 'P', 'v', 't', 's']);

        assert_eq!(priority, 157);
    }

    #[test]
    fn should_find_ruck_sack_group_badges() {
        let badges = search_for_badges("vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw".to_string());


        assert_eq!(badges, vec!['r', 'Z']);
    }

}
