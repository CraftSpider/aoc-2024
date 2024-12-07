use advent_of_code::{int_u64, Parser};
use chumsky::prelude::*;
use itertools::Itertools;

advent_of_code::solution!(7);

#[derive(Copy, Clone)]
enum Op {
    Add,
    Mul,
    Concat,
}

struct Equation {
    answer: u64,
    inputs: Vec<u64>,
}

impl Equation {
    fn try_add_mul(&self) -> bool {
        self.inputs
            .iter()
            .map(|_| [Op::Add, Op::Mul])
            .multi_cartesian_product()
            .any(|ops| {
                let val =
                    ops.iter()
                        .zip(&self.inputs[1..])
                        .fold(self.inputs[0], |acc, (&op, &val)| match op {
                            Op::Add => acc + val,
                            Op::Mul => acc * val,
                            _ => unreachable!(),
                        });
                val == self.answer
            })
    }

    fn try_any(&self) -> bool {
        self.inputs
            .iter()
            .map(|_| [Op::Add, Op::Mul, Op::Concat])
            .multi_cartesian_product()
            .any(|ops| {
                let val =
                    ops.iter()
                        .zip(&self.inputs[1..])
                        .fold(self.inputs[0], |acc, (&op, &val)| match op {
                            Op::Add => acc + val,
                            Op::Mul => acc * val,
                            Op::Concat => format!("{acc}{val}").parse().unwrap(),
                        });
                val == self.answer
            })
    }
}

fn parser<'a>() -> Parser!['a, Vec<Equation>] {
    int_u64()
        .then_ignore(just(": "))
        .then(int_u64().separated_by(just(" ")).collect())
        .map(|(answer, inputs)| Equation { answer, inputs })
        .separated_by(just("\n"))
        .allow_trailing()
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = parser().parse(input).unwrap();
    let out = input
        .into_iter()
        .map(|eq| if eq.try_add_mul() { eq.answer } else { 0 })
        .sum();
    Some(out)
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parser().parse(input).unwrap();
    let out = input
        .into_iter()
        .map(|eq| if eq.try_any() { eq.answer } else { 0 })
        .sum();
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
