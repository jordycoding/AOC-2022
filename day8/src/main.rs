use std::fs;

fn main() {
    let input = read_input("./src/input.txt");
    let part1_output = part1(&input);
    println!("Part 1: {}", part1_output);
}

fn read_input(filename: &str) -> Vec<Vec<usize>> {
    let mut output = Vec::new();
    let input = fs::read_to_string(filename)
        .expect("There was an error reading the input")
        .lines()
        .for_each(|line| {
            let mut inner = Vec::new();
            line.chars().for_each(|tree| {
                inner.push(tree.to_string().parse::<usize>().unwrap());
            });
            output.push(inner);
        });
    output
}

fn part1(input: &Vec<Vec<usize>>) -> usize {
    let mut visible_trees = 0;
    // The entire top and bottom row are visible
    visible_trees += input.len() * 2;
    // All trees on the left and right edges are also visible
    visible_trees += &input[1..input.len() - 1].len() * 2;

    for row in 1..input.len() - 1 {
        for tree in 1..input.get(row).unwrap().len() - 1 {
            if check_tree_visibility(&input, row, tree) {
                visible_trees += 1;
            }
        }
    }
    visible_trees
}

fn check_tree_visibility(input: &Vec<Vec<usize>>, tree_row: usize, tree_index: usize) -> bool {
    let searched_tree = input.get(tree_row).unwrap().get(tree_index).unwrap();
    let mut top_blocked = false;
    let mut bottom_blocked = false;
    let mut left_blocked = false;
    let mut right_blocked = false;
    for row in 0..tree_row {
        if input.get(row).unwrap().get(tree_index).unwrap() >= searched_tree {
            top_blocked = true;
        }
    }
    for row in tree_row + 1..input.len() {
        if input.get(row).unwrap().get(tree_index).unwrap() >= searched_tree {
            bottom_blocked = true;
        }
    }
    for column in 0..tree_index {
        if input.get(tree_row).unwrap().get(column).unwrap() >= searched_tree {
            left_blocked = true;
        }
    }
    for column in tree_index + 1..input.get(0).unwrap().len() {
        if input.get(tree_row).unwrap().get(column).unwrap() >= searched_tree {
            right_blocked = true;
        }
    }
    if bottom_blocked == false
        || top_blocked == false
        || right_blocked == false
        || left_blocked == false
    {
        return true;
    }
    false
}
