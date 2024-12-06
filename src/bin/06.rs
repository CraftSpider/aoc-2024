use numeric::compound::vector::Vec2;
use numeric::traits::ops::checked::{CheckedAdd, CheckedSub};
use std::collections::HashSet;

advent_of_code::solution!(6);

#[derive(Default)]
struct Map {
    blocks: HashSet<Vec2<u32>>,
    size: Vec2<u32>,
    guard: Vec2<u32>,
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn rotate_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

impl Map {
    fn from_input(input: &str) -> Map {
        let mut map = Map::default();

        let mut size = Vec2::default();
        for (y, line) in input.rsplit('\n').enumerate() {
            let y = y as u32;

            if y > *size.y() {
                size.set_y(y);
            }
            for (x, char) in line.chars().enumerate() {
                let x = x as u32;

                if x > *size.x() {
                    size.set_x(x);
                }
                if char == '^' {
                    map.guard = Vec2::new([x, y]);
                } else if char == '#' {
                    map.blocks.insert(Vec2::new([x, y]));
                }
            }
        }
        map.size = size;

        map
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = Map::from_input(input);

    let mut visited = HashSet::new();
    let mut direction = Direction::Up;
    loop {
        visited.insert(map.guard);
        let next = match direction {
            Direction::Up => map.guard.checked_add(Vec2::new([0, 1])),
            Direction::Down => map.guard.checked_sub(Vec2::new([0, 1])),
            Direction::Right => map.guard.checked_add(Vec2::new([1, 0])),
            Direction::Left => map.guard.checked_sub(Vec2::new([1, 0])),
        };
        let Some(next) = next else { break };
        if next.x() > map.size.x() || next.y() > map.size.y() {
            break;
        }
        if map.blocks.contains(&next) {
            direction = direction.rotate_right();
        } else {
            map.guard = next;
        }
    }
    Some(visited.len() as u32)
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
