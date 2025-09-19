use std::cmp::{min};
use numeric::compound::vector::Vec2;
use advent_of_code::{int_u64, Parser};
use chumsky::Parser;
use numeric::traits::identity::Zero;

advent_of_code::solution!(13);

fn parser<'a>() -> Parser!['a, Vec<(Vec2<u64>, Vec2<u64>, Vec2<u64>)>] {
    use chumsky::prelude::*;

    let pref_int = |p: &'static str| just(p).ignore_then(int_u64());

    let button = pref_int("X+").then_ignore(just(", ")).then(pref_int("Y+"))
        .map(|(x, y)| Vec2::new([x, y]));
    let out = pref_int("X=").then_ignore(just(", ")).then(pref_int("Y="))
        .map(|(x, y)| Vec2::new([x, y]));

    let a = just("Button A: ")
        .ignore_then(button.clone())
        .then_ignore(just('\n'));

    let b = just("Button B: ")
        .ignore_then(button)
        .then_ignore(just('\n'));

    let prize = just("Prize: ")
        .ignore_then(out)
        .then_ignore(just('\n'));

    group((a, b, prize))
        .separated_by(just('\n'))
        .collect::<Vec<_>>()

}

fn solve(goal: Vec2<u64>, a: Vec2<u64>, b: Vec2<u64>) -> Vec<Vec2<u64>> {
    let mut solutions = Vec::new();

    for i in 0..100 {
        for j in 0..100 {
            if a*i + b*j == goal {
                solutions.push(Vec2::new([i, j]))
            }
        }
    }

    solutions

    // X1, X2, Y1, Y2, XF, YF
    // XF = A*X1 + B*X2
    // YF = A*Y1 + B*Y2
    // A <= 100, B <= 100
    // We are given _F, _1, _2

    // let end = min(goal.x() / a.x(), goal.y() / a.y());
    //
    // for i in 0..end {
    //     let new_goal = goal - a*i;
    //     if new_goal % b != Vec2::zero() {
    //         continue;
    //     }
    //     let solution = new_goal / b;
    //     if solution.x() == solution.y() {
    //         solutions.push(Vec2::new([i, *solution.x()]));
    //     }
    // }
    // solutions
}

pub fn part_one(input: &str) -> Option<u64> {
    let machines = parser().parse(input)
        .unwrap();

    let mut out = 0;
    for (a, b, goal) in machines {
        let solutions = solve(goal, a, b);
        out += solutions.iter()
            .map(|presses| presses.x() * 3 + presses.y())
            .min()
            .unwrap_or(0);
    }
    Some(out)
}

pub fn part_two(input: &str) -> Option<u64> {
    // let machines = parser().parse(input)
    //     .unwrap();
    //
    // let mut out = 0;
    // for (a, b, mut goal) in machines {
    //     goal += 10000000000000;
    //     let solutions = solve(goal, a, b);
    //     out += solutions.iter()
    //         .map(|presses| presses.x() * 3 + presses.y())
    //         .min()
    //         .unwrap_or(0);
    // }
    // Some(out)
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
