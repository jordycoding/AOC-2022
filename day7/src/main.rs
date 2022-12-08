use std::{
    cell::{Ref, RefCell},
    collections::HashSet,
    fs,
    rc::Rc,
    thread::current,
};

use regex::Regex;

#[derive(PartialEq, Debug)]
struct TreeNode {
    pub name: String,
    pub size: usize,
    pub children: Vec<Rc<RefCell<TreeNode>>>,
    pub parent: Option<Rc<RefCell<TreeNode>>>,
    pub is_dir: bool,
}

impl TreeNode {
    pub fn new(name: &str) -> TreeNode {
        TreeNode {
            name: name.to_string(),
            size: 0,
            children: vec![],
            parent: None,
            is_dir: false,
        }
    }

    pub fn add_child(&mut self, new_node: Rc<RefCell<TreeNode>>) {
        self.children.push(new_node);
    }

    pub fn get_size(&self) -> usize {
        let mut size = 0;
        size += self.size;

        for child in self.children.iter() {
            let child_clone = child.borrow();
            size += child_clone.get_size();
        }
        size
    }
}

fn main() {
    // let input = read_input("./src/jona.test");
    let root_node = read_input("./src/input.prod");

    let output_part1 = part1(root_node);
    println!("Part 1: {}", output_part1);
    // println!("Total size: {}", total);
}

fn read_input(filename: &str) -> Rc<RefCell<TreeNode>> {
    let cd_regex = Regex::new(r"cd (.*)").unwrap();
    let size_regex = Regex::new(r"(\d+) (.*)").unwrap();

    let root = Rc::new(RefCell::new(TreeNode::new("root")));
    let mut current = Rc::clone(&root);

    fs::read_to_string(filename)
        .expect("Error reading file")
        .lines()
        .for_each(|line| {
            if line.contains("$") {
                if cd_regex.is_match(line) {
                    let cap = cd_regex.captures(line).unwrap();
                    if !(&cap[1] == ".." || &cap[1] == "/") {
                        let child = Rc::new(RefCell::new(TreeNode::new(&cap[1])));
                        println!("Creating new child: {}", &cap[1]);
                        current.borrow_mut().add_child(Rc::clone(&child));
                        {
                            let mut mut_child = child.borrow_mut();
                            mut_child.is_dir = true;
                            mut_child.parent = Some(Rc::clone(&current));
                        }
                        current = child;
                    } else if &cap[1] == ".." {
                        let current_clone = Rc::clone(&current);
                        current = Rc::clone(current_clone.borrow().parent.as_ref().unwrap());
                    } else {
                        return;
                    }
                }
            } else {
                if size_regex.is_match(line) {
                    let size_cap = size_regex.captures(line).unwrap();
                    let size = &size_cap[1].parse::<usize>().unwrap();
                    current.borrow_mut().size += size;
                    // current_size += size;
                }
            }
        });
    root
}

fn part1(root_node: Rc<RefCell<TreeNode>>) -> usize {
    let mut size = 0;

    let root = root_node.borrow();

    if root.get_size() <= 100000 {
        size += root.get_size();
    }
    for child in root.children.iter() {
        println!(
            "Found child: {} with size: {}",
            child.borrow().name,
            child.borrow().get_size()
        );
        size += part1(Rc::clone(&child));
    }
    size
}

fn total_size(root_node: Rc<RefCell<TreeNode>>) -> usize {
    let mut size = 0;
    let root = root_node.borrow();

    size += root.size;
    for child in root.children.iter() {
        size += total_size(Rc::clone(&child));
    }
    size
}
