use arrayvec::ArrayVec;
use std::collections::HashMap;

advent_of_code::solution!(11);

fn to_line(input: &str) -> Vec<u64> {
    input
        .trim()
        .split(" ")
        .map(|val| val.parse().unwrap())
        .collect()
}

fn blink(stones: &mut Vec<u64>) {
    let mut idx = 0;
    while idx < stones.len() {
        let val = stones[idx];
        if val == 0 {
            stones[idx] = 1;
        } else if (val.ilog10() + 1) % 2 == 0 {
            let digits = val.ilog10() + 1;
            let power = 10u64.pow(digits / 2);
            stones[idx] = val / power;
            // This is the expensive part. It keeps needing to shift everything over by one
            stones.insert(idx + 1, val % power);
            idx += 1;
        } else {
            stones[idx] *= 2024;
        }
        idx += 1;
    }
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

fn blink_fast(stones: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut out = HashMap::new();
    for (stone, count) in stones {
        for new in alter(stone) {
            let val = out.entry(new).or_insert(0);
            *val += count;
        }
    }
    out
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut stones = to_line(input);
    for _ in 0..25 {
        blink(&mut stones);
    }
    Some(stones.len() as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut stones = HashMap::from_iter(to_line(input).into_iter().map(|v| (v, 1)));
    for _ in 0..75 {
        stones = blink_fast(stones);
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
    fn test_example_2() {
        let mut init = to_line(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        blink(&mut init);
        assert_eq!(&init, &[1, 2024, 1, 0, 9, 9, 2021976]);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
