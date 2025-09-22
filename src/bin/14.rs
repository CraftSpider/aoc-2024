use std::cmp::Ordering;
use std::sync::Mutex;
use chumsky::Parser;
use numeric::compound::matrix::DynMatrix;
use numeric::compound::vector::Vec2;
use advent_of_code::{int_i64, Parser};

pub static GRID_SIZE: Mutex<Vec2<i64>> = Mutex::new(Vec2::new([101, 103]));

advent_of_code::solution!(14);

#[derive(Debug)]
struct Robot {
    pos: Vec2<i64>,
    velocity: Vec2<i64>
}

impl Robot {
    fn pos_after(&self, time: i64, grid: Vec2<i64>) -> Vec2<i64> {
        let new_pos = self.pos + self.velocity * time;
        Vec2::from_xy(
            new_pos.x().rem_euclid(*grid.x()),
            new_pos.y().rem_euclid(*grid.y()),
        )
    }
}

fn grid_size() -> Vec2<i64> {
    *GRID_SIZE.lock().unwrap()
}

fn parser<'a>() -> Parser!['a, Vec<Robot>] {
    use chumsky::prelude::*;

    let v = int_i64().then_ignore(just(','))
        .then(int_i64())
        .map(|(x, y)| Vec2::from_xy(x, y));

    just("p=").ignore_then(v.clone())
        .then_ignore(just(" v="))
        .then(v)
        .map(|(pos, velocity)| Robot { pos, velocity })
        .separated_by(just('\n'))
        .allow_trailing()
        .collect::<Vec<_>>()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut robots = parser().parse(input).unwrap();
    let grid_size = grid_size();
    for robot in &mut robots {
        robot.pos = robot.pos_after(100, grid_size);
    }

    let mut quadrants = [0; 4];
    for robot in robots {
        let lines = grid_size / 2;
        let x = robot.pos.x().cmp(lines.x());
        let y = robot.pos.y().cmp(lines.y());
        match (x, y) {
            (Ordering::Less, Ordering::Less) => quadrants[0] += 1,
            (Ordering::Less, Ordering::Greater) => quadrants[1] += 1,
            (Ordering::Greater, Ordering::Less) => quadrants[2] += 1,
            (Ordering::Greater, Ordering::Greater) => quadrants[3] += 1,
            _ => (),
        }
    }
    Some(quadrants.into_iter().product())
}

fn check_tree(robots: &[Robot], grid_size: Vec2<i64>) -> bool {
    let mut grid = DynMatrix::new(vec![0; *grid_size.x() as usize * *grid_size.y() as usize], *grid_size.x() as usize, *grid_size.y() as usize);
    for robot in robots {
        grid[(*robot.pos.x() as usize, *robot.pos.y() as usize)] += 1;
    }

    let mut has_row = false;
    for y in 0..(*grid_size.y() as usize) {
        let mut row_count = 0;
        for x in 0..(*grid_size.x() as usize) {
            if grid[(x, y)] > 0 {
                row_count += 1;
            } else if grid[(x, y)] == 0 {
                row_count = 0;
            }
            if row_count > 10 {
                has_row = true;
                break;
            }
        }
    }

    if has_row {
        for y in 0..(*grid_size.y() as usize) {
            for x in 0..(*grid_size.x() as usize) {
                if grid[(x, y)] > 0 {
                    print!("X")
                } else {
                    print!(".")
                }
            }
            println!()
        }
    }

    has_row
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut robots = parser().parse(input).unwrap();
    let grid_size = grid_size();

    for i in 1.. {
        for robot in &mut robots {
            robot.pos = robot.pos_after(1, grid_size);
        }

        if check_tree(&robots, grid_size) {
            return Some(i);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        *GRID_SIZE.lock().unwrap() = Vec2::new([11, 7]);
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        *GRID_SIZE.lock().unwrap() = Vec2::new([11, 7]);
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
