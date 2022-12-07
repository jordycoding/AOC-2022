use std::{fs, thread::current};

use regex::Regex;

#[derive(Debug)]
struct Dir {
    abs_path: String,
    name: String,
    file_sizes: usize,
    parent_dir: String,
}

fn main() {
    let input = read_input("./src/jona.test");
    // println!("{:?}", input);
    part1(&input);
}
fn part1(input: &Vec<Dir>) {
    // println!("{}", input.len());
    let input_clone = input.clone();
    let sizes: Vec<(&String, usize)> = input_clone
        .iter()
        .map(|dir| {
            let size = get_dir_size(&input_clone, &dir.abs_path);
            (&dir.name, size)
        })
        .filter(|dir_size| dir_size.1 <= 100000)
        .collect();

    let sum: usize = sizes.iter().map(|dir_size| dir_size.1).sum();
    // println!("{:?}", sizes.len());

    println!("{:?}", sizes);
    println!("{:?}", sum);
}

fn read_input(filename: &str) -> Vec<Dir> {
    let mut current_dir = String::from("");
    let mut current_size = 0;
    let cd_regex = Regex::new(r"cd (.*)").unwrap();
    let size_regex = Regex::new(r"(\d+).*").unwrap();
    let mut dirs: Vec<Dir> = Vec::new();

    let all_dirs_regex = Regex::new(r"([a-z]+/)").unwrap();

    fs::read_to_string(filename)
        .expect("Error reading file")
        .lines()
        .for_each(|line| {
            if line.contains("$") {
                if cd_regex.is_match(line) {
                    let cap = cd_regex.captures(line).unwrap();
                    push_current_dir_size(&current_dir, &mut current_size, &mut dirs);
                    if !(&cap[1] == ".." || &cap[1] == "/") {
                        current_dir.push_str(&cap[1]);
                        current_dir.push_str("/");
                    } else if &cap[1] == ".." {
                        if all_dirs_regex.is_match(&current_dir) {
                            let child_cap = all_dirs_regex.find_iter(&current_dir).last().unwrap();
                            current_dir = current_dir.replace(child_cap.as_str(), "");
                        }
                    } else {
                        return;
                    }
                    // println!("{} Current dir: {}", line, current_dir);
                }
            } else {
                if !(current_dir == "") {
                    if size_regex.is_match(line) {
                        let size_cap = size_regex.captures(line).unwrap();
                        let size = &size_cap[1].parse::<usize>().unwrap();
                        current_size += size;
                        // println!("{:?} {:?}", current_size, size);
                    }
                }
            }
        });
    push_current_dir_size(&current_dir, &mut current_size, &mut dirs);
    dirs
}

fn push_current_dir_size(current_dir: &str, current_size: &mut usize, dirs: &mut Vec<Dir>) {
    let all_dirs_regex = Regex::new(r"([a-z]+/)").unwrap();
    if all_dirs_regex.is_match(&current_dir) {
        // let parent_cap = all_dirs_regex.find_iter(&current_dir).nth(0).unwrap();
        let parent_cap = all_dirs_regex
            .find_iter(&current_dir)
            .fold("".to_owned(), |parent_string, curr| {
                parent_string + curr.as_str()
            });
        let cap_clone = parent_cap.clone();
        let child_cap = all_dirs_regex.find_iter(&current_dir).last().unwrap();
        let cap_count = all_dirs_regex.find_iter(&current_dir).count();
        if cap_count > 1 {
            dirs.push(Dir {
                abs_path: cap_clone,
                name: child_cap.as_str().replace("/", ""),
                file_sizes: *current_size,
                parent_dir: parent_cap.as_str().replace(child_cap.as_str(), ""),
            });
            *current_size = 0;
        }
        if cap_count == 1 {
            if !(*current_size == 0) {
                let name = parent_cap.as_str().replace("/", "");
                if !dirs.iter().any(|dir| dir.name == name) {
                    dirs.push(Dir {
                        abs_path: parent_cap,
                        name,
                        file_sizes: *current_size,
                        parent_dir: "".to_owned(),
                    });
                }
                *current_size = 0;
            }
        }
    }
}

fn get_dir_size(input: &[Dir], path: &str) -> usize {
    // let mut size = 0;
    // if size == 0 {
    //     println!("initial call");
    // }
    for (index, possible_child) in input.iter().enumerate() {
        if possible_child.abs_path == path {
            println!("Found it!");
            // size += possible_child.file_sizes;
            return possible_child.file_sizes
                + get_dir_size(&input[index + 1..], &possible_child.abs_path);
        }
        if possible_child.parent_dir == path {
            println!("Found a child");
            // size += get_dir_size(&input[index..], &possible_child.name);
            return possible_child.file_sizes
                + get_dir_size(&input[index + 1..], &possible_child.abs_path);
        }
    }
    0
    // return size;
}
