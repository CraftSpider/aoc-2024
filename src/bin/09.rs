use std::iter;

advent_of_code::solution!(9);

struct Disk {
    data: Vec<Option<u64>>,
    bytes: usize,
}

impl Disk {
    fn from_input(input: &str) -> Disk {
        let (data, bytes, _) =
            input
                .trim()
                .chars()
                .enumerate()
                .fold((Vec::new(), 0, true), |mut acc, (idx, c)| {
                    let num = (c as u8 - b'0') as usize;
                    if acc.2 {
                        acc.0.extend(iter::repeat_n(Some(idx as u64 / 2), num));
                        acc.1 += num;
                    } else {
                        acc.0.extend(iter::repeat_n(None, num))
                    }
                    (acc.0, acc.1, !acc.2)
                });
        Disk { data, bytes }
    }

    fn rightmost_full(&self) -> usize {
        self.data.iter().rposition(|val| val.is_some()).unwrap()
    }

    fn leftmost_empty(&self) -> usize {
        self.data.iter().position(|val| val.is_none()).unwrap()
    }

    fn most_compact_byte(&self) -> bool {
        self.data[self.bytes..].iter().all(|&val| val.is_none())
    }

    fn most_compact_files(&self) -> bool {
        todo!()
    }

    fn move_rightmost_byte(&mut self) {
        let byte = self.rightmost_full();
        let empty = self.leftmost_empty();
        self.data.swap(byte, empty);
    }

    fn move_rightmost_file(&mut self) {
        todo!()
    }

    fn checksum(&self) -> u64 {
        self.data
            .iter()
            .enumerate()
            .fold(0, |acc, (idx, val)| match val {
                &Some(val) => acc + idx as u64 * val,
                None => acc,
            })
    }

    #[allow(unused)]
    fn print(&self) {
        fn encode_char(val: u64) -> char {
            const CHARS: [char; 62] = {
                let mut out = ['\0'; 10 + 26 + 26];
                let mut i = 0;
                while i < 10 {
                    out[i] = (b'0' + i as u8) as char;
                    i += 1;
                }
                let mut i = 0;
                while i < 26 {
                    out[i + 10] = (b'a' + i as u8) as char;
                    i += 1;
                }
                let mut i = 0;
                while i < 26 {
                    out[i + 36] = (b'A' + i as u8) as char;
                    i += 1;
                }
                out
            };

            CHARS
                .get(val as usize)
                .copied()
                .unwrap_or_else(|| (255 as char..).nth(val as usize).unwrap())
        }

        for &val in &self.data {
            match val {
                None => print!("."),
                Some(val) => print!("{}", encode_char(val)),
            }
        }
        println!()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut disk = Disk::from_input(input);
    // disk.print();
    while !disk.most_compact_byte() {
        disk.move_rightmost_byte();
    }
    // disk.print();
    Some(disk.checksum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut disk = Disk::from_input(input);
    // disk.print();
    while !disk.most_compact_files() {
        disk.move_rightmost_file();
    }
    // disk.print();
    Some(disk.checksum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
