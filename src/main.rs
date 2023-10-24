use std::env;

mod aoc2022;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Expected 1 argument: <day>.");
    }

    let day = args[1].trim().parse().expect("Day must be an integer.");
    aoc2022::run(day);
}
