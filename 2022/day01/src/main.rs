use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Can't read the input file");

    let sorted_calories = sort_calories(contents);

    let top_three = sorted_calories[0] + sorted_calories[1] + sorted_calories[2];
    let max = sorted_calories[0];
    println!("Top three = {:?}", top_three);
    println!("Max = {:?}", max);

    assert_eq!(top_three, 199172);
    assert_eq!(max, 66616);
}

fn sort_calories(calorie_list: String) -> Vec<usize> {
    let mut elves = vec![];
    let mut total_calories = 0;
    for line in calorie_list.lines() {
        if line.is_empty() {
            elves.push(total_calories);
            total_calories = 0;
        } else {
            let calories = line.parse::<usize>().unwrap();
            total_calories += calories;
        }
    }

    // Last one isn't empty so not stored
    elves.push(total_calories);

    // Sort list largest to smallest
    elves.sort_by(|a, b| b.partial_cmp(a).unwrap());
    return elves;
}

#[cfg(test)]
mod tests {
    use crate::sort_calories;

    #[test]
    fn example_calorie_list() {
        let calorie_list = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        let calorie_count = sort_calories(calorie_list.to_string());

        let max = calorie_count[0];
        assert_eq!(max, 24000);

        assert_eq!(calorie_count[1], 11000);
        assert_eq!(calorie_count[2], 10000);

        let top_three = calorie_count[0] + calorie_count[1] + calorie_count[2];
        assert_eq!(top_three, 45000);
    }

}
