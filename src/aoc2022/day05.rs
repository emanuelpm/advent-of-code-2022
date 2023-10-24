use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use std::collections::VecDeque;


pub fn day05() {
    let filename = "dat/2022/day05.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    #[derive(PartialEq, Eq)]
    enum ReaderState {
        FirstLine,
        StartingState,
        Actions
    }

    let mut stacks_p1: Vec<VecDeque<char>> = Vec::new();
    let mut stacks_p2: Vec<VecDeque<char>> = Vec::new();
    let mut reader_state : ReaderState = ReaderState::FirstLine;

    // We're going to make some assumptions:
    //     1) The file isn't empty
    //     2) The starting state block lines are all the same length
    //     3) Due to #2 trailing spaces might be important, so we won't trim
    let mut num_stacks = 0;
    for line in reader.lines() {
        let line = line.unwrap();

        if reader_state == ReaderState::FirstLine {
            num_stacks = line.len() / 4 + 1;
            for _ in 0..num_stacks {
                // TODO: more efficient to deep copy one to the other, but clone isn't working as I expect?
                stacks_p1.push(VecDeque::new());
                stacks_p2.push(VecDeque::new());
            }

            reader_state = ReaderState::StartingState;
        }

        if reader_state == ReaderState::StartingState {
            if line.trim().len() == 0 {
                reader_state = ReaderState::Actions;
            }
            else {
                let line_chars = line.chars().collect::<Vec<char>>();
                for stack in 0..num_stacks {
                    if line_chars[stack * 4] == '[' {
                        let c: char = line_chars[stack * 4 + 1];
                        stacks_p1[stack].push_front(c);
                        stacks_p2[stack].push_front(c);
                    }
                }
            }
        }
        else {
            // Parse the actions: move <num> from <stack1> to <stack2>
            let words = line.split(' ').collect::<Vec<&str>>();
            assert!(words.len() == 6);

            let num_to_move: usize = words[1].parse().expect("Expected an integer");
            let stack1: usize = words[3].parse().expect("Expected an integer");
            let stack2: usize = words[5].parse().expect("Expected an integer");

            let mut tmp: VecDeque<char> = VecDeque::new();
            for _ in 0..num_to_move {
                {
                    // [Part 1]
                    let c = stacks_p1[stack1 - 1].pop_back().unwrap();
                    stacks_p1[stack2 - 1].push_back(c);
                }

                {
                    // [Part 2]
                    let c = stacks_p2[stack1 - 1].pop_back().unwrap();
                    tmp.push_front(c);
                }
            }

            // [Part 2]
            for c in tmp {
                stacks_p2[stack2 - 1].push_back(c);
            }
        }
    }

    // Expected: QNHWJVJZW
    print!("[Part 1] : ");
    for stack in &stacks_p1 {
        print!("{}", stack.back().unwrap());
    }
    println!("");

    // Expected: BPCZJLFJW
    print!("[Part 2] : ");
    for stack in &stacks_p2 {
        print!("{}", stack.back().unwrap());
    }
    println!("");
}