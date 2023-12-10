use crate::days::Day;
pub struct Day06;

impl Day for Day06 {
    
fn part_one(&self, input: &Vec<String>) -> Option<String> {
        let times : Vec<&str> = input[0].split_once(": ").unwrap().1.split(" ").filter(|x| !x.is_empty()).collect();
        let distances : Vec<&str> = input[1].split_once(": ").unwrap().1.split(" ").filter(|x| !x.is_empty()).collect();
        let mut margin_of_error = 1_u32;

        for i in 0..times.len() {
            let time_limit = times[i].parse::<u32>().unwrap();
            let record_distance = distances[i].parse::<u32>().unwrap();
            let mut distances = Vec::<u32>::new(); 
            for time in 0..time_limit {
                let velocity = time;
                let distance = velocity * (time_limit - time);
                distances.push(distance);
            }
            margin_of_error *= distances.iter().filter(|x| {x > &&record_distance}).count() as u32;
        }

        Some(margin_of_error.to_string())
    }

    fn part_two(&self, input: &Vec<String>) -> Option<String> {
        let time : u64 = input[0].split_once(": ").unwrap().1.replace(" ", "").parse::<u64>().unwrap();
        let distance : u64 = input[1].split_once(": ").unwrap().1.replace(" ", "").parse::<u64>().unwrap();

        let mut distances = Vec::<u64>::new(); 
        for t in 0..time {
            let velocity = t;
            let distance = velocity * (time - t);
            distances.push(distance);
        }

        Some((distances.iter().filter(|x| x > &&distance).count() as u32).to_string())
    }
}
