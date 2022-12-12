use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;


pub fn day01() {
    let filename = "dat/2022/day01.txt";

    let mut elves : Vec<u64> = Vec::new();
	let mut cur_elf : usize = 0;

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
    	let line = line.unwrap();
		if line.len() > 0 {
			if elves.len() == cur_elf {
				elves.push(0);
			}

            let amount : u64 = line.trim().parse().expect("Expected a number");
			elves[cur_elf] += amount;
		}
		else {
            cur_elf += 1;
		}
    }

    elves.sort_by(|a, b| b.cmp(a));

	let mut elf_sum : u64 = 0;
	let mut elves_left_for_sum : i32 = 3;
	for elf in &elves {
		if elves_left_for_sum > 0 {
			elves_left_for_sum -= 1;
			elf_sum += elf;
		}
		else {
			break;
		}
	}

	println!("[Part 2] Top elf: {}", elves[0]);
	println!("[Part 2] Top elves sum: {}", elf_sum);
}
