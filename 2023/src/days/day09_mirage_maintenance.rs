use crate::days::Day;

pub struct Day09;

impl Day for Day09 {
    fn part_one(&self, input: &Vec<String>) -> Option<String> {
        Some(
            input
                .iter()
                .map(|line| {
                    get_next_num_in_sequence(
                        line.split(" ").map(|v| v.parse::<i64>().unwrap()).collect(),
                    )
                })
                .sum::<i64>()
                .to_string(),
        )
    }

    fn part_two(&self, input: &Vec<String>) -> Option<String> {
        Some(
            input
                .iter()
                .map(|line| {
                    let mut line_rev: Vec<i64> =
                        line.split(" ").map(|v| v.parse::<i64>().unwrap()).collect();
                    line_rev.reverse();
                    get_next_num_in_sequence(line_rev)
                })
                .sum::<i64>()
                .to_string(),
        )
    }
}

fn get_next_num_in_sequence(sequence: Vec<i64>) -> i64 {
    let mut series_pyramid: Vec<Vec<i64>> = Vec::new();
    series_pyramid.push(sequence.to_owned());

    let mut diff_line: Vec<i64> = get_diff_line(&sequence);
    while !diff_line.iter().all(|v| *v == 0_i64) {
        series_pyramid.push(diff_line.to_owned());
        diff_line = get_diff_line(&diff_line);
    }

    let mut next_value = 0_i64;
    series_pyramid.reverse();
    for series in &series_pyramid {
        next_value += series.last().unwrap();
    }
    next_value
}

fn get_diff_line(sequence: &Vec<i64>) -> Vec<i64> {
    let mut next_row: Vec<i64> = Vec::new();
    for i in 0..sequence.len() - 1 {
        next_row.push(sequence[i + 1] - sequence[i])
    }
    next_row
}
