use std::fs;
mod days;

fn main() {
    day(2);
}

fn day(day: u8) {
    let zero = if day >= 10 { "" } else { "0" };
    let path = format!("./src/data/day{zero}{day}.txt");
    // let path = "./src/data/test.txt".to_owned();

    let input = read_file_to_list(&path);

    match day {
        1 => {
            println!(
                "Challenge A: {}",
                crate::days::day01_trebuchet::part_one(&input)
            );
            println!(
                "Challenge B: {}",
                crate::days::day01_trebuchet::part_two(&input)
            );
        }
        2 => {
            println!(
                "Challenge A: {}",
                crate::days::day02_cube_conundrum::part_one(&input)
            );
            println!(
                "Challenge B: {}",
                crate::days::day02_cube_conundrum::part_two(&input)
            );
        }
        _ => println!("Invalid day"),
    }
}

fn read_file_to_list(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
