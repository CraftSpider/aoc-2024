use advent_of_code::Parser;
use chumsky::prelude::*;
use std::str::FromStr;

advent_of_code::solution!(3);

#[derive(Copy, Clone)]
enum Instr {
    Mul(u32, u32),
    Do,
    Dont,
}

fn parser<'a>() -> Parser!['a, Vec<Instr>] {
    let int = text::digits(10)
        .at_least(1)
        .at_most(3)
        .to_slice()
        .map(u32::from_str)
        .unwrapped();
    let mul_instr = just("mul(")
        .ignore_then(int)
        .then_ignore(just(','))
        .then(int)
        .then_ignore(just(')'))
        .map(|(a, b)| Instr::Mul(a, b));
    let do_instr = just("do()").to(Instr::Do);
    let dont_instr = just("don't()").to(Instr::Dont);

    let valid_instr = choice((mul_instr, do_instr, dont_instr)).map(Some);

    valid_instr
        .or(any().to(None))
        .repeated()
        .collect::<Vec<_>>()
        .map(|v| v.into_iter().filter_map(|a| a).collect::<Vec<_>>())
}

pub fn part_one(input: &str) -> Option<u32> {
    let vals = parser().parse(input).unwrap();
    Some(
        vals.into_iter()
            .filter_map(|instr| {
                if let Instr::Mul(a, b) = instr {
                    Some(a * b)
                } else {
                    None
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let vals = parser().parse(input).unwrap();
    let out = vals
        .into_iter()
        .fold((0, true), |(acc, do_mul), instr| match instr {
            Instr::Mul(a, b) => (acc + (a * b * do_mul as u32), do_mul),
            Instr::Do => (acc, true),
            Instr::Dont => (acc, false),
        })
        .0;
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
