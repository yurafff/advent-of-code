use std::collections::{HashMap, HashSet};
use regex::Regex;
advent_of_code::solution!(8);


pub fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}


#[derive(Debug)]
struct GraphWalk<'game> {
    graph: HashMap<&'game str, (&'game str, &'game str)>,
    visited: HashSet<&'game str>,
    instructions: &'game str,
    validation_pattern: Regex
}


impl <'game> GraphWalk<'game> {
    fn new(input: &'game str, validation_pattern: Regex) -> Self {
        let data: Vec<&str> = input.lines().collect();
        let instructions = data[0].trim();
        let graph = data[2..]
            .iter()
            .map(|&s| (&s[0..3], (&s[7..10], &s[12..15])))
            .fold(
                HashMap::new(),
                |mut map, (key, val)| {map.entry(key).or_insert(val); map}
            );
        let visited = HashSet::new();
        let walk = GraphWalk {
            graph, visited, instructions, validation_pattern
        };
        walk
    }

    fn find_steps(&mut self, start_node: &'game str) -> usize {
        let mut node = start_node;
        for (ind, instr) in self.instructions.chars().cycle().enumerate() {
            if self.validation_pattern.is_match(node) {
                self.visited.insert(node);
                return ind;
            }
            node = if instr == 'L' { self.graph[node].0 } else { self.graph[node].1 };
        }
        0usize
    }

    fn ghost_walk(&mut self) -> usize {
        let keys: Vec<&str> = self.graph.keys().map(|&k| k).collect();

        keys
            .iter()
            .filter(|&&node| node.ends_with('A'))
            .map(|&node| self.find_steps(node))
            .into_iter()
            .reduce(lcm)
            .unwrap()
    }
}


pub fn part_one(input: &str) -> Option<usize> {
    Some(GraphWalk::new(input, Regex::new(r"ZZZ").unwrap()).find_steps("AAA"))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(GraphWalk::new(input, Regex::new(r"[A-Z][A-Z]Z").unwrap()).ghost_walk())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12361usize));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18215611419223usize));
    }
}
