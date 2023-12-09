use std::io::Read;
use std::{iter};
advent_of_code::solution!(9);


fn interpolate(input_nums: Vec<i32>) -> i32 {
    let mut triangle: Vec<Vec<i32>> = vec![input_nums];
    while !triangle.last().unwrap().iter().all(|&num| num == 0) {
        let result = pairwise(triangle.last().unwrap())
            .skip(1)
            .map(|(a, b)| b - a.unwrap())
            .collect();
        triangle.push(result)
    }
    let triangle_len = triangle.len();
    triangle[triangle_len-1].push(0);
    for ind in (0..&triangle.len()-1).rev() {
        let &prev_last = triangle[ind].last().unwrap();
        let &last = triangle[ind+1].last().unwrap();
        triangle[ind].push(prev_last + last);
    }
    *triangle[0].last().unwrap()
}


fn pairwise<I>(right: I) -> impl Iterator<Item = (Option<I::Item>, I::Item)>
    where
        I: IntoIterator + Clone,
{
    let left = iter::once(None)
        .chain(right.clone()
        .into_iter()
        .map(Some));
    left.zip(right)
}


fn parse_line(line: &str) -> Vec<i32> {
    line
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

pub fn part_one(input: &str) -> Option<i32> {
    Some(input
        .lines()
        .map(parse_line)
        .map(interpolate)
        .sum())
}

pub fn part_two(input: &str) -> Option<i32> {
    Some(input
        .lines()
        .map(parse_line)
        .map(|mut nums| {nums.reverse(); nums})
        .map(interpolate)
        .sum())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1993300041));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1038));
    }
}
