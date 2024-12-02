use advent_of_code::Parser;
use chumsky::prelude::*;
use std::str::FromStr;

advent_of_code::solution!(2);

#[derive(Copy, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
}

fn validate(report: &[u32]) -> Result<(), usize> {
    let dir = if report[0] > report[1] {
        Direction::Down
    } else {
        Direction::Up
    };
    report.windows(2).enumerate().try_for_each(|(idx, vals)| {
        let &[a, b] = vals else { panic!() };
        let diff = u32::abs_diff(a, b);
        if diff < 1 || diff > 3 {
            return Err(idx);
        }
        let cond = match dir {
            Direction::Up => a < b,
            Direction::Down => a > b,
        };
        cond.then_some(()).ok_or(idx)
    })
}

fn parser<'a>() -> Parser!['a, Vec<Vec<u32>>] {
    text::int(10)
        .map(u32::from_str)
        .unwrapped()
        .separated_by(just(' '))
        .at_least(1)
        .collect()
        .separated_by(just('\n'))
        .allow_trailing()
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let reports = parser().parse(input).unwrap();

    let count = reports
        .iter()
        .filter(|&vals| validate(vals).is_ok())
        .count();
    Some(count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports = parser().parse(input).unwrap();

    let count = reports
        .into_iter()
        .filter_map(|mut vals| match validate(&vals) {
            Ok(_) => Some(()),
            Err(fail_idx) => {
                // Because maybe the first value set the wrong direction
                let maybe_first = if fail_idx == 1 {
                    validate(&vals[1..]).is_ok()
                } else {
                    false
                };
                // If it's not the direction, it must be either the left or right value that can be
                // removed
                let a = {
                    let mut val = vals.clone();
                    val.remove(fail_idx);
                    validate(&val).is_ok()
                };
                let b = {
                    let mut val = vals.clone();
                    val.remove(fail_idx + 1);
                    validate(&val).is_ok()
                };
                (maybe_first || a || b).then_some(())
            }
        })
        .count();
    Some(count as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
