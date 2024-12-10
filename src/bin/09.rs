use std::iter;
use std::iter::from_fn;
use std::ops::Range;

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

    fn compress_bytes(&mut self) {
        let (left, right) = self.data.split_at_mut(self.bytes);
        let slots = left.iter_mut().filter(|v| v.is_none());
        let bytes = right.iter_mut().rev().filter(|v| v.is_some());
        slots.zip(bytes).for_each(|(slot, byte)| {
            *slot = *byte;
            *byte = None;
        });
    }

    fn empty_ranges(&self) -> impl Iterator<Item = Range<usize>> + '_ {
        let mut iter = self.data.iter();
        let mut idx = 0;
        from_fn(move || {
            while let Some(_) = iter.next()? {
                idx += 1;
            }
            let start = idx;
            idx += 1;
            while let Some(None) = iter.next() {
                idx += 1;
            }
            let end = idx;
            idx += 1;
            Some(Range { start, end })
        })
    }

    fn file_ranges(&self) -> impl Iterator<Item = (u64, Range<usize>)> + '_ {
        let mut file_idx = 0;
        let mut idx = 0;
        from_fn(move || {
            file_idx += 1;

            while let Some(None) = self.data.get(idx) {
                idx += 1;
            }
            let fidx = self.data.get(idx)?.unwrap();
            let start = idx;
            loop {
                if self.data.get(idx).copied() == Some(Some(fidx)) {
                    idx += 1;
                } else {
                    break;
                }
            }
            let end = idx;
            if let Some(None) | None = self.data.get(idx) {
                idx += 1;
            }
            Some((file_idx - 1, Range { start, end }))
        })
    }

    fn compress_files(&mut self) {
        let files = self.file_ranges().collect::<Vec<_>>();
        let mut empties = self.empty_ranges().collect::<Vec<_>>();
        for file in files.into_iter().rev() {
            self.try_compact_file(file.0, file.1, &mut empties);
        }
    }

    fn try_compact_file(&mut self, idx: u64, file: Range<usize>, ranges: &mut [Range<usize>]) {
        let Some(slot) = ranges
            .iter_mut()
            .find(|r| file.len() <= r.len() && file.start > r.start)
        else {
            return;
        };
        self.data[file.clone()].fill(None);
        self.data[slot.start..slot.start + file.len()].fill(Some(idx));
        slot.start = slot.start + file.len();
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
    disk.compress_bytes();
    // disk.print();
    Some(disk.checksum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut disk = Disk::from_input(input);
    // disk.print();
    disk.compress_files();
    // disk.print();
    Some(disk.checksum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter_empty() {
        let disk = Disk::from_input("1234");
        assert_eq!(disk.empty_ranges().collect::<Vec<_>>(), vec![1..3, 6..10]);

        let disk = Disk::from_input("0120");
        assert_eq!(disk.empty_ranges().collect::<Vec<_>>(), vec![0..1]);

        let disk = Disk::from_input("2333133121414131402");
        assert_eq!(
            disk.empty_ranges().collect::<Vec<_>>(),
            vec![2..5, 8..11, 12..15, 18..19, 21..22, 26..27, 31..32, 35..36]
        )
    }

    #[test]
    fn test_iter_files() {
        let disk = Disk::from_input("1234");
        assert_eq!(
            disk.file_ranges().collect::<Vec<_>>(),
            vec![(0, 0..1), (1, 3..6)]
        );

        let disk = Disk::from_input("0120");
        assert_eq!(disk.file_ranges().collect::<Vec<_>>(), vec![(0, 1..3)]);

        let disk = Disk::from_input("2333133121414131402");
        assert_eq!(
            disk.file_ranges().collect::<Vec<_>>(),
            vec![
                (0, 0..2),
                (1, 5..8),
                (2, 11..12),
                (3, 15..18),
                (4, 19..21),
                (5, 22..26),
                (6, 27..31),
                (7, 32..35),
                (8, 36..40),
                (9, 40..42),
            ]
        )
    }

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
