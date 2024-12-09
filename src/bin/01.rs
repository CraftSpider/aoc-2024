advent_of_code::solution!(1);

use advent_of_code::{int_u32, Parser};
use chumsky::prelude::*;
use std::collections::HashMap;

fn parser<'a>() -> Parser!['a, Vec<(u32, u32)>] {
    int_u32()
        .padded()
        .then(int_u32())
        .separated_by(just('\n'))
        .allow_trailing()
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let inp = parser().parse(input).unwrap();
    let (mut left, mut right): (Vec<_>, Vec<_>) = inp.into_iter().unzip();
    left.sort();
    right.sort();
    let out = left
        .into_iter()
        .zip(right)
        .map(|(l, r)| u32::abs_diff(l, r))
        .sum();
    Some(out)
}

pub fn part_two(input: &str) -> Option<u32> {
    let inp = parser().parse(input).unwrap();
    let (left, right): (Vec<_>, Vec<_>) = inp.into_iter().unzip();
    let right = right
        .into_iter()
        .fold(HashMap::<_, u32>::new(), |mut map, val| {
            let entry = map.entry(val).or_default();
            *entry += 1;
            map
        });
    let out = left
        .into_iter()
        .map(|a| a * right.get(&a).copied().unwrap_or_default())
        .sum();
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
