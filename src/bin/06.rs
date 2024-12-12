use advent_of_code::Cardinal;
use numeric::compound::vector::Vec2;
use std::collections::HashSet;

advent_of_code::solution!(6);

#[derive(Default)]
struct Map {
    blocks: HashSet<Vec2<u32>>,
    size: Vec2<u32>,
    guard: Vec2<u32>,
}

impl Map {
    fn start_state(&self) -> GuardState {
        GuardState {
            pos: self.guard,
            direction: Cardinal::Up,
            extra_block: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct GuardState {
    pos: Vec2<u32>,
    direction: Cardinal,
    extra_block: Option<Vec2<u32>>,
}

impl GuardState {
    fn rotate_right(&mut self) {
        self.direction = self.direction.rotate_right()
    }
}

enum MoveFail {
    OffMap,
    Blocked,
}

impl Map {
    fn from_input(input: &str) -> Map {
        let mut map = Map::default();

        let mut size = Vec2::default();
        for (y, line) in input.lines().rev().enumerate() {
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
        map.size = size + 1;

        map
    }

    #[allow(unused)]
    fn print(&self) {
        for y in (0..=*self.size.y()).rev() {
            for x in 0..=*self.size.x() {
                if self.guard == Vec2::new([x, y]) {
                    print!("^");
                } else if self.blocks.contains(&(Vec2::new([x, y]))) {
                    print!("#")
                } else {
                    print!(".")
                }
            }
            println!();
        }
    }

    #[allow(unused)]
    fn print_with_paths(&self, block: Vec2<u32>, locs: &HashSet<(Vec2<u32>, Cardinal)>) {
        for y in (0..=*self.size.y()).rev() {
            for x in 0..=*self.size.x() {
                if Vec2::new([x, y]) == block {
                    print!("O");
                    continue;
                }

                let up = locs.contains(&(Vec2::new([x, y]), Cardinal::Up));
                let down = locs.contains(&(Vec2::new([x, y]), Cardinal::Down));
                let left = locs.contains(&(Vec2::new([x, y]), Cardinal::Left));
                let right = locs.contains(&(Vec2::new([x, y]), Cardinal::Right));
                match (up, down, left, right) {
                    (true, true, true, true) => print!("┼"),
                    (true, true, true, false) => print!("├"),
                    (true, true, false, true) => print!("┥"),
                    (true, true, false, false) => print!("│"),
                    (true, false, true, true) => print!("┬"),
                    (true, false, true, false) => print!("└"),
                    (true, false, false, true) => print!("┌"),
                    (true, false, false, false) => print!("│"),
                    (false, true, true, true) => print!("┴"),
                    (false, true, true, false) => print!("┘"),
                    (false, true, false, true) => print!("┐"),
                    (false, true, false, false) => print!("│"),
                    (false, false, true, true) => print!("─"),
                    (false, false, true, false) => print!("─"),
                    (false, false, false, true) => print!("─"),
                    (false, false, false, false) => {
                        if self.blocks.contains(&(Vec2::new([x, y]))) {
                            print!("#")
                        } else {
                            print!(".")
                        }
                    }
                }
            }
            println!()
        }
        println!();
    }

    fn try_move(&self, state: &GuardState) -> Result<Vec2<u32>, MoveFail> {
        let next = state
            .direction
            .try_move(state.pos)
            .ok_or(MoveFail::OffMap)?;
        if next.x() >= self.size.x() || next.y() >= self.size.y() {
            Err(MoveFail::OffMap)
        } else if self.blocks.contains(&next)
            || state.extra_block.is_some_and(|block| next == block)
        {
            Err(MoveFail::Blocked)
        } else {
            Ok(next)
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = Map::from_input(input);

    let mut visited = HashSet::new();
    let mut guard = map.start_state();
    loop {
        visited.insert(guard.pos);
        match map.try_move(&guard) {
            Ok(next) => guard.pos = next,
            Err(MoveFail::Blocked) => guard.rotate_right(),
            Err(MoveFail::OffMap) => break,
        }
    }
    Some(visited.len() as u32)
}

fn check_loop(map: &Map, mut guard: GuardState) -> bool {
    let mut seen_locs = HashSet::new();
    guard.rotate_right();
    loop {
        // Can't shortcut by turn count - some loops may be more complex than that
        if seen_locs.contains(&guard) {
            // map.print_with_paths(_block, &seen_locs);
            return true;
        }
        seen_locs.insert(guard.clone());

        match map.try_move(&guard) {
            Ok(next) => guard.pos = next,
            Err(MoveFail::Blocked) => guard.rotate_right(),
            Err(MoveFail::OffMap) => break false,
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = Map::from_input(input);

    let mut seen_locs = HashSet::new();
    let mut guard = map.start_state();
    let mut loops = 0;
    loop {
        seen_locs.insert(guard.pos);

        match map.try_move(&guard) {
            Ok(next) => {
                // Try inserting next as block
                if !seen_locs.contains(&next) {
                    guard.extra_block = Some(next);
                    if check_loop(&map, guard.clone()) {
                        loops += 1;
                    }
                    guard.extra_block = None;
                }
                guard.pos = next
            }
            Err(MoveFail::Blocked) => guard.rotate_right(),
            Err(MoveFail::OffMap) => break,
        }
    }
    Some(loops)
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
    fn test_weird_loops() {
        let map = Map::from_input(
            "\
..#.....#..
.#...^...#.
..#.....#..",
        );
        let mut guard = map.start_state();
        guard.extra_block = Some(Vec2::new([5, 2]));
        assert!(check_loop(&map, map.start_state()));

        let map = Map::from_input(
            "\
..#..
.#^#.
..#..",
        );
        let mut guard = map.start_state();
        guard.extra_block = Some(Vec2::new([2, 2]));
        assert!(check_loop(&map, map.start_state()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
