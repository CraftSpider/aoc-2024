use advent_of_code::{int_u32, Parser};
use chumsky::prelude::*;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

struct Input {
    ordering: Vec<(u32, u32)>,
    updates: Vec<Vec<u32>>,
}

fn parser<'a>() -> Parser!['a, Input] {
    let ordering = int_u32()
        .then_ignore(just('|'))
        .then(int_u32())
        .separated_by(just('\n'))
        .allow_trailing()
        .at_least(1)
        .collect();

    let updates = int_u32()
        .separated_by(just(','))
        .at_least(1)
        .collect()
        .separated_by(just('\n'))
        .at_least(1)
        .allow_trailing()
        .collect();

    ordering
        .then_ignore(just('\n'))
        .then(updates)
        .map(|(ordering, updates)| Input { ordering, updates })
}

fn is_valid(ordering: &HashMap<u32, HashSet<u32>>, update: &[u32]) -> bool {
    let mut seen = HashSet::new();
    update.iter().all(|val| {
        seen.insert(*val);
        // No values that must be after self are before it
        seen.intersection(ordering.get(val).unwrap_or(&HashSet::new()))
            .next()
            .is_none()
    })
}

fn reorder(ordering: &HashMap<u32, HashSet<u32>>, update: &mut [u32]) {
    let empty = HashSet::new();
    update.sort_by(|l, r| {
        let l_comes_before = ordering.get(l).unwrap_or(&empty);
        let r_comes_before = ordering.get(r).unwrap_or(&empty);

        if l_comes_before.contains(r) {
            Ordering::Less
        } else if r_comes_before.contains(l) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = parser().parse(input).unwrap();
    let ordering =
        input
            .ordering
            .into_iter()
            .fold(HashMap::<u32, HashSet<u32>>::new(), |mut acc, (l, r)| {
                acc.entry(l).or_default().insert(r);
                acc
            });

    let mut out = 0;
    for update in input.updates {
        if is_valid(&ordering, &update) {
            out += update[update.len() / 2];
        }
    }
    Some(out)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parser().parse(input).unwrap();
    let ordering =
        input
            .ordering
            .into_iter()
            .fold(HashMap::<u32, HashSet<u32>>::new(), |mut acc, (l, r)| {
                acc.entry(l).or_default().insert(r);
                acc
            });

    let mut out = 0;
    for mut update in input.updates {
        if !is_valid(&ordering, &update) {
            reorder(&ordering, &mut update);
            out += update[update.len() / 2];
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
