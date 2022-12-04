use std::fs;

struct Range {
    begin: usize,
    end: usize,
}

fn main() {
    let input: Vec<(Range, Range)> = fs::read_to_string("./src/input.txt")
        .expect("Failed to read file")
        .lines()
        .map(|s| {
            let (first, second) = s.split_once(",").unwrap();
            let (begin_first_range, end_first_range) = first.split_once("-").unwrap();
            let (begin_second_range, end_second_range) = second.split_once("-").unwrap();
            let first_elf = Range {
                begin: begin_first_range.parse::<usize>().unwrap(),
                end: end_first_range.parse::<usize>().unwrap(),
            };
            let second_elf = Range {
                begin: begin_second_range.parse::<usize>().unwrap(),
                end: end_second_range.parse::<usize>().unwrap(),
            };
            (first_elf, second_elf)
        })
        .collect();

    part1(&input);
    part2(&input);
}

fn part1(input: &Vec<(Range, Range)>) {
    let sum: usize = input
        .iter()
        .map(|pair| {
            if pair.0.begin <= pair.1.begin && pair.0.end >= pair.1.end {
                1
            } else if pair.1.begin <= pair.0.begin && pair.1.end >= pair.0.end {
                1
            } else {
                0
            }
        })
        .sum();

    println!("{}", sum);
}

fn part2(input: &Vec<(Range, Range)>) {
    let sum: usize = input
        .iter()
        .map(|pair| {
            let first_range = pair.0.begin..=pair.0.end;
            let second_range = pair.1.begin..=pair.1.end;
            let mut sum = 0;
            for i in first_range {
                if second_range.contains(&i) {
                    sum += 1;
                    break;
                }
            }
            sum
        })
        .sum();

    println!("{}", sum);
}
