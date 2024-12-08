use chumsky::text::Char;
use itertools::Itertools;
use numeric::compound::vector::Vec2;
use std::collections::{BTreeMap, HashSet};

advent_of_code::solution!(8);

struct Map {
    // Map of frequencies to sets of antennas
    antennas: BTreeMap<u8, Vec<Vec2<i32>>>,
    size: Vec2<i32>,
}

impl Map {
    fn from_input(input: &str) -> Map {
        let mut antennas = BTreeMap::<_, Vec<_>>::new();
        let mut size = Vec2::default();
        for (y, line) in input.trim().rsplit('\n').enumerate() {
            let y = y as i32;

            if y > *size.y() {
                size.set_y(y);
            }
            for (x, char) in line.chars().enumerate() {
                let x = x as i32;

                if x > *size.x() {
                    size.set_x(x);
                }
                if char != '.' {
                    antennas
                        .entry(char as u8)
                        .or_default()
                        .push(Vec2::new([x, y]))
                }
            }
        }
        Map { antennas, size }
    }

    #[allow(unused)]
    fn print(&self, nodes: &HashSet<Vec2<i32>>) {
        for y in (0..=*self.size.y()).rev() {
            for x in 0..=*self.size.x() {
                let loc = Vec2::new([x, y]);
                if let Some((&freq, _)) = self
                    .antennas
                    .iter()
                    .find(|(freq, locs)| locs.contains(&loc))
                {
                    print!("{}", char::from_ascii(freq))
                } else if nodes.contains(&loc) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!()
        }
    }
}

fn validate_node(node: Vec2<i32>, size: Vec2<i32>) -> Option<Vec2<i32>> {
    if (0..=*size.x()).contains(node.x()) && (0..=*size.y()).contains(node.y()) {
        Some(node)
    } else {
        None
    }
}

fn find_antinodes(map: &Map) -> HashSet<Vec2<i32>> {
    let mut positions = HashSet::new();
    for (frequency, locs) in &map.antennas {
        let _freq = char::from_ascii(*frequency);
        let new_pos = locs
            .iter()
            .cartesian_product(locs.iter())
            .map(|(&a, &b)| {
                if a == b {
                    return None;
                };
                // Given two antennas, antinodes will be along the line by doubling the difference
                // between the two.
                // Get vector pointing from b to a: (b - a)
                // Double it, re-add it to a
                let node = a + (b - a) * 2;
                validate_node(node, map.size)
            })
            .filter_map(|v| v);
        positions.extend(new_pos);
    }
    positions
}

fn find_more_antinodes(map: &Map) -> HashSet<Vec2<i32>> {
    let mut positions = HashSet::new();
    for (frequency, locs) in &map.antennas {
        let _freq = char::from_ascii(*frequency);
        let new_pos = locs
            .iter()
            .cartesian_product(locs.iter())
            .map(|(&a, &b)| {
                if a == b {
                    return Vec::new();
                };
                // Given two antennas, antinodes will be along the line by doubling the difference
                // between the two.
                // Get vector pointing from b to a: (b - a)
                // Double it, re-add it to a
                let diff = b - a;
                let mut out = Vec::new();
                let mut factor = 1;
                while let Some(node) = validate_node(a + diff * factor, map.size) {
                    factor += 1;
                    out.push(node);
                }
                out
            })
            .flatten();
        positions.extend(new_pos);
    }
    positions
}

// not 258
pub fn part_one(input: &str) -> Option<u32> {
    let map = Map::from_input(input);
    let out = find_antinodes(&map);
    Some(out.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = Map::from_input(input);
    let out = find_more_antinodes(&map);
    Some(out.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let map = Map::from_input(
            "\
..........
..........
..........
....a.....
........a.
.....a....
..........
..........
..........
..........",
        );
        let antinodes = find_antinodes(&map);
        assert_eq!(
            antinodes,
            HashSet::from([
                Vec2::new([6, 2]),
                Vec2::new([2, 3]),
                Vec2::new([0, 7]),
                Vec2::new([3, 8])
            ])
        )
    }

    #[test]
    fn test_example_2() {
        let map = Map::from_input(
            "\
..........
..........
..........
....a.....
........a.
.....a....
..........
......A...
..........
..........",
        );
        let antinodes = find_antinodes(&map);
        assert_eq!(
            antinodes,
            HashSet::from([
                Vec2::new([6, 2]),
                Vec2::new([2, 3]),
                Vec2::new([0, 7]),
                Vec2::new([3, 8])
            ])
        )
    }

    #[test]
    fn test_example_3() {
        let map = Map::from_input(
            "\
..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
..........",
        );
        let antinodes = find_antinodes(&map);
        assert_eq!(
            antinodes,
            HashSet::from([Vec2::new([6, 2]), Vec2::new([3, 8])])
        )
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
