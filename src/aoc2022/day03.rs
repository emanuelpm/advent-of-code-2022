use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;


pub fn day03() {
    let filename = "dat/2022/day03.txt";

	let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut priorities : u64 = 0;
    for line in reader.lines() {
    	let line = line.unwrap();
    	let len = line.len();

    	// Going to treat compartments as bitfields since we don't care about
    	// the number of items in each type:
    	// Bits  1 - 26: a - z
    	// Bits 27 - 52: A - Z
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

    	// Any set bits in both are overlaps
    	let compartment_overlap : u64 = first_compartment & second_compartment;

    	for index in 1..=52 {
    		if (compartment_overlap & (1 << index)) != 0 {
    			priorities += index;
    		}
    	}
    }

    println!("[Part 1] Sum of priorities: {}", priorities);
}
