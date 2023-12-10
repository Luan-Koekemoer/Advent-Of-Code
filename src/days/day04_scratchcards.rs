use crate::days::Day;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct Day04;

impl Day for Day04 {
    fn part_one(&self, input: &Vec<String>) -> Option<String> {
        Some((get_card_points(input)
            .iter()
            .map(|x| 2_f32.powi(x - 1) as i32)
            .sum::<i32>() as u32)
            .to_string())
    }

    fn part_two(&self, input: &Vec<String>) -> Option<String> {
        let mut card_copies = HashMap::<usize, u32>::new();
        let card_points = get_card_points(input);

        for (i, _) in card_points.iter().enumerate() {
            card_copies.insert(i, 1);
        }

        for (card, points) in card_points.iter().enumerate() {
            for card_copy_nr in 0..*points {
                let card_inx: usize = 1 + card + card_copy_nr as usize;
                card_copies.insert(card_inx, card_copies[&card_inx] + 1 * card_copies[&card]);
            }
        }

        Some(card_copies.values().sum::<u32>().to_string())
    }
}

fn get_card_points(input: &Vec<String>) -> Vec<i32> {
    input
        .iter()
        .map(|card| card.split_once(": ").unwrap().1.split_once(" | ").unwrap())
        .map(|nums| {
            let winning_nums = HashSet::<&str>::from_iter(
                nums.0
                    .split(" ")
                    .filter(|value| !value.is_empty())
                    .collect::<Vec<&str>>(),
            );
            let elf_nums = HashSet::<&str>::from_iter(
                nums.1
                    .split(" ")
                    .filter(|value| !value.is_empty())
                    .collect::<Vec<&str>>(),
            );
            winning_nums.intersection(&elf_nums).count() as i32
        })
        .collect()
}
