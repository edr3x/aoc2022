use std::path::Path;

mod solutions;
use solutions::day1;

fn solution_day1() {
    let day1_input = Path::new("./src/inputs/01.txt");

    let first_part = day1::part1(day1_input);
    let second_part = day1::part2(day1_input);

    println!(
        "First part: {:?}\nSecond part: {:?}",
        first_part, second_part
    );
}

fn main() {
    solution_day1();
}
