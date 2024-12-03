use advent_of_code::Parser;
use chumsky::prelude::*;
use std::str::FromStr;

advent_of_code::solution!(3);

fn parser<'a>() -> Parser!['a, Vec<(u32, u32)>] {
    let int = text::digits(10)
        .at_least(1)
        .at_most(3)
        .to_slice()
        .map(u32::from_str)
        .unwrapped();
    let valid_instr = just("mul(")
        .ignore_then(int)
        .then_ignore(just(','))
        .then(int)
        .then_ignore(just(')'))
        .map(Some);

    valid_instr
        .or(any().to(None))
        .repeated()
        .collect::<Vec<_>>()
        .map(|v| v.into_iter().filter_map(|a| a).collect::<Vec<_>>())
}

pub fn part_one(input: &str) -> Option<u32> {
    let out = parser().parse(input).unwrap();
    Some(out.into_iter().map(|(a, b)| a * b).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
