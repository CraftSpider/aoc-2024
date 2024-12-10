use advent_of_code::Direction;
use numeric::compound::vector::Vec2;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(10);

struct Map {
    topography: Vec<Vec<u8>>,
    size: Vec2<usize>,
}

impl Map {
    fn from_input(input: &str) -> Map {
        let topography = input
            .lines()
            .rev()
            .map(|l| l.chars().map(|c| c as u8 - b'0').collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let y = topography.len();
        let x = topography.iter().map(|l| l.len()).max().unwrap();
        Map {
            topography,
            size: Vec2::new([x, y]),
        }
    }

    fn iter_peaks(&self) -> impl Iterator<Item = Vec2<usize>> + '_ {
        self.topography.iter().enumerate().flat_map(|(y, line)| {
            line.iter().enumerate().filter_map(move |(x, &height)| {
                if height == 9 {
                    Some(Vec2::new([x, y]))
                } else {
                    None
                }
            })
        })
    }

    fn height(&self, pos: Vec2<usize>) -> u8 {
        self.topography[*pos.y()][*pos.x()]
    }

    fn try_move(&self, dir: Direction, pos: Vec2<usize>) -> Option<Vec2<usize>> {
        let next = dir.try_move(pos)?;
        if next.x() < self.size.x() && next.y() < self.size.y() {
            Some(next)
        } else {
            None
        }
    }
}

fn find_ends(map: &Map, start: Vec2<usize>) -> Vec<Vec2<usize>> {
    fn search(map: &Map, pos: Vec2<usize>, height: u8, out: &mut Vec<Vec2<usize>>) {
        if height == 0 {
            if map.height(pos) == 0 {
                out.push(pos);
            }
            return;
        }

        for dir in Direction::all() {
            let Some(pos) = map.try_move(dir, pos) else {
                continue;
            };
            if map.height(pos) == height - 1 {
                search(map, pos, height - 1, out)
            }
        }
    }

    let mut out = Vec::new();
    search(map, start, map.height(start), &mut out);
    out
}

pub fn part_one(input: &str) -> Option<u32> {
    // For each endpoint, we track backwards to all possible start points
    let map = Map::from_input(input);

    let mut trailheads = HashMap::<_, HashSet<_>>::new();
    for peak in map.iter_peaks() {
        // eprintln!("Peak: {peak:?}");
        for end in find_ends(&map, peak) {
            // eprintln!("  End: {end:?}");
            let ends = trailheads.entry(end).or_default();
            ends.insert(peak);
        }
    }
    // dbg!(trailheads.get(&Vec2::new([1, 0])).unwrap());
    let out = trailheads
        .iter()
        .map(|(_, peaks)| peaks.len())
        .sum::<usize>();
    Some(out as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    // For each endpoint, we track backwards to all possible start points
    let map = Map::from_input(input);

    let mut trailheads = HashMap::<_, u32>::new();
    for peak in map.iter_peaks() {
        // eprintln!("Peak: {peak:?}");
        for end in find_ends(&map, peak) {
            // eprintln!("  End: {end:?}");
            let ends = trailheads.entry(end).or_default();
            *ends += 1;
        }
    }
    // dbg!(trailheads.get(&Vec2::new([1, 0])).unwrap());
    let out = trailheads.iter().map(|(_, &peaks)| peaks).sum();
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
