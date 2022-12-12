use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;


pub fn day04() {
    let filename = "dat/2022/day04.txt";

	let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut num_overlaps : u64 = 0;
    for line in reader.lines() {
        let line = line.unwrap();

        // We're just going to assume the file has the input we expect :)
        let elves = line.split(',').collect::<Vec<&str>>();

        // Going to treat the sections as bits so we can just and the elves
        // together, and if the anded value matches one of the elves then
        // that means its rooms were completely taken care of by the other

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
        if overlap == elf1 || overlap == elf2 {
            num_overlaps += 1;
        }
    }

    println!("[Part 1] Number of overlaps: {}", num_overlaps);
}