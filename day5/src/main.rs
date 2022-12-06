use std::io::Write;
use std::thread;
use std::{fs, io, time::Instant};

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

#[derive(Debug, Clone)]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

#[derive(Debug, Clone)]
struct Input {
    crates: Vec<Vec<char>>,
    moves: Vec<Move>,
}

fn main() {
    let m = MultiProgress::new();
    let sty = ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue} {msg} ({eta})")
        .unwrap()
        .progress_chars("##-");

    let mut now = Instant::now();
    let input = read_input("./src/large_input.txt");
    let input_clone = input.clone();
    let mut elapsed = now.elapsed();
    println!("Parsing input took: {:.2?}", elapsed);
    now = Instant::now();

    let m_clone = m.clone();
    let sty_clone = sty.clone();

    let thread1 = thread::spawn(move || move_crates(&input, false, &m, sty));
    let thread2 = thread::spawn(move || move_crates(&input_clone, true, &m_clone, sty_clone));

    let _ = thread1.join();
    let _ = thread2.join();
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

fn move_crates(
    input: &Input,
    keep_order: bool,
    multi_progress: &MultiProgress,
    progress_style: ProgressStyle,
) {
    let mut crates_clone = input.crates.clone();
    let mut counter = 0;
    let mut now = Instant::now();

    let pb = multi_progress.add(ProgressBar::new(input.moves.len() as u64));
    pb.set_style(progress_style);
    for crate_move in &input.moves {
        counter += 1;
        pb.inc(1);
        pb.set_message(format!("Move {}/{}", counter, input.moves.len()));
        let to = crate_move.to - 1;
        let from = crate_move.from - 1;
        let mut crates_to_move: Vec<char> = crates_clone[from].drain(..crate_move.amount).collect();
        crates_clone[from].splice(..0, vec![]);
        // let mut crates_to_move: Vec<char> = crates_clone[from].split_off(crate_move.amount - 1);
        // let length = crates_clone[from].len();
        // println!("{} {}", length, crate_move.amount);
        // crates_clone[from].truncate(length - crate_move.amount);
        if !keep_order {
            crates_to_move.reverse();
        }
        crates_clone[to].splice(..0, crates_to_move);
    }
    let mut answer = "".to_owned();
    crates_clone
        .iter()
        .for_each(|column| answer.push(column[0]));
    let elapsed = now.elapsed();
    let format_finish_message = format!(
        "Finished in {:.2?} with result {} and keep order set to {}",
        elapsed, answer, keep_order
    );
    pb.finish_with_message(format_finish_message);
}
