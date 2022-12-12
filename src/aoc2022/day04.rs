use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;


pub fn day04() {
    let filename = "dat/2022/day04.txt";

	let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut num_total_overlaps : u64 = 0;
    let mut num_any_overlaps : u64 = 0;
    for line in reader.lines() {
        let line = line.unwrap();

        // We're just going to assume the file has the input we expect :)
        let elves = line.split(',').collect::<Vec<&str>>();

        // Going to treat the sections as bits so we can just and the elves
        // together

        // [Part 1] if the anded value matches one of the elves then
        // that means its rooms were completely taken care of by the other

        // [Part 2] if the anded value isn't 0 there is some overlap

        // First elf (left side)
        let range = elves[0].split('-').collect::<Vec<&str>>();
        let low : u64 = range[0].parse().expect("Expected an integer");
        let high : u64 = range[1].parse().expect("Expected an integer");
        let mut elf1 : u128 = 0; 
        assert!(low < 128 && high < 128);
        for bit in low..=high {
            elf1 |= 1 << bit;
        }

        // Second elf (right side)
        let range = elves[1].split('-').collect::<Vec<&str>>();
        let low : u64 = range[0].parse().expect("Expected an integer");
        let high : u64 = range[1].parse().expect("Expected an integer");
        let mut elf2 : u128 = 0; 
        assert!(low < 128 && high < 128);
        for bit in low..=high {
            elf2 |= 1 << bit;
        }

        let overlap : u128 = elf1 & elf2;

        // [Part 1] Checking for a full overlap
        if overlap == elf1 || overlap == elf2 {
            num_total_overlaps += 1;
        }

        // [Part 2] Checking for any overlap
        if overlap != 0 {
            num_any_overlaps += 1;
        }
    }

    println!("[Part 1] Number of total overlaps: {}", num_total_overlaps);
    println!("[Part 2] Number of partial overlaps: {}", num_any_overlaps);
}