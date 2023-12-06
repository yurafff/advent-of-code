advent_of_code::solution!(1);


static RADIX: u32 = 10;
static WORDS: [&str; 9] = [
    "one", "two", "three",
    "four", "five", "six",
    "seven", "eight", "nine"
];


fn parse_number_2(ind: usize, chr: char, line: &str) -> Option<u32> {
    match chr {
        '0'..='9' => chr.to_digit(RADIX),
        _ => WORDS
            .iter()
            .enumerate()
            .find_map(|(i, word)| {
                line[ind..].starts_with(word).then_some(i as u32 + 1u32)
            })
    }
}

fn parse_number(ind: usize, chr: char, line: &str) -> Option<u32> {
    if chr.is_ascii_digit() {
        return chr.to_digit(RADIX);
    }
    return None
}

fn parse_line_digits(
    line: &str,
    parse_func: fn(usize, char, &str) -> Option<u32>
) -> Vec<u32> {
    line
        .chars()
        .enumerate()
        .map(|(ind, chr)| parse_func(ind, chr, line))
        .filter(|num| num.is_some())
        .map(|num| num.unwrap())
        .collect()
}


pub fn part_one(input: &str) -> Option<u32> {
    Some(input
        .lines()
        .map(|line| {
            let digits = parse_line_digits(line, parse_number);
            10*digits[0] + digits[digits.len() - 1]
        })
        .sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input
        .lines()
        .map(|line| {
            let digits = parse_line_digits(line, parse_number_2);
            10*digits[0] + digits[digits.len() - 1]
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(54597));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(54504));
    }
}
