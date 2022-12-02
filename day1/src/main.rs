use std::fs;

fn main() {
    let input = read_input("./input.txt");
    let calories_list = get_calories(input);
    let most_calories = part1(&calories_list);
    let top_three = part2(&calories_list);
    println!("{}", most_calories);
    println!("{}", top_three);
}

fn part1(calories_list: &Vec<isize>) -> isize {
    *calories_list.iter().max().unwrap()
}

fn part2(calories_list: &Vec<isize>) -> isize {
    let mut list_clone = calories_list.clone();
    list_clone.sort();
    let mut result = 0;
    for i in 1..4 {
        result += list_clone[list_clone.len() - i];
    }
    result
}

fn get_calories(all_elfs: Vec<isize>) -> Vec<isize> {
    let mut calories_list = Vec::new();
    let mut current = 0;
    all_elfs.iter().for_each(|el| {
        if *el == -1 {
            calories_list.push(current);
            current = 0;
            return;
        }
        current += *el;
    });
    calories_list
}

fn read_input(filename: &str) -> Vec<isize> {
    fs::read_to_string(filename)
        .expect("There was an error reading the file")
        .lines()
        .map(|line: &str| line.parse::<isize>().unwrap_or(-1))
        .collect()
}
