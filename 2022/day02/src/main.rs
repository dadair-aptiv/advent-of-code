use std::str::FromStr;
use std::cmp::Ordering;
use std::fs;
use std::fmt;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Can't read the input file");

    // How scoring changed so no longer true
    // let round_one = total_score(contents);
    // println!("{:?}", round_one);
    // assert_eq!(round_one, 8890);

    let round_two = total_score(contents);
    println!("{:?}", round_two);
    assert_eq!(round_two, 10238);
}

fn score_round(elf: &str, round: &str) -> usize {
    let elf_play = elf.parse().unwrap();
    let round_status = round.parse().unwrap();

    println!("elf: {}", elf_play);
    println!("round: {}", round_status);

    let your_play = match round_status {
        Scoring::Win => match elf_play {
            Play::Rock => Play::Paper,
            Play::Paper => Play::Scissors,
            Play::Scissors => Play::Rock,
        },
        Scoring::Lose => match elf_play {
            Play::Rock => Play::Scissors,
            Play::Paper => Play::Rock,
            Play::Scissors => Play::Paper,
        },
        Scoring::Draw => elf_play,
    };

    println!("you: {}", your_play);
    your_play.value() + round_status.value()
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
    fn value(&self) -> usize {
        match *self {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        }
    }
}

impl FromStr for Play{
    type Err = ();

    fn from_str(letter: &str) -> Result<Self, Self::Err> {
        match letter {
            "A" => Ok(Play::Rock),
            "B" => Ok(Play::Paper),
            "C" => Ok(Play::Scissors),
            &_ => Err(()),
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

#[derive(Debug, PartialEq, Eq)]
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

impl FromStr for Scoring{
    type Err = ();

    fn from_str(letter: &str) -> Result<Self, Self::Err> {
        match letter {
            "X" => Ok(Scoring::Lose),
            "Y" => Ok(Scoring::Draw),
            "Z" => Ok(Scoring::Win),
            &_ => Err(()),
        }
    }
}

impl fmt::Display for Scoring {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let score = match *self {
            Scoring::Win => "Win".to_string(),
            Scoring::Draw => "Draw".to_string(),
            Scoring::Lose => "Lose".to_string(),
        };
        write!(f, "{} = {}", score, self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_score_draw() {
        let win = score_round("A", "Y");
        assert_eq!(win, 4);
    }

    #[test]
    fn should_score_loss() {
        let loss = score_round("B", "X");
        assert_eq!(loss, 1);
    }

    #[test]
    fn should_score_win() {
        let draw = score_round("C", "Z");
        assert_eq!(draw, 7);
    }

    #[test]
    fn rock_with_win() {
        let rock = score_round("A", "Z");
        assert_eq!(rock, 8);
    }

    #[test]
    fn should_score_all_given_rounds() {
        let example_scoring = "A Y
B X
C Z";

        let score = total_score(example_scoring.to_string());
        assert_eq!(score, 12);
    }
}

#[cfg(test)]
mod test_enums {
    use super::*;

    #[test]
    fn should_create_enum_from_rock_string() {
        let rock = "A".parse::<Play>().unwrap();
        assert_eq!(rock, Play::Rock);
    }

    #[test]
    fn should_create_enum_from_paper_string() {
        let paper = "B".parse::<Play>().unwrap();
        assert_eq!(paper, Play::Paper);
    }

    #[test]
    fn should_create_enum_from_scissors_string() {
        let scissors = "C".parse::<Play>().unwrap();
        assert_eq!(scissors, Play::Scissors);
    }

    #[test]
    fn should_create_enum_from_lose_string() {
        let loss = "X".parse::<Scoring>().unwrap();
        assert_eq!(loss, Scoring::Lose);
    }

    #[test]
    fn should_create_enum_from_draw_string() {
        let draw = "Y".parse::<Scoring>().unwrap();
        assert_eq!(draw, Scoring::Draw);
    }

    #[test]
    fn should_create_enum_from_win_string() {
        let win = "Z".parse::<Scoring>().unwrap();
        assert_eq!(win, Scoring::Win);
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
