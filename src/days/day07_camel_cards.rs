use crate::days::Day;
use std::{collections::HashMap, cmp::Ordering};

struct CamelCard {
    strenght: u32,
}

impl CamelCard {
    fn new(name: char, j_is_joker: bool) -> CamelCard {
        let strenght = match name {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => if j_is_joker { 1 } else { 11 },
            'T' => 10,
            _ => name.to_string().parse::<u32>().unwrap(),
        };

        CamelCard { strenght }
    }
}

pub struct Day07;

impl Day for Day07 {
    fn part_one(&self, input: &Vec<String>) -> Option<String> {
        Some(sort_and_compute_winnings(input, false).to_string())
    }


    fn part_two(&self, input: &Vec<String>) -> Option<String> {
        Some(sort_and_compute_winnings(input, true).to_string())
    }
}

fn determine_hand_rank(hand: &Vec<CamelCard>, j_is_joker: bool) -> u32 {
    let mut known_cards = HashMap::<u32, u32>::new();
    for card in hand {
        if !known_cards.contains_key(&card.strenght) {
            known_cards.insert(card.strenght, 0);
        }
        known_cards.insert(card.strenght, known_cards[&card.strenght] + 1);
    }

    if j_is_joker && known_cards.contains_key(&1) {
        let jokers = known_cards[&1];
        known_cards.remove(&1);

        if known_cards.len() > 0 {
            let key_with_max_value = *known_cards.iter().max_by_key(|entry | entry.1).unwrap().0;
            known_cards.insert(key_with_max_value, known_cards[&key_with_max_value] + jokers) ;
        } else {
            known_cards.insert(1, jokers) ;
        }
    }

    let vals = Vec::from_iter(known_cards.values());

    if vals.contains(&&5) {
        return 6;
    } else if vals.contains(&&4) {
        return 5;
    } else if vals.contains(&&3) && vals.contains(&&2) {
        return 4;
    } else if vals.contains(&&3) {
        return 3;
    } else if vals.iter().filter(|x| ***x == 2_u32).count() == 2 {
        return 2;
    } else if vals.contains(&&2) {
        return 1;
    } else {
        return 0;
    }
}

fn sort_and_compute_winnings(input: &Vec<String>, j_is_joker: bool) -> u32 {
    let mut plays: Vec<(u32, u32, Vec<CamelCard>)> = input
        .iter()
        .map(|h| h.split_once(" ").unwrap())
        .map(|t| {
            let hand : Vec<CamelCard> = t.0.chars().map(|c| CamelCard::new(c, j_is_joker)).collect();
            (
                t.1.parse::<u32>().unwrap(),
                determine_hand_rank(&hand, j_is_joker),
                hand
            )
        })
        .collect();
    // Sort by card strength
    plays.sort_by(|a, b| {
        let cards_a = &a.2;
        let cards_b = &b.2;
        for i in 0..5 {
            if cards_a[i].strenght > cards_b[i].strenght {
                return Ordering::Greater;
            } else if cards_a[i].strenght < cards_b[i].strenght {
                return Ordering::Less;
            }
        }
        Ordering::Equal
    });

    // Sort by hand value
    plays.sort_by(|a, b| a.1.cmp(&b.1));

    let mut limit = 0_u32;
    plays.iter().map(|a| {limit += 1; a.0 * &limit}).sum()
}
