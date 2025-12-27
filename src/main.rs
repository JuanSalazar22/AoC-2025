use std::env;
use std::fs;

mod days;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day: u32 = args
        .get(1)
        .expect("Please provide a day number: cargo run -- <day>")
        .parse()
        .expect("Day must be a number");

    let input = fs::read_to_string(format!("inputs/day{:02}.txt", day))
        .unwrap_or_else(|_| panic!("Could not read input file for day {}", day));

    match day {
        1 => days::day01::solve(&input),
        2 => days::day02::solve(&input),
        3 => days::day03::solve(&input),
        4 => days::day04::solve(&input),
        5 => days::day05::solve(&input),
        6 => days::day06::solve(&input),
        7 => days::day07::solve(&input),
        8 => days::day08::solve(&input),
        9 => days::day09::solve(&input),
        10 => days::day10::solve(&input),
        11 => days::day11::solve(&input),
        12 => days::day12::solve(&input),
        13 => days::day13::solve(&input),
        14 => days::day14::solve(&input),
        15 => days::day15::solve(&input),
        16 => days::day16::solve(&input),
        17 => days::day17::solve(&input),
        18 => days::day18::solve(&input),
        19 => days::day19::solve(&input),
        20 => days::day20::solve(&input),
        21 => days::day21::solve(&input),
        22 => days::day22::solve(&input),
        23 => days::day23::solve(&input),
        24 => days::day24::solve(&input),
        25 => days::day25::solve(&input),
        _ => println!("Day {} is not valid! Please choose a day between 1 and 25.", day),
    }
}

