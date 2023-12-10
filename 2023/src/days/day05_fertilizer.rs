use crate::days::Day;

struct Mapper {
    mappings: Vec<MappingRanges>,
}

struct MappingRanges {
    source_start: u64,
    destination_start: u64,
    range: u64,
}

impl MappingRanges {
    fn get_dest_from_source(&self, source: u64) -> u64 {
        if self.source_start <= source && self.source_start + self.range > source {
            return self.map_shift(source);
        }
        return source;
    }

    fn map_shift(&self, source: u64) -> u64 {
        source - self.source_start + self.destination_start
    }

    fn get_dest_from_range(&self, range: (u64, u64)) -> (Vec<(u64, u64)>, Vec<(u64, u64)>) {
        let mut lookup_ranges: Vec<(u64, u64)> = Vec::new();
        let mut qualifiers: Vec<(u64, u64)> = Vec::new();

        let start = range.0;
        let end = range.1;

        let start_bound = start.max(self.source_start);
        let end_bound = start.min(self.source_start + self.range - 1);

        if start_bound < end_bound {
            qualifiers.push((self.map_shift(start_bound), self.map_shift(end_bound)));
            if start_bound > start {
                lookup_ranges.push((start, start_bound))
            }
            if end > end_bound {
                lookup_ranges.push((end_bound, end))
            }
        } else {
            qualifiers.push((start, end))
        }

        println!("{:?}", qualifiers);
        (lookup_ranges, qualifiers)
    }
}

pub struct Day05;

#[allow(unused)]
impl Day for Day05 {
    fn part_one(&self, input: &Vec<String>) -> Option<String> {
        let maps = get_mapping(input);
        let mut min_location = u64::MAX;
        for seed in input
            .first()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split(" ")
        {
            let mut source = seed.parse::<u64>().unwrap();
            for map in &maps {
                for m in &map.mappings {
                    let next = m.get_dest_from_source(source);
                    if next != source {
                        source = next;
                        break;
                    }
                }
            }

            if source < min_location {
                min_location = source;
            }
        }
        Some((min_location as u64).to_string())
    }

    fn part_two(&self, input: &Vec<String>) -> Option<String> {
        todo!("Does not work");
        let maps = get_mapping(input);
        let mut min_location = u64::MAX;
        let seeds: Vec<u64> = input
            .first()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split(" ")
            .map(|s| s.parse::<u64>().unwrap())
            .collect();

        for i in (0..seeds.len()).step_by(2) {
            let mut lookup_pairs = vec![(seeds[i], seeds[i] + seeds[i + 1] - 1)];
            let mut unfound_pairs: Vec<(u64, u64)> = Vec::new();
            let mut qualifiers: Vec<(u64, u64)> = Vec::new();

            for map in &maps {
                for m in &map.mappings {
                    for pair in &lookup_pairs {
                        let mut ret = m.get_dest_from_range(*pair);
                        unfound_pairs.append(&mut ret.0);
                        qualifiers.append(&mut ret.1);
                    }
                    lookup_pairs = unfound_pairs;
                    unfound_pairs = Vec::new();
                }
                qualifiers.append(&mut lookup_pairs); // pairs for next round of map checks
                lookup_pairs = Vec::new();
            }

            let seed_pair_min = qualifiers.iter().map(|a| a.0).min();
            if !seed_pair_min.is_none() && seed_pair_min.unwrap() < min_location {
                min_location = seed_pair_min.unwrap();
            }
        }
        Some(min_location.to_string())
    }
}

fn get_mapping(input: &Vec<String>) -> Vec<Mapper> {
    let mut mappers = Vec::<Mapper>::new();

    let mut current_mapper: Option<Mapper> = None;

    for data_line in input[1..].iter() {
        if data_line.is_empty() {
            if current_mapper.is_some() {
                mappers.push(current_mapper.unwrap());
            }
            current_mapper = None;
            continue;
        }

        if current_mapper.is_none() {
            current_mapper = Some(Mapper {
                mappings: Vec::new(),
            });
            continue;
        }
        let range_details: Vec<&str> = data_line.split(" ").collect();
        let map = MappingRanges {
            source_start: range_details[1].parse::<u64>().unwrap(),
            destination_start: range_details[0].parse::<u64>().unwrap(),
            range: range_details[2].parse::<u64>().unwrap(),
        };

        current_mapper.as_mut().unwrap().mappings.push(map);
    }
    mappers.push(current_mapper.unwrap());
    return mappers;
}
