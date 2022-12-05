use std::{fs, str::FromStr, time::Instant};

#[derive(Debug)]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

#[derive(Debug)]
struct Input {
    crates: Vec<Vec<char>>,
    moves: Vec<Move>,
}

fn main() {
    let mut now = Instant::now();
    let input = read_input("./src/input.txt");
    let elapsed = now.elapsed();
    println!("Parsing input took: {:.2?}", elapsed);
    move_crates(&input, false);
    move_crates(&input, true);
}

fn read_input(filename: &str) -> Input {
    let file_input: Vec<String> = fs::read_to_string(filename)
        .expect("Failed to read file")
        .lines()
        .map(|l| l.to_string())
        .collect();

    let parse_input: Vec<String> = file_input
        .iter()
        .filter(|line| line.contains("["))
        .map(|line| line.to_string())
        .collect();

    let alphabet: Vec<char> = ('A'..='Z').into_iter().collect::<Vec<char>>();

    let mut storage_width = 0;

    for (i, el) in parse_input.last().unwrap().chars().enumerate() {
        if alphabet.contains(&el) {
            storage_width += 1;
        }
    }

    let mut storage_crates: Vec<Vec<char>> = Vec::with_capacity(storage_width);

    for (i, el) in parse_input.last().unwrap().chars().enumerate() {
        if alphabet.contains(&el) {
            let mut column: Vec<char> = Vec::new();
            column.push(el);
            for j in (0..parse_input.len() - 1).rev() {
                let line = &parse_input[j];
                let storage_crate = line.chars().nth(i).unwrap();
                if !storage_crate.is_whitespace() {
                    column.push(line.chars().nth(i).unwrap());
                }
            }
            column.reverse();
            storage_crates.push(column);
        }
    }

    let second_part = file_input
        .iter()
        .filter(|line| line.contains("move"))
        .map(|line| {
            let (first, second) = line.split_once("from").unwrap();
            let amount_int = first
                .split_once(" ")
                .unwrap()
                .1
                .trim()
                .parse::<usize>()
                .unwrap();
            let second_part = second.split_once(" ").unwrap().1;
            let parts = second_part.split("to").collect::<Vec<_>>();
            let from_int = parts[0].trim().parse::<usize>().unwrap();
            let to_int = parts[1].trim().parse::<usize>().unwrap();
            Move {
                amount: amount_int,
                from: from_int,
                to: to_int,
            }
        })
        .collect();

    Input {
        crates: storage_crates,
        moves: second_part,
    }
}

fn move_crates(input: &Input, keep_order: bool) {
    let mut crates_clone = input.crates.clone();
    for crate_move in &input.moves {
        let to = crate_move.to - 1;
        let from = crate_move.from - 1;
        let mut crates_to_move: Vec<char> = Vec::new();

        for _ in 0..crate_move.amount {
            let to_move = crates_clone[from].remove(0);
            crates_to_move.push(to_move);
        }
        if keep_order {
            crates_to_move.reverse();
        }
        for crate_to_move in crates_to_move {
            crates_clone[to].insert(0, crate_to_move);
        }
    }
    let mut answer = "".to_owned();
    crates_clone
        .iter()
        .for_each(|column| answer.push(column[0]));
    println!("{}", answer);
}
