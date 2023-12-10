use std::env;
mod days;

mod util;
use days::{
    day01_trebuchet::Day01, day02_cube_conundrum::Day02, day03_gear_ratios::Day03,
    day04_scratchcards::Day04, day05_fertilizer::Day05, day06_wait_for_it::Day06,
    day07_camel_cards::Day07, day08_haunted_wasteland::Day08, day09_mirage_maintenance::Day09, Day,
};

pub use util::{extended_math, challenge_file_reader};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        for d in args[1..].iter() {
            let d_num = d.parse::<u8>();
            if d_num.is_ok() {
                day(d_num.unwrap());
            } else {
                println!("Invalid parameter -  {d}")
            }
        }
    }
}

fn day(day: u8) {
    let zero = if day >= 10 { "" } else { "0" };
    let path = format!("./src/data/day{zero}{day}.txt");
    // let path = "./src/data/test.txt".to_owned();

    let input = challenge_file_reader::read_file_to_list(&path);

    match day {
        1 => Day01 {}.print(day, &input),
        2 => Day02 {}.print(day, &input),
        3 => Day03 {}.print(day, &input),
        4 => Day04 {}.print(day, &input),
        5 => Day05 {}.print(day, &input),
        6 => Day06 {}.print(day, &input),
        7 => Day07 {}.print(day, &input),
        8 => Day08 {}.print(day, &input),
        9 => Day09 {}.print(day, &input),
        _ => println!("Invalid day {day}"),
    }
}
