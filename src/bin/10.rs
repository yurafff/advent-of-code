use std::iter;advent_of_code::solution!(10);


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


#[derive(Clone, Copy, Debug)]
struct StartPositionNotFound;

#[derive(Clone, Copy, Debug)]
enum Direction {
    TOP,
    BOTTOM,
    RIGHT,
    LEFT
}

fn find_start_position(desk: &Vec<Vec<char>>) -> Result<(i32, i32), StartPositionNotFound>{
    for (ind_x, row) in desk.iter().enumerate() {
        for (ind_y, &chr) in row.iter().enumerate() {
            if chr == 'S' {
                return Ok((ind_x as i32, ind_y as i32))
            }
        }
    }
    Err(StartPositionNotFound)
}


fn find_initial_direction(desk: &Vec<Vec<char>>, (x, y): (i32, i32)) -> ((i32, i32), Direction) {
    if x > 0 && matches!(desk[(x-1) as usize][y as usize], '|' | '7' | 'F') {
        return ((x-1, y), Direction::TOP); // 0
    } else if matches!(desk[(x+1) as usize][y as usize], '|' | 'L' | 'J') {
        return ((x+1, y), Direction::BOTTOM); // 2
    } else if matches!(desk[x as usize][(y-1) as usize], '-')  {
        return ((x, y-1), Direction::LEFT) // 3
    }
    ((x, y+1), Direction::RIGHT)
}


fn walk(map: &Vec<Vec<char>>) -> (Vec<Vec<i32>>, i32){
    let mut visits_map: Vec<Vec<i32>> = map.iter().map(|_| vec![]).collect();
    let (s_x, s_y) = find_start_position(&map).unwrap();
    let ((mut pos_x, mut pos_y), mut dir) = find_initial_direction(&map, (s_x, s_y));
    let mut counter = 1;
    loop {
        visits_map[pos_x as usize].push(pos_y);
        match (map[pos_x as usize][pos_y as usize], dir) {
            ('|', Direction::TOP) => {pos_x -= 1;},
            ('|', Direction::BOTTOM) => {pos_x += 1;},
            ('-', Direction::LEFT) => {pos_y -= 1;},
            ('-', Direction::RIGHT) => {pos_y += 1;},
            ('L', Direction::BOTTOM) => {pos_y += 1; dir = Direction::RIGHT; },
            ('L', Direction::LEFT) => {pos_x -= 1; dir = Direction::TOP},
            ('J', Direction::BOTTOM) => {pos_y -= 1; dir = Direction::LEFT},
            ('J', Direction::RIGHT) => {pos_x -= 1; dir = Direction::TOP},
            ('F', Direction::TOP) => {pos_y += 1; dir = Direction::RIGHT},
            ('F', Direction::LEFT) => {pos_x += 1; dir = Direction::BOTTOM},
            ('7', Direction::RIGHT) => {pos_x += 1; dir = Direction::BOTTOM},
            ('7', Direction::TOP) => {pos_y -= 1; dir = Direction::LEFT},
            ('S', _) => break,
            (_, _) => unreachable!()
        }
        counter += 1;
    }
    (visits_map, counter)
}

pub fn part_one(input: &str) -> Option<i32> {
    let map: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect::<Vec<char>>()).collect();
    let (visits_map, counter) = walk(&map);
    Some(counter / 2)
}

pub fn part_two(input: &str) -> Option<i32> {
    let map: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect::<Vec<char>>()).collect();
    let (mut visits_map, counter) = walk(&map);
    let result = visits_map
        .iter_mut()
        .enumerate()
        .filter(|(ind, v)| v.len() > 0)
        .map(|(ind, mut v)| {
            v.sort();
            pairwise(v.iter())
                .skip(1)
                .map(|(a, b)| { b - a.unwrap() - 1})
                .enumerate()
                .filter(|(ind, val)| ind % 2 == 1)
                .map(|(ind, val)| val)
                .sum::<i32>()
        }).sum::<i32>();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6815));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
