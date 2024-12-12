use arrayvec::ArrayVec;
use std::collections::HashMap;

advent_of_code::solution!(11);

fn to_map(input: &str) -> HashMap<u64, u64> {
    input
        .trim()
        .split(" ")
        .map(|val| (val.parse().unwrap(), 1))
        .collect()
}

fn alter(val: u64) -> ArrayVec<u64, 2> {
    if val == 0 {
        ArrayVec::from_iter([1])
    } else if (val.ilog10() + 1) % 2 == 0 {
        let digits = val.ilog10() + 1;
        let power = 10u64.pow(digits / 2);
        ArrayVec::from_iter([val / power, val % power])
    } else {
        ArrayVec::from_iter([val * 2024])
    }
}

fn blink(stones: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut out = HashMap::new();
    for (stone, count) in stones {
        for new in alter(stone) {
            let val = out.entry(new).or_insert(0);
            *val += count;
        }
    }
    out
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut stones = to_map(input);
    for _ in 0..25 {
        stones = blink(stones);
    }
    Some(stones.values().copied().sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut stones = to_map(input);
    for _ in 0..75 {
        stones = blink(stones);
    }
    Some(stones.values().copied().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
