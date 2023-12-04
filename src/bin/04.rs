use std::collections::{HashSet, VecDeque};
use std::convert::TryFrom;

advent_of_code::solution!(4);

#[derive(Debug, Clone)]
struct ParseError;


#[derive(Debug)]
struct Scratchcard<'crd> {
    winning_numbers: HashSet<&'crd str>,
    numbers: HashSet<&'crd str>,
    overlap: usize
}


trait Parse<T>: Sized {
    type Error;
    fn parse_str(value: T) -> Result<Self, Self::Error>;
}


impl <'crd> Parse<&'crd str> for Scratchcard<'crd> {
    type Error = ParseError;

    fn parse_str(input: &'crd str) -> Result<Scratchcard, Self::Error> {
        let &card_info = input.split(": ").collect::<Vec<&str>>().last().unwrap();
        let card_info_split: Vec<&str> = card_info.split(" | ").take(2).collect();
        let [winning_str, have_str] = <[&str; 2]>::try_from(card_info_split).ok().unwrap();
        let winning_numbers: HashSet<&str> = winning_str.trim().split_whitespace().collect();
        let numbers: HashSet<&str> = have_str.trim().split_whitespace().collect();
        let overlap = winning_numbers.intersection(&numbers).count();
        Ok(Scratchcard {winning_numbers, numbers, overlap })
    }
}

trait ScratchcardsWorth {
    fn worth(&self) -> u32;
}

impl <'crd> Parse<&'crd str> for Vec<Scratchcard<'crd>> {
    type Error = ParseError;

    fn parse_str(value: &'crd str) -> Result<Self, Self::Error> {
        let mut scratchcards: Vec<Scratchcard> = vec![];
        for line in value.lines() {
            scratchcards.push(Scratchcard::parse_str(line).unwrap())
        }
        Ok(scratchcards)
    }
}

impl ScratchcardsWorth for Vec<Scratchcard<'_>> {
    fn worth(&self) -> u32 {
        self.iter()
            .map(|card| card.overlap)
            .filter(|&overlap| overlap > 0)
            .map(|overlap| u32::pow(2, (overlap-1) as u32))
            .sum()
    }
}


pub fn part_one(input: &str) -> Option<u32> {
    let mut scratchcards: Vec<Scratchcard> = Vec::parse_str(input).unwrap();
    Some(scratchcards.worth())
}

pub fn part_two(input: &str) -> Option<u32> {
    let scratchcards: Vec<Scratchcard> = Vec::parse_str(input).unwrap();
    let mut stack: VecDeque<(usize, &Scratchcard)> = scratchcards.iter().enumerate().collect();
    let mut res = 0u32;
    while stack.len() > 0 {
        let (ind, card) = stack.pop_front().unwrap();
        res += 1;
        if card.overlap == 0 || ind == (scratchcards.len() - 1){
            continue;
        }
        for new_ind in (ind+1..=ind+card.overlap).rev() {
            stack.push_front((new_ind, &scratchcards[new_ind]))
        }
    }
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(20667));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5833065));
    }
}
