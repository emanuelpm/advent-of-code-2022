mod day01;
mod day02;
mod day03;
mod day04;


pub fn run(day : i32) {
    match day {
        1 => day01::day01(),
        2 => day02::day02(),
        3 => day03::day03(),
        4 => day04::day04(),
        _ => panic!("Day {} not found for year 2022", day),
    }
}