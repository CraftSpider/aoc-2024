use advent_of_code::{Cardinal, Diagonal};
use numeric::compound::vector::Vec2;
use std::collections::HashSet;

advent_of_code::solution!(12);

struct Map {
    data: Vec<Vec<u8>>,
    size: Vec2<usize>,
}

impl Map {
    fn from_input(input: &str) -> Map {
        let data = input
            .lines()
            .rev()
            .map(|l| l.chars().map(|c| c as u8).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let y = data.len();
        let x = data.iter().map(|l| l.len()).max().unwrap();
        Map {
            data,
            size: Vec2::new([x, y]),
        }
    }

    fn plant(&self, pos: Vec2<usize>) -> u8 {
        self.data[*pos.y()][*pos.x()]
    }

    fn set_plant(&mut self, pos: Vec2<usize>, data: u8) {
        self.data[*pos.y()][*pos.x()] = data;
    }

    fn try_move(&self, pos: Vec2<usize>, dir: Cardinal) -> Option<Vec2<usize>> {
        let pos = dir.try_move(pos)?;
        if pos.y() < self.size.y() && pos.x() < self.size.x() {
            Some(pos)
        } else {
            None
        }
    }

    fn try_move_diag(&self, pos: Vec2<usize>, diag: Diagonal) -> Option<Vec2<usize>> {
        let pos = diag.try_move(pos)?;
        if pos.y() < self.size.y() && pos.x() < self.size.x() {
            Some(pos)
        } else {
            None
        }
    }

    fn eat_region(&mut self, pos: Vec2<usize>) -> Option<Region> {
        // Recursive region eating.
        // Try moving in every direction we haven't yet seen
        // If you succeed in moving, add 1 to area
        // If you don't, add 1 to perimeter
        // Set all eaten areas to 0
        fn eat_recursive(
            map: &mut Map,
            plant: u8,
            pos: Vec2<usize>,
            seen: &mut HashSet<Vec2<usize>>,
            region: &mut Region,
        ) {
            seen.insert(pos);
            for dir in Cardinal::all() {
                match map.try_move(pos, dir) {
                    Some(pos) => {
                        if seen.contains(&pos) {
                            continue;
                        } else if map.plant(pos) != plant {
                            region.perimeter += 1;
                        } else {
                            region.area += 1;
                            eat_recursive(map, plant, pos, seen, region);
                        }
                    }
                    None => {
                        region.perimeter += 1;
                    }
                }
            }
            map.set_plant(pos, 0);
        }

        let plant = self.plant(pos);
        if plant == 0 {
            return None;
        }

        let mut region = Region {
            area: 1,
            perimeter: 0,
        };
        let mut seen = HashSet::new();
        eat_recursive(self, plant, pos, &mut seen, &mut region);
        Some(region)
    }

    fn eat_side_region(
        &self,
        pos: Vec2<usize>,
        global_seen: &mut HashSet<Vec2<usize>>,
    ) -> Option<SideRegion> {
        // Recursive region eating.
        // Try moving in every direction we haven't yet seen
        // If you succeed in moving, add 1 to area
        // If you don't, add 1 to perimeter
        // Set all eaten areas to 0
        fn eat_recursive(
            map: &Map,
            plant: u8,
            pos: Vec2<usize>,
            global_seen: &mut HashSet<Vec2<usize>>,
            seen: &mut HashSet<Vec2<usize>>,
            region: &mut SideRegion,
        ) {
            global_seen.insert(pos);
            seen.insert(pos);
            // For each direction, try to add area
            // For each pair of adjacent directions, try to add a corner (sides == corners)
            for dir in Cardinal::all() {
                match map.try_move(pos, dir) {
                    Some(pos2) => {
                        if seen.contains(&pos2) || map.plant(pos2) != plant {
                            continue;
                        } else {
                            region.area += 1;
                            eat_recursive(map, plant, pos2, seen, global_seen, region);
                        }
                    }
                    None => (),
                }
            }

            for diag in Diagonal::all() {
                // A corner is either:
                // Both cardinals aren't the same plant as us
                // *OR*
                // Both cardinals are the same plant, and the diagonal isn't
                let cardinal_plants = diag.cardinals().map(|c| match map.try_move(pos, c) {
                    Some(pos) => map.plant(pos) == plant,
                    None => false,
                });

                let both_cardinals = cardinal_plants.into_iter().all(|b| b);
                let neither_cardinals = cardinal_plants.into_iter().all(|b| !b);

                let diag_plant = match map.try_move_diag(pos, diag) {
                    Some(pos) => map.plant(pos) == plant,
                    None => false,
                };

                if neither_cardinals || (both_cardinals && !diag_plant) {
                    region.sides += 1;
                }
            }
        }

        if global_seen.contains(&pos) {
            return None;
        }

        let plant = self.plant(pos);
        let mut region = SideRegion { area: 1, sides: 0 };
        let mut seen = HashSet::new();
        eat_recursive(self, plant, pos, global_seen, &mut seen, &mut region);
        Some(region)
    }
}

struct Region {
    perimeter: usize,
    area: usize,
}

#[derive(Debug)]
struct SideRegion {
    sides: usize,
    area: usize,
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut map = Map::from_input(input);
    let mut regions = Vec::new();
    for x in 0..*map.size.x() {
        for y in 0..*map.size.y() {
            if let Some(r) = map.eat_region(Vec2::new([x, y])) {
                regions.push(r);
            }
        }
    }
    let out = regions.iter().map(|r| r.area * r.perimeter).sum::<usize>();
    Some(out as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = Map::from_input(input);
    let mut regions = Vec::new();
    let mut global_seen = HashSet::new();
    for x in 0..*map.size.x() {
        for y in 0..*map.size.y() {
            let pos = Vec2::new([x, y]);
            if let Some(r) = map.eat_side_region(pos, &mut global_seen) {
                regions.push(r);
            }
        }
    }
    let out = regions.iter().map(|r| r.area * r.sides).sum::<usize>();
    Some(out as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }

    #[test]
    fn test_part_two_2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(80));
    }

    #[test]
    fn test_part_two_3() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(236));
    }

    #[test]
    fn test_part_two_4() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(368));
    }

    #[test]
    fn test_part_two_5() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(436));
    }
}
