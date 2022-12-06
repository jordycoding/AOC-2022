use std::fs;

fn main() {
    let input = read_input("./src/input.txt");
    part1(&input);
    part2(&input);
}

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Failed to read file")
}

fn part1(input: &str) {
    for (i, el) in input.chars().skip(4).enumerate() {
        let index = i + 4;
        let previous = &input[index - 4..index];
        let mut previous_chars = previous.chars().collect::<Vec<char>>();
        previous_chars.sort();
        previous_chars.dedup();
        if previous_chars.len() == previous.len() {
            println!("Found marker at position {}", index);
            break;
        }
    }
}

fn part2(input: &str) {
    for (i, el) in input.chars().skip(14).enumerate() {
        let index = i + 14;
        let previous = &input[index - 14..index];
        let mut previous_chars = previous.chars().collect::<Vec<char>>();
        previous_chars.sort();
        previous_chars.dedup();
        if previous_chars.len() == previous.len() {
            println!("Found marker at position {}", index);
            break;
        }
    }
}
