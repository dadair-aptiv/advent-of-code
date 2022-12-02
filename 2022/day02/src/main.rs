use std::cmp::Ordering;
use std::fs;
use std::fmt;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Can't read the input file");

    let round_one = total_score(contents);
    println!("{:?}", round_one);
    assert_eq!(round_one, 8890);
}

fn score_round(elf: &str, you: &str) -> usize {
    let elf_play = Play::from(elf);
    let your_play = Play::from(you);

    println!("elf: {}", elf_play);
    println!("you: {}", your_play);

    if your_play > elf_play {
        return your_play.value() + Scoring::Win.value();
    } else if your_play < elf_play {
        return your_play.value() + Scoring::Lose.value();
    } else {
        return your_play.value() + Scoring::Draw.value();
    }
}

fn total_score(list_of_plays: String) -> usize {
    let mut total = 0;

    for line in list_of_plays.lines() {
        let plays: Vec<&str> = line.split(" ").collect();
        let round = score_round(plays[0], plays[1]);
        println!("{} {} = {}\n", plays[0], plays[1], round);
        total += round;
    }
    total
}

#[derive(Debug, PartialEq, Eq)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

impl Play {
    fn from(letter: &str) -> Self {
        match letter {
            "A" | "X" => Play::Rock,
            "B" | "Y" => Play::Paper,
            "C" | "Z" => Play::Scissors,
            &_ => unreachable!(),
        }
    }

    fn value(&self) -> usize {
        match *self {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        }
    }
}

impl fmt::Display for Play {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let play = match *self {
            Play::Rock => "Rock".to_string(),
            Play::Paper => "Paper".to_string(),
            Play::Scissors => "Scissors".to_string(),
        };
        write!(f, "{} = {}", play, self.value())
    }
}

impl PartialOrd for Play {

    fn partial_cmp(&self, other: &Play) -> Option<std::cmp::Ordering> {
        match *self {
            Play::Rock => match *other {
                Play::Scissors => Some(Ordering::Greater),
                Play::Paper => Some(Ordering::Less),
                Play::Rock => Some(Ordering::Equal),
            },
            Play::Paper => match *other {
                Play::Scissors => Some(Ordering::Less),
                Play::Paper => Some(Ordering::Equal),
                Play::Rock => Some(Ordering::Greater),
            },
            Play::Scissors => match *other {
                Play::Scissors => Some(Ordering::Equal),
                Play::Paper => Some(Ordering::Greater),
                Play::Rock => Some(Ordering::Less),
            },
        }
    }

}

#[derive(Debug)]
enum Scoring {
    Win,
    Lose,
    Draw,
}

impl Scoring {
    fn value(&self) -> usize {
        match *self {
            Scoring::Win => 6,
            Scoring::Draw => 3,
            Scoring::Lose => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_score_win() {
        let win = score_round("A", "Y");
        assert_eq!(win, 8);
    }

    #[test]
    fn should_score_loss() {
        let loss = score_round("B", "X");
        assert_eq!(loss, 1);
    }

    #[test]
    fn should_score_draw() {
        let draw = score_round("C", "Z");
        assert_eq!(draw, 6);
    }

    #[test]
    fn rock_beats_scissors() {
        let loss = score_round("A", "Z");
        assert_eq!(loss, 3);
    }

    #[test]
    fn should_score_all_given_rounds() {
        let example_scoring = "A Y
B X
C Z";

        let score = total_score(example_scoring.to_string());
        assert_eq!(score, 15);
    }
}

#[cfg(test)]
mod test_enums {
    use super::*;

    #[test]
    fn should_create_enum_from_rock_string() {
        let rock = Play::from("A");
        assert_eq!(rock, Play::Rock);

        let rock = Play::from("X");
        assert_eq!(rock, Play::Rock);
    }

    #[test]
    fn should_create_enum_from_paper_string() {
        let paper = Play::from("B");
        assert_eq!(paper, Play::Paper);

        let paper = Play::from("Y");
        assert_eq!(paper, Play::Paper);
    }

    #[test]
    fn should_create_enum_from_scissors_string() {
        let scissors = Play::from("C");
        assert_eq!(scissors, Play::Scissors);

        let scissors = Play::from("Z");
        assert_eq!(scissors, Play::Scissors);
    }

    #[test]
    fn should_comparisons_for_rock() {
        let rock = Play::Rock;
        let scissors = Play::Scissors;
        let paper = Play::Paper;
        assert!(rock > scissors);
        assert!(rock ==  rock);
        assert!(rock < paper);
    }

    #[test]
    fn should_comparisons_for_scissors() {
        let rock = Play::Rock;
        let scissors = Play::Scissors;
        let paper = Play::Paper;
        assert!(scissors > paper);
        assert!(scissors ==  scissors);
        assert!(scissors < rock);
    }

    #[test]
    fn should_comparisons_for_paper() {
        let rock = Play::Rock;
        let scissors = Play::Scissors;
        let paper = Play::Paper;
        assert!(paper > rock);
        assert!(paper ==  paper);
        assert!(paper < scissors);
    }
}
