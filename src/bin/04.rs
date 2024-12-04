use itertools::Itertools;

advent_of_code::solution!(4);

fn make_array(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .split("\n")
        .map(|s| s.chars().collect())
        .collect()
}

fn check_line(
    matrix: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    x_add: isize,
    y_add: isize,
) -> Option<()> {
    const LINE: [char; 3] = ['M', 'A', 'S'];
    for i in 1..4 {
        let x = x.checked_add_signed(x_add * i)?;
        let y = y.checked_add_signed(y_add * i)?;
        if *matrix.get(x)?.get(y)? != LINE[(i - 1) as usize] {
            return None;
        }
    }
    Some(())
}

fn check_xmas(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> u32 {
    if matrix[x][y] != 'X' {
        return 0;
    }
    let mut out = 0;
    for x_diff in -1..=1 {
        for y_diff in -1..=1 {
            if x_diff == 0 && y_diff == 0 {
                continue;
            }
            if check_line(matrix, x, y, x_diff, y_diff).is_some() {
                out += 1;
            }
        }
    }
    out
}

/* Possible Forms:

M.M
.A.
S.S

M.S
.A.
M.S

S.M
.A.
S.M

S.S
.A.
M.M
*/

fn check_x_mas(window: &((char, char, char), (char, char, char), (char, char, char))) -> bool {
    if window.1 .1 != 'A' {
        return false;
    }
    let tl = window.0 .0;
    let tr = window.0 .2;
    let bl = window.2 .0;
    let br = window.2 .2;
    // Any adjacent pair == M while the other pair == S
    // Takes advantage of corners never being in a pair, and that each corner will
    // always be either M or S.
    if (tl == tr && bl == br) || (tl == bl && tr == br) {
        if (tl == 'M' && br == 'S') || (tl == 'S' && br == 'M') {
            return true;
        }
    }
    false
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = make_array(input);
    let mut count = 0;
    for x in 0..input.len() {
        for y in 0..input[0].len() {
            count += check_xmas(&input, x, y);
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = make_array(input);
    let count = input
        .iter()
        .tuple_windows()
        .enumerate()
        .map(|(idx, (a, b, c))| {
            let a = a.iter().copied().tuple_windows::<(_, _, _)>();
            let b = b.iter().copied().tuple_windows::<(_, _, _)>();
            let c = c.iter().copied().tuple_windows::<(_, _, _)>();

            itertools::multizip((a, b, c))
                .enumerate()
                .filter(|(idx2, val)| check_x_mas(val))
                .count()
        })
        .sum::<usize>();
    Some(count as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
