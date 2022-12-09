use std::{cell::RefCell, collections::HashMap, rc::Rc};

fn main() {
    let root = Rc::new(Folder::new_empty(None));
    let mut cwd = Rc::clone(&root);

    for line in include_str!("input.txt").lines() {
        let args = line.split_whitespace().collect::<Vec<&str>>();
        match (args[0], args[1]) {
            ("$", "ls") => {}
            ("$", "cd") => match args[2] {
                "/" => {}
                ".." => {
                    cwd = Rc::clone(cwd.parent.as_ref().unwrap());
                }
                dir => {
                    let new_folder = cwd.children.borrow().get(dir).unwrap().clone();
                    cwd = new_folder;
                }
            },
            ("dir", dir) => {
                cwd.children.borrow_mut().insert(
                    dir.to_string(),
                    Rc::new(Folder::new_empty(Some(Rc::clone(&cwd)))),
                );
            }
            (size, name) => {
                let file_size = size.parse().unwrap();
                cwd.files.borrow_mut().insert(file_size, name.to_string());
            }
        }
    }

    let sums = root.sum_children();

    let n = 30000000 - (70000000 - &sums[0]);
    let mut dirs: Vec<&usize> = sums.iter().filter(|x| **x >= n).collect();
    dirs.sort();

    let part_1: usize = sums.iter().filter(|x| **x <= 100000).sum();
    let part_2 = dirs[0];

    println!("{}, {}", part_1, part_2);
}

struct Folder {
    parent: Option<Rc<Folder>>,
    children: RefCell<HashMap<String, Rc<Folder>>>,
    files: RefCell<HashMap<usize, String>>,
}

impl Folder {
    fn new_empty(parent: Option<Rc<Folder>>) -> Folder {
        Folder {
            parent,
            files: RefCell::new(HashMap::new()),
            children: RefCell::new(HashMap::new()),
        }
    }

    fn get_size(&self) -> usize {
        let mut size: usize = self.files.borrow().keys().sum();
        for child in self.children.borrow().values() {
            size += child.get_size();
        }
        size
    }

    fn sum_children(&self) -> Vec<usize> {
        let mut sums: Vec<usize> = Vec::new();
        sums.push(self.get_size());
        for child in self.children.borrow().values() {
            sums.append(&mut child.sum_children())
        }
        sums
    }
}
