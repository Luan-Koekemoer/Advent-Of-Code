use crate::days::Day;
use std::collections::HashMap;

pub struct Day03;

impl Day for Day03 {
    fn part_one(&self, input: &Vec<String>) -> Option<String> {
        let grid: Vec<Vec<char>> = input.iter().map(|row| row.chars().collect()).collect();

        let mut sum = 0;
        for (row, char_row) in grid.iter().enumerate() {
            let mut current_engine_part_number = String::new();
            let mut is_valid_part = false;
            for (col, character) in char_row.iter().enumerate() {
                if character.is_numeric() {
                    current_engine_part_number.push(*character);

                    // check neighbours
                    for i in -1_i32..2 {
                        for j in -1_i32..2 {
                            let rx = row as i32 + i;
                            let ry = col as i32 + j;
                            is_valid_part |= !(i == 0 && j == 0)
                                && rx >= 0
                                && ry >= 0
                                && rx < char_row.len() as i32
                                && ry < grid.len() as i32
                                && grid[rx as usize][ry as usize] != '.'
                                && !grid[rx as usize][ry as usize].is_numeric()
                        }
                    }
                } else {
                    if is_valid_part && current_engine_part_number.len() > 0 {
                        sum += current_engine_part_number.parse::<u32>().unwrap();
                    }
                    is_valid_part = false;
                    current_engine_part_number = String::new();
                }
            }

            if is_valid_part && current_engine_part_number.len() > 0 {
                sum += current_engine_part_number.parse::<u32>().unwrap();
            }
        }

        return Some(sum.to_string());
    }

    fn part_two(&self, input: &Vec<String>) -> Option<String> {
        let grid: Vec<Vec<char>> = input.iter().map(|row| row.chars().collect()).collect();
        let mut gear_map: HashMap<String, Vec<u32>> = HashMap::new();

        let mut sum = 0;
        for (row, char_row) in grid.iter().enumerate() {
            let mut current_engine_part_number = String::new();
            let mut is_valid_part = false;
            let mut found_gear = String::new();
            for (col, character) in char_row.iter().enumerate() {
                if character.is_numeric() {
                    current_engine_part_number.push(*character);

                    // check neighbours
                    for i in -1_i32..2 {
                        for j in -1_i32..2 {
                            let rx = row as i32 + i;
                            let ry = col as i32 + j;
                            if !(i == 0 && j == 0)
                                && rx >= 0
                                && ry >= 0
                                && rx < char_row.len() as i32
                                && ry < grid.len() as i32
                                && grid[rx as usize][ry as usize] != '.'
                                && !grid[rx as usize][ry as usize].is_numeric()
                            {
                                if grid[rx as usize][ry as usize] == '*' {
                                    found_gear = format!("{rx},{ry}");
                                    if !gear_map.contains_key(&found_gear) {
                                        gear_map.insert(found_gear.to_owned(), Vec::new());
                                    }
                                    is_valid_part = true;
                                }
                            }
                        }
                    }
                } else {
                    if is_valid_part && current_engine_part_number.len() > 0 {
                        let part_num = current_engine_part_number.parse::<u32>().unwrap();
                        if found_gear.len() > 0 {
                            let lst = gear_map.get_mut(&found_gear).unwrap();
                            found_gear = String::new();
                            lst.push(part_num);
                        } else {
                            sum += part_num;
                        }
                    }
                    is_valid_part = false;
                    current_engine_part_number = String::new();
                }
            }

            if is_valid_part && current_engine_part_number.len() > 0 {
                let part_num = current_engine_part_number.parse::<u32>().unwrap();
                if found_gear.len() > 0 {
                    let lst = gear_map.get_mut(&found_gear).unwrap();
                    lst.push(part_num);
                } else {
                    sum += part_num;
                }
            }
        }
        let temp = gear_map.iter().filter(|k| k.1.len() > 1).map(|v| {
            let mut c = 1;
            v.1.iter().for_each(|num| c *= num);
            c
        });

        return Some((sum + temp.sum::<u32>()).to_string());
    }
}
