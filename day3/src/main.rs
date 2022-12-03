use std::{collections::HashSet, fs};

fn main() {
    let input: Vec<String> = fs::read_to_string("./src/input.txt")
        .expect("Failed to read file")
        .lines()
        .map(|s| s.to_string())
        .collect();

    part1(&input);
    part2(&input);
}

fn part1(input: &Vec<String>) {
    let mut sum = 0;

    for line in input {
        let half_position = (line.len() / 2) - 1;
        let first_half = &line[..half_position + 1];
        let second_half = &line[half_position + 1..];
        let mut chars_in_both: HashSet<char> = HashSet::new();

        for i in 0..first_half.len() {
            if second_half.contains(first_half.chars().nth(i).unwrap()) {
                chars_in_both.insert(first_half.chars().nth(i).unwrap());
            }
        }
        for letter in chars_in_both {
            let score = get_char_score(letter);
            sum += score;
        }
    }

    println!("{}", sum);
}

fn part2(input: &Vec<String>) {
    let mut sum = 0;

    for group in input.chunks(3) {
        for i in 0..group.get(0).unwrap().len() {
            let first_elf = group.get(0).unwrap();
            let second_elf = group.get(1).unwrap();
            let third_elf = group.get(2).unwrap();

            let letter = first_elf.chars().nth(i).unwrap();
            if second_elf.contains(letter) && third_elf.contains(letter) {
                let score = get_char_score(letter);
                sum += score;
                break;
            }
        }
    }

    println!("{}", sum);
}

fn get_char_score(letter: char) -> usize {
    let lowercase = ('a'..='z').into_iter().collect::<Vec<char>>();
    let uppercase = ('A'..='Z').into_iter().collect::<Vec<char>>();

    if uppercase.contains(&letter) {
        return uppercase
            .iter()
            .position(|&character| character == letter)
            .unwrap()
            + 27;
    } else {
        return lowercase
            .iter()
            .position(|&character| character == letter)
            .unwrap()
            + 1;
    }
}
