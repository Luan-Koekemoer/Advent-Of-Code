use std::collections::HashMap;

pub fn part_one(input: &Vec<String>) -> u32 {
    return compute_calibration(input, &HashMap::new());
}

pub fn part_two(input: &Vec<String>) -> u32 {
    let special_words_map = HashMap::from([
        ("one".to_string(), '1'),
        ("two".to_string(), '2'),
        ("three".to_string(), '3'),
        ("four".to_string(), '4'),
        ("five".to_string(), '5'),
        ("six".to_string(), '6'),
        ("seven".to_string(), '7'),
        ("eight".to_string(), '8'),
        ("nine".to_string(), '9'),
    ]);

    return compute_calibration(input, &special_words_map);
}

fn compute_calibration(input: &Vec<String>, special_words_map: &HashMap<String, char>) -> u32 {
    let mut special_words_map_reversed: HashMap<String, char> = HashMap::new();
    for (key, value) in special_words_map {
        special_words_map_reversed.insert(key.chars().rev().collect(), *value);
    }

    let mut calibrated_value = 0;
    input.iter().for_each(|calibration_line| {
        let mut chars: Vec<char> = calibration_line.chars().collect();
        let first_num = find_first_number(&chars, special_words_map);
        chars.reverse();
        let second_num = find_first_number(&chars, &special_words_map_reversed);

        calibrated_value += String::from_iter([first_num, second_num])
            .parse::<u32>()
            .unwrap();
    });
    return calibrated_value;
}

fn find_first_number(chars: &Vec<char>, wordnumber_to_number_map: &HashMap<String, char>) -> char {
    let mut current_word = String::new();
    for c in chars {
        current_word.push(c.to_owned());
        current_word = if wordnumber_to_number_map
            .keys()
            .any(|x: &String| x.starts_with(&current_word))
        {
            current_word
        } else {
            current_word.get(1..).unwrap().to_string()
        };

        if wordnumber_to_number_map.contains_key(&current_word) {
            return wordnumber_to_number_map
                .get(&current_word)
                .unwrap()
                .to_owned();
        }

        if c.is_numeric() {
            return c.to_owned();
        }
    }

    return '0';
}
