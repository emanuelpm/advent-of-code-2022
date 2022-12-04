mod day01;

pub fn run(day : i32) {
    match day {
        1 => day01::day01(),
        _ => panic!("Day {} not found for year 2022", day),
    }
}