use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;


pub fn day02() {
    let filename = "dat/2022/day02.txt";

	let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut p1_total_score : u64 = 0;
    let mut p2_total_score : u64 = 0;
    for line in reader.lines() {
    	let line : String = line.unwrap();
    	let opponent = line.as_bytes()[0] as char;
    	let me = line.as_bytes()[2] as char;

    	// Part 1
    	let p1_round_score = match me {
    		'X' => 1 + match opponent {
    			'A' => 3,
    			'B' => 0,
    			'C' => 6,
    			_ => panic!("Unexpected input character found"),
    		},
    		'Y' => 2 + match opponent {
    			'A' => 6,
    			'B' => 3,
    			'C' => 0,
    			_ => panic!("Unexpected input character found"),
    		},
    		'Z' => 3 + match opponent {
    			'A' => 0,
    			'B' => 6,
    			'C' => 3,
    			_ => panic!("Unexpected input character found"),
    		},
    		_ => panic!("Unexpected input character found"),
    	};

    	p1_total_score += p1_round_score;

    	// Part 2
    	let p2_round_score = match me {
    		'X' => 0 + match opponent {
    			'A' => 3,
    			'B' => 1,
    			'C' => 2,
    			_ => panic!("Unexpected input character found"),
    		},
    		'Y' => 3 + match opponent {
    			'A' => 1,
    			'B' => 2,
    			'C' => 3,
    			_ => panic!("Unexpected input character found"),
    		},
    		'Z' => 6 + match opponent {
    			'A' => 2,
    			'B' => 3,
    			'C' => 1,
    			_ => panic!("Unexpected input character found"),
    		},
    		_ => panic!("Unexpected input character found"),
    	};

    	p2_total_score += p2_round_score;
    }

    println!("[Part 1] Total score: {}", p1_total_score); // 13526
    println!("[Part 2] Total score: {}", p2_total_score); // 14204
}
