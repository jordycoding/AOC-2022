use std::{fs, str::FromStr};

#[derive(Eq, PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

#[derive(Eq, PartialEq)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl FromStr for Outcome {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            &_ => Err(()),
        }
    }
}

impl FromStr for Choice {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Choice::Rock),
            "B" => Ok(Choice::Paper),
            "C" => Ok(Choice::Scissors),
            "X" => Ok(Choice::Rock),
            "Y" => Ok(Choice::Paper),
            "Z" => Ok(Choice::Scissors),
            &_ => Err(()),
        }
    }
}

impl Choice {
    pub fn outcome(&self, other: &Choice) -> Outcome {
        if *self == Choice::Paper && *other == Choice::Rock {
            Outcome::Win
        } else if *self == Choice::Scissors && *other == Choice::Paper {
            Outcome::Win
        } else if *self == Choice::Rock && *other == Choice::Scissors {
            Outcome::Win
        } else if *self == *other {
            Outcome::Draw
        } else {
            Outcome::Lose
        }
    }

    pub fn get_score(&self) -> usize {
        if *self == Choice::Rock {
            1
        } else if *self == Choice::Paper {
            2
        } else {
            3
        }
    }
}

fn main() {
    let first_input = input_part1("./src/input.txt");
    println!("{}", part1(first_input));
    let second_input = input_part2("./src/input.txt");
    println!("{}", part2(second_input));
}

fn part1(input: Vec<(Choice, Choice)>) -> usize {
    let mut score = 0;
    input.iter().for_each(|set: &(Choice, Choice)| {
        score += set.1.get_score();
        match set.1.outcome(&set.0) {
            Outcome::Win => score += 6,
            Outcome::Draw => score += 3,
            Outcome::Lose => score += 0,
        }
    });
    score
}

fn part2(input: Vec<(Choice, Outcome)>) -> usize {
    let mut score = 0;
    input.iter().for_each(|set: &(Choice, Outcome)| {
        if set.1 == Outcome::Lose {
            match set.0 {
                Choice::Paper => score += Choice::Rock.get_score(),
                Choice::Rock => score += Choice::Scissors.get_score(),
                Choice::Scissors => score += Choice::Paper.get_score(),
            }
        } else if set.1 == Outcome::Draw {
            score += set.0.get_score();
            score += 3;
        } else {
            match set.0 {
                Choice::Rock => score += Choice::Paper.get_score(),
                Choice::Paper => score += Choice::Scissors.get_score(),
                Choice::Scissors => score += Choice::Rock.get_score(),
            }
            score += 6;
        }
    });
    score
}

fn input_part1(filename: &str) -> Vec<(Choice, Choice)> {
    fs::read_to_string(filename)
        .expect("There was an error reading the file")
        .lines()
        .map(|line: &str| {
            let (first, second) = line.split_once(" ").unwrap();
            let first_choice = Choice::from_str(first).expect("Error parsing");
            let second_choice = Choice::from_str(second).expect("Error parsing");
            (first_choice, second_choice)
        })
        .collect()
}

fn input_part2(filename: &str) -> Vec<(Choice, Outcome)> {
    fs::read_to_string(filename)
        .expect("There was an error reading the file")
        .lines()
        .map(|line: &str| {
            let (first, second) = line.split_once(" ").unwrap();
            let first_choice = Choice::from_str(first).expect("Error parsing");
            let second_outcome = Outcome::from_str(second).expect("Error parsing");
            (first_choice, second_outcome)
        })
        .collect()
}
