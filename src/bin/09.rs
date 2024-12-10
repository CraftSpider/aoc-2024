use std::iter;
use std::iter::from_fn;
use std::ops::Range;

advent_of_code::solution!(9);

struct Disk {
    data: Vec<Option<u64>>,
    bytes: usize,
    files: u64,
}

impl Disk {
    fn from_input(input: &str) -> Disk {
        let (data, bytes, files, _) =
            input
                .trim()
                .chars()
                .enumerate()
                .fold((Vec::new(), 0, 0, true), |mut acc, (idx, c)| {
                    let num = (c as u8 - b'0') as usize;
                    if acc.3 {
                        acc.0.extend(iter::repeat_n(Some(idx as u64 / 2), num));
                        acc.1 += num;
                        acc.2 += 1;
                    } else {
                        acc.0.extend(iter::repeat_n(None, num))
                    }
                    (acc.0, acc.1, acc.2, !acc.3)
                });
        Disk { data, bytes, files }
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

    fn iter_empty_range(&self) -> impl Iterator<Item = Range<usize>> + '_ {
        let mut iter = self.data.iter();
        let mut idx = 0;
        from_fn(move || {
            while let Some(_) = iter.next()? {
                idx += 1;
            }
            let start = idx;
            idx += 1;

            loop {
                match iter.next() {
                    Some(None) => idx += 1,
                    Some(Some(_)) => {
                        idx += 1;
                        break;
                    }
                    None => {
                        idx += 1;
                        break;
                    }
                }
            }
            Some(Range {
                start,
                end: idx - 1,
            })
        })
    }

    fn file(&self, id: u64) -> Range<usize> {
        let start = self.data.iter().position(|&val| val == Some(id)).unwrap();
        let end = self.data.iter().rposition(|&val| val == Some(id)).unwrap();
        start..end + 1
    }

    fn try_compact_file(&mut self, idx: u64) {
        let file = self.file(idx);
        let Some(slot) = self
            .iter_empty_range()
            .find(|r| file.len() <= r.len() && file.start > r.start)
        else {
            return;
        };
        self.data[file.clone()].fill(None);
        self.data[slot.start..slot.start + file.len()].fill(Some(idx));
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
    for idx in (0..disk.files).rev() {
        disk.try_compact_file(idx);
    }
    // disk.print();
    Some(disk.checksum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter_empty() {
        let disk = Disk::from_input("1234");
        assert_eq!(
            disk.iter_empty_range().collect::<Vec<_>>(),
            vec![1..3, 6..10]
        );

        let disk = Disk::from_input("0120");
        assert_eq!(disk.iter_empty_range().collect::<Vec<_>>(), vec![0..1]);

        let disk = Disk::from_input("2333133121414131402");
        assert_eq!(
            disk.iter_empty_range().collect::<Vec<_>>(),
            vec![2..5, 8..11, 12..15, 18..19, 21..22, 26..27, 31..32, 35..36]
        )
    }

    #[test]
    fn test_file_range() {
        let disk = Disk::from_input("1234");
        assert_eq!(disk.file(1), 3..6)
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
