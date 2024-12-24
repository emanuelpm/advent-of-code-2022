use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;

pub fn day07() {
    let filename = "dat/2022/day07.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    struct Item<'a> {
        name: String,
        size: u64,
        is_dir: bool,
        parent: Option<&'a mut Item<'a>>,
        children: Vec<Item<'a>>,
    }

    let mut root = Item {
        name: String::from("/"),
        size: 0,
        is_dir: true,
        parent: None,
        children: Vec::new(),
    };

    let mut current = &mut root;

    for line in reader.lines() {
        let line = line.unwrap();
        let tokens = line.split(' ').collect::<Vec<&str>>();

        // User input
        if tokens[0] == "$" {
            if tokens[1] == "ls" {
                // Because we are assuming valid input, we don't need to do anything here
            }
            else if tokens[1] == "cd" {
                if tokens[2] == ".." {
                    let parent : &mut Item = current.parent.as_deref_mut().unwrap();
                    current = parent;
                }
                else {
                    for mut child in current.children {
                        if child.name == tokens[2] {
                            current = &mut child;
                            break;
                        }
                    }
                }
            }
        }
        // Listing a directory
        else if tokens[0] == "dir" {
            let mut is_found = false;
            for child in current.children {
                if child.name == tokens[1] {
                    is_found = true;
                    break;
                }
            }

            if !is_found {
                let new_item = Item {
                    name: tokens[1].to_string(),
                    size: 0,
                    is_dir: true,
                    parent: Some(&mut current),
                    children: Vec::new(),
                };

                current.children.push(new_item);
            }
        }
        // Listing a file
        else {
            let mut is_found = false;
            for child in current.children {
                if child.name == tokens[1] {
                    is_found = true;
                    break;
                }
            }

            if !is_found {
                let new_item = Item {
                    name: tokens[1].to_string(),
                    size: tokens[0].parse::<u64>().unwrap(),
                    is_dir: false,
                    parent: Some(&mut current),
                    children: Vec::new(),
                };

                // Update the size of the parents
                let mut parent = current.parent.as_deref_mut();
                while parent.is_some() {
                    let item = parent.as_deref_mut().unwrap();
                    item.size += new_item.size;
                    parent = item.parent.as_deref_mut();
                }

                current.children.push(new_item);
            }
        }
    }

    // [Part 1]
    println!("Total size: {}", root.size);
}
