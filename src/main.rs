use std::env;

mod aoc2022;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Expected 2 arguments: <year> <day>."); 
    }

    let year = args[1].trim().parse().expect("Year must be an integer.");
    let day = args[2].trim().parse().expect("Day must be an integer.");

    match year {
        2022 => aoc2022::run(day),
        _ => panic!("No programs found for year {}", year),
    }
}
