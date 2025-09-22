use std::fmt;
use std::fmt::Write;
use chumsky::Parser;
use numeric::compound::matrix::DynMatrix;
use numeric::compound::vector::Vec2;
use advent_of_code::{Parser, Cardinal, VecExt};

advent_of_code::solution!(15);

#[derive(Debug, Copy, Clone, PartialEq)]
enum Tile {
    Empty,
    Robot,
    Wall,
    Box,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tile::Empty => f.write_char('.'),
            Tile::Robot => f.write_char('@'),
            Tile::Wall => f.write_char('#'),
            Tile::Box => f.write_char('O'),
        }
    }
}

struct Warehouse {
    map: DynMatrix<Tile>,
    robot: Vec2<usize>,
}

impl Warehouse {
    fn new(map: Vec<Vec<Tile>>) -> Warehouse {
        let cols = map.len();
        let rows = map[0].len();

        let robot = map.iter().rev().enumerate()
            .find_map(|(i, row)| {
                let j = row.iter().enumerate().find_map(|(j, tile)| if *tile == Tile::Robot { Some(j) } else { None })?;
                Some((j, i))
            }).unwrap();

        Warehouse {
            map: DynMatrix::new(map.into_iter().rev().flatten().collect(), rows, cols),
            robot: Vec2::new([robot.0, robot.1])
        }
    }

    fn move_robot(&mut self, dir: Cardinal) {
        // Get all crates along line from robot till air or wall
        // Move robot and all crates along that line if air, don't move at all if wall
        let mut end = self.robot;
        let mut wall = false;
        while let Some(new) = dir.try_move(end) {
            end = new;
            match self.map[(*new.y(), *new.x())] {
                Tile::Wall => {
                    wall = true;
                    break;
                }
                Tile::Box => (),
                Tile::Empty => break,
                Tile::Robot => (),
            }
        }

        if !wall {
            let unit = dir.unit();
            let mut cur = self.robot;

            let mut prev_tile = Tile::Robot;
            while cur != end {
                let next = cur.add_signed(unit);
                let next_tile = self.map[(*next.y(), *next.x())];
                self.map[(*next.y(), *next.x())] = prev_tile;
                prev_tile = next_tile;

                cur = next;
            }

            self.map[(*self.robot.y(), *self.robot.x())] = Tile::Empty;
            self.robot = self.robot.add_signed(unit);
        }
    }

    fn iter_boxes(&self) -> Vec<Vec2<usize>> {
        let mut out = Vec::new();
        for y in 0..self.map.rows() {
            for x in 0..self.map.cols() {
                if self.map[(y, x)] == Tile::Box {
                    out.push(Vec2::new([x, y]));
                }
            }
        }
        out
    }

    #[allow(unused)]
    fn print(&self) {
        for y in (0..self.map.cols()).rev() {
            for x in 0..self.map.rows() {
                print!("{}", self.map[(y, x)])
            }
            println!()
        }
    }
}

fn parser<'a>() -> Parser!['a, (Warehouse, Vec<Cardinal>)] {
    use chumsky::prelude::*;

    let instrs = one_of("^v<>")
        .map(|c: char| match c {
            '^' => Cardinal::Up,
            'v' => Cardinal::Down,
            '<' => Cardinal::Left,
            '>' => Cardinal::Right,
            _ => unreachable!(),
        })
        .repeated()
        .at_least(1)
        .collect::<Vec<_>>()
        .separated_by(just('\n'))
        .allow_trailing()
        .collect::<Vec<_>>()
        .map(|v| v.into_iter().flatten().collect::<Vec<_>>());

    one_of("#.O@")
        .map(|c: char| match c {
            '#' => Tile::Wall,
            '.' => Tile::Empty,
            'O' => Tile::Box,
            '@' => Tile::Robot,
            _ => unreachable!(),
        })
        .repeated()
        .at_least(1)
        .collect::<Vec<_>>()
        .separated_by(just('\n'))
        .collect::<Vec<_>>()
        .map(Warehouse::new)
        .then_ignore(just("\n\n"))
        .then(instrs)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (mut wh, instrs) = parser().parse(input).unwrap();
    for instr in instrs {
        wh.move_robot(instr);
    }

    let rows = wh.map.rows();
    Some(wh.iter_boxes()
        .into_iter()
        // Convert from internal bottom-left 0 to top-left 0
        .map(|pos| 100 * (rows - pos.y() - 1) + pos.x())
        .sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
