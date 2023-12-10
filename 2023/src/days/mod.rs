pub mod day01_trebuchet;
pub mod day02_cube_conundrum;
pub mod day03_gear_ratios;
pub mod day04_scratchcards;
pub mod day05_fertilizer;
pub mod day06_wait_for_it;
pub mod day07_camel_cards;
pub mod day08_haunted_wasteland;
pub mod day09_mirage_maintenance;
pub mod day10_pipe_maze;

pub trait Day {
    fn part_one(&self, input: &Vec<String>) -> Option<String>;
    fn part_two(&self, input: &Vec<String>) -> Option<String>;
    fn print(&self, day: u8, input: &Vec<String>){
        println!("\nDay {day}");
        println!("-----------------------------");
        println!("Part 1: {}", self.part_one(input).expect("Failed to run Part 1 - day {day}"));
        println!("Part 2: {}", self.part_two(input).expect("Failed to run Part 1 - day {day}"));
        println!("-----------------------------\n");
    }
}
