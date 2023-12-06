use std::iter::{Product, zip};
use regex::Regex;
advent_of_code::solution!(6);


static NUMBERS_RE: &str = r"[0-9]+";


pub struct Race {
    time: u64,
    record: u64
}

impl Race {
    fn new(data: (&u64, &u64)) -> Self {
        Race {
            time: *data.0,
            record: *data.1
        }
    }

    fn possible_win_count(&self) -> u32 {
        return (0..=self.time)
            .map(|speed| (self.time - speed) * speed)
            .filter(|distance| distance > &self.record)
            .count() as u32;
    }
}


fn extract_numbers(line: &str) -> Vec<u64> {
    Regex::new(NUMBERS_RE)
        .unwrap()
        .find_iter(line)
        .map(|s| s.as_str())
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}

struct RacesInfo {
    races: Vec<Race>
}

impl RacesInfo {
    fn possible_win_dot_product(&self) -> u32 {
        self.races
            .iter()
            .map(|race| race.possible_win_count())
            .product()
    }

    fn new(input: &str) -> Self {
        let raw_races = input
            .lines()
            .map(extract_numbers)
            .collect::<Vec<Vec<u64>>>();
        let races: Vec<Race> = zip(&raw_races[0],&raw_races[1])
            .map(Race::new)
            .collect();

        RacesInfo { races }
    }
}



pub fn part_one(input: &str) -> Option<u32> {
    let race_info = RacesInfo::new(input);
    Some(race_info.possible_win_dot_product())
}


pub fn part_two(input: &str) -> Option<u32> {
    part_one(&input.replace(" ", ""))
}

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use super::*;

    #[test]
    fn test_part_one() {
        let now = Instant::now();
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(449550));
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
    }

    #[test]
    fn test_part_two() {
        let now = Instant::now();
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(28360140));
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
    }
}
