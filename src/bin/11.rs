#![feature(slice_range)]

use std::cmp::min;
use std::collections::VecDeque;
use itertools::Itertools;
advent_of_code::solution!(11);


fn find_star_positions(desk: &VecDeque<VecDeque<char>>) -> VecDeque<(usize, usize)> {
    let mut res: VecDeque<(usize, usize)> = VecDeque::new();
    for (ind_x, row) in desk.iter().enumerate() {
        for (ind_y, &chr) in row.iter().enumerate() {
            if chr == '#' {
                res.push_back((ind_x, ind_y));
            }
        }
    }
    res
}


fn calculate_x_distances(map: &VecDeque<VecDeque<char>>, scale_factor: usize) -> Vec<i64> {
    let mut distances: Vec<i64> = vec![];
    for row in map.iter() {
        let last_elem = distances.last().cloned().unwrap_or(0);
        if row.iter().all(|&elem| elem == '.') {
            distances.push(last_elem + scale_factor as i64);
        } else {
            distances.push(last_elem + 1)
        }
    }
    distances
}

fn calculate_y_distances(map: &VecDeque<VecDeque<char>>, scale_factor: usize) -> Vec<i64> {
    let mut distances: Vec<i64> = vec![];
    for ind in 0..map[0].len() {
        let column_empty = map.iter().all(|row| row[ind] == '.');
        let last_elem = distances.last().cloned().unwrap_or(0);
        if column_empty {
            distances.push(last_elem + scale_factor as i64);
        } else {
            distances.push(last_elem + 1)
        }
    }
    distances
}


fn calculate_distance_sums(map: &VecDeque<VecDeque<char>>, scale_factor: usize) -> i64 {
    let star_positions = find_star_positions(map);
    let x_dist = calculate_x_distances(map, scale_factor);
    let y_dist = calculate_y_distances(map, scale_factor);
    star_positions.iter().combinations(2).map(
        |coms| {
            let ((st_x, st_y), (end_x, end_y)) = (
                coms.first().unwrap(), coms.last().unwrap()
            );
            let (del_x, del_y) = ((x_dist[*st_x] - x_dist[*end_x]).abs(), (y_dist[*st_y] - y_dist[*end_y]).abs());
            min(del_x, del_y)*2 + (del_x - del_y).abs()
        }
    ).sum()
}

pub fn part_one(input: &str) -> Option<i64> {
    let map: VecDeque<VecDeque<char>> = input
        .lines()
        .map(|s| s.chars().collect::<VecDeque<char>>())
        .collect();

    let dist_sum = calculate_distance_sums(&map, 2);
    Some(dist_sum)
}

pub fn part_two(input: &str) -> Option<i64> {
    let map: VecDeque<VecDeque<char>> = input
        .lines()
        .map(|s| s.chars().collect::<VecDeque<char>>())
        .collect();

    let dist_sum = calculate_distance_sums(&map, 1000000);
    Some(dist_sum)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9445168));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(742305960572));
    }
}
