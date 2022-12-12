use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;


pub fn day03() {
    let filename = "dat/2022/day03.txt";

	let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Going to treat compartments as bitfields since we don't care about
    // the number of items in each type:
    // Bits  1 - 26: a - z
    // Bits 27 - 52: A - Z

    // [Part 1]
    let mut priorities : u64 = 0;

    // [Part 2]
    let mut group_overlaps : u64 = 0;
    let mut grouped_elf_priorities : u64 = 0;

    let mut line_number : u64 = 1;      // We'll increment at the end of the loop
    for line in reader.lines() {
    	let line = line.unwrap();
    	let len = line.len();

    	let mut first_compartment : u64 = 0;
    	let mut second_compartment : u64 = 0;

    	let mut index : usize = 0;
    	for c in line.chars() {
    		let shift_amount = match c {
    			'a'..='z' => c as u64 - 'a' as u64 + 1,
    			'A'..='Z' => c as u64 - 'A' as u64 + 27,
    			_ => panic!("Unexpected character found in file"),
    		};

    		if index < len / 2 {
    			first_compartment |= 1 << shift_amount;
    		}
    		else {
    			second_compartment |= 1 << shift_amount;
    		}

    		index += 1;
    	}

    	let find_priorities = |value: u64| {
            let mut priorities : u64 = 0;
            for index in 1..=52 {
        		if (value & (1 << index)) != 0 {
        			priorities += index;
        		}
            }

            return priorities;
        };

        // [Part 1] Any set bits in both are overlaps
        priorities += find_priorities(first_compartment & second_compartment);

        // [Part 2] We treat both compartments as one bag (or together)
        if line_number % 3 == 1 {
            // First line so we need to set the overlaps to the current setup
            group_overlaps = first_compartment | second_compartment; 
        }
        else {
            // Not the first line so we can just and them in
            group_overlaps &= first_compartment | second_compartment;
        }

        if line_number % 3 == 0 {
            grouped_elf_priorities += find_priorities(group_overlaps);
        }

        line_number += 1;
    }

    println!("[Part 1] Sum of priorities: {}", priorities);
    println!("[Part 2] Sum of priorities: {}", grouped_elf_priorities);
}
