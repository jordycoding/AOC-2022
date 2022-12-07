use std::{fs, thread::current};

use regex::Regex;

#[derive(Debug)]
struct Dir {
    name: String,
    file_sizes: usize,
    parent_dir: String,
}

fn main() {
    let input = read_input("./src/input.prod");
    println!("{:?}", input);
}

fn read_input(filename: &str) -> Vec<Dir> {
    let mut current_dir = String::from("");
    let mut current_size = 0;
    let cd_regex = Regex::new(r"cd (.*)").unwrap();
    let size_regex = Regex::new(r"(\d+).*").unwrap();
    let mut dirs: Vec<Dir> = Vec::new();

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
                        // if current_dir.len() == 2 {
                        //     current_dir = "".to_owned();
                        //     return;
                        // }
                        current_dir.pop();
                        if current_dir.len() > 1 {
                            current_dir.pop();
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
        let parent_cap = all_dirs_regex.find_iter(&current_dir).nth(0).unwrap();
        let child_cap = all_dirs_regex.find_iter(&current_dir).last().unwrap();
        let cap_count = all_dirs_regex.find_iter(&current_dir).count();
        if cap_count > 1 {
            dirs.push(Dir {
                name: child_cap.as_str().replace("/", ""),
                file_sizes: *current_size,
                parent_dir: parent_cap.as_str().replace("/", ""),
            });
            *current_size = 0;
        }
        if cap_count == 1 {
            if !(*current_size == 0) {
                let name = parent_cap.as_str().replace("/", "");
                if !dirs.iter().any(|dir| dir.name == name) {
                    dirs.push(Dir {
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
