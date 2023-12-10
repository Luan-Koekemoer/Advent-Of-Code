use crate::days::Day;
use std::collections::HashMap;

use crate::util;

pub struct Day08;


impl Day for Day08 {
    fn part_one(&self, input: &Vec<String>) -> Option<String> {
        let steps: Vec<char> = input[0].chars().collect();
        Some(find_step_count("AAA".to_string(), &steps, get_map(input), |s| s == "ZZZ").to_string())
    }

    fn part_two(&self, input: &Vec<String>) -> Option<String> {
        let steps: Vec<char> = input[0].chars().collect();
        let map = get_map(input);
        Some(util::extended_math::lcm_vec(
            map.keys()
                .filter(|k| k.ends_with("A"))
                .map(|s| {
                    find_step_count(s.to_string(), &steps, get_map(input), |s| s.ends_with("Z")) as u64
                })
                .collect(),
        ).to_string())
    }
}

fn get_map(input: &Vec<String>) -> HashMap<String, (String, String)> {
    HashMap::from_iter(
        input[2..]
            .iter()
            .map(|line| {
                let mapping = line.split_once(" = ").unwrap();
                (mapping.0.to_owned(), mapping.1.to_owned())
            })
            .map(|left_right| {
                let str_val = left_right.1.replace("(", "").replace(")", "");
                let return_val = str_val.split_once(", ").unwrap();
                (
                    left_right.0,
                    (return_val.0.to_owned(), return_val.1.to_owned()),
                )
            }),
    )
}

fn find_step_count(
    start: String,
    steps: &Vec<char>,
    map: HashMap<String, (String, String)>,
    end_condition: fn(current: &String) -> bool,
) -> u32 {
    let mut current_step_inx = 0;
    let mut next_move = start;

    let mut count = 0;
    while !end_condition(&next_move) {
        let step_right = steps[current_step_inx] == 'R';

        count += 1;
        next_move = if step_right {
            map[&next_move].1.to_owned()
        } else {
            map[&next_move].0.to_owned()
        };

        current_step_inx += 1;
        if current_step_inx == steps.len() {
            current_step_inx = 0;
        }
    }

    count
}
