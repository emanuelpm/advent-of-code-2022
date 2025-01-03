mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;


pub fn run(day : i32) {
    // #TODO: Some clever way to do this without writing out each case?
    match day {
        1 => day01::day01(),
        2 => day02::day02(),
        3 => day03::day03(),
        4 => day04::day04(),
        5 => day05::day05(),
        6 => day06::day06(),
        7 => day07::day07(),
        _ => panic!("Day {} not found for year 2022", day),
    }
}