use std::collections::VecDeque;
use std::ops::{Add, AddAssign};
use num::{BigInt, Num, Signed, ToPrimitive};
use num::bigint::ToBigInt;
use num::complex::ComplexFloat;

advent_of_code::solution!(18);



struct Polygon<T: Num> {
    points: VecDeque<(T, T)>,
    sides_sum: T
}

trait Area {
    type AreaValue;

    fn area(&self) -> Self::AreaValue;
}

impl Area for Polygon<i32> {
    type AreaValue = BigInt;

    fn area(&self) -> BigInt {
        let mut res = 0.to_bigint().unwrap();
        for i in 0..self.points.len() {
            let y_prev_ind = if i as i32 - 1 < 0 { self.points.len() - 1 } else { i - 1 };
            let y_prev = self.points[y_prev_ind].1.to_bigint().unwrap();
            let y_next_ind = (i + 1) % self.points.len();
            let y_next = self.points[y_next_ind].1.to_bigint().unwrap();
            let x_current = self.points[i].0.to_bigint().unwrap();
            res += (x_current*(y_prev - y_next));
        }
        res.abs() / 2 + self.sides_sum.to_bigint().unwrap() / 2 + 1
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Right,
    Left
}

#[derive(Debug)]
struct ParseDirectionError;


impl TryFrom<char> for Direction {
    type Error = ParseDirectionError;

    fn try_from(data: char) -> Result<Direction, Self::Error>{
        match data {
            'U' => Ok(Direction::Up),
            'D' => Ok(Direction::Down),
            'R' => Ok(Direction::Right),
            'L' => Ok(Direction::Left),
            '0' => Ok(Direction::Right),
            '1' => Ok(Direction::Down),
            '2' => Ok(Direction::Left),
            '3' => Ok(Direction::Up),
            _ => Err(ParseDirectionError)
        }
    }
}


impl Direction {
    fn as_tuple(&self) -> (i32, i32) {
        match self {
            Direction::Up => (1, 0),
            Direction::Down => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Left => (0, -1)
        }
    }
}


impl From<VecDeque<(Direction, i32)>> for Polygon<i32>{
    fn from(value: VecDeque<(Direction, i32)>) -> Polygon<i32> {
        let mut points: VecDeque<(i32, i32)> = VecDeque::from([(0, 0)]);

        let (mut x, mut y)  = (0, 0);
        let mut sides_sum = 0;
        for &(direction, delta) in value.iter().rev() {
            let (delta_x, delta_y) = direction.as_tuple();
            let (new_x, new_y) = (x + delta_x*(delta), y + delta_y*(delta));
            sides_sum += delta;
            points.push_back((new_x, new_y));
            (x, y) = (new_x, new_y);
        }
        Polygon { points, sides_sum }
    }
}



pub fn part_one(input: &str) -> Option<i32> {
    let data: VecDeque<(Direction, i32)> = input
        .lines()
        .map(|s| s.split_whitespace().collect::<Vec<&str>>())
        .map(|s| (
            Direction::try_from(s[0].chars().next().unwrap()).unwrap(),
            s[1].parse::<i32>().unwrap(),
        ))
        .collect();
    let polygon = Polygon::from(data);
    polygon.area().to_i32()
}

fn parse_color(color: &str) -> (Direction, i32) {
    let delta_str = &color[2..=(color.len() - 3)];
    let delta = i32::from_str_radix(delta_str, 16).unwrap();
    let color_chr = color.chars().rev().nth(1).unwrap();
    let direction = Direction::try_from(color_chr).unwrap();
    (direction, delta)
}

pub fn part_two(input: &str) -> Option<BigInt> {
    let data: VecDeque<(Direction, i32)> = input
        .lines()
        .map(|s| s.split_whitespace().collect::<Vec<&str>>())
        .map(|s| parse_color(s[2]))
        .collect();
    let polygon = Polygon::from(data);
    Some(polygon.area())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31171));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(BigInt::from_str_radix("131431655002266", 10).unwrap()));
    }
}
