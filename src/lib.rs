use chumsky::{text, Parser};
use numeric::compound::vector::Vec2;
use numeric::traits::class::Integral;
use numeric::traits::ops::checked::{CheckedAdd, CheckedSub};
use std::str::FromStr;

pub mod fast_cartesian;
mod size_hint;
pub mod template;

// Use this file to add helper functions and additional modules.

#[macro_export]
macro_rules! Parser {
    ($lt:lifetime, $ty:ty) => {
        impl chumsky::Parser<$lt, &$lt str, $ty, chumsky::extra::Err<chumsky::error::Rich<$lt, char>>>
    };
}

pub fn int_u32<'a>() -> Parser!['a, u32] {
    text::int(10).map(u32::from_str).unwrapped()
}

pub fn int_u64<'a>() -> Parser!['a, u64] {
    text::int(10).map(u64::from_str).unwrapped()
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    pub fn all() -> [Direction; 4] {
        [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ]
    }

    pub fn rotate_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    pub fn try_move<I: Integral + CheckedAdd<Output = I> + CheckedSub<Output = I>>(
        self,
        pos: Vec2<I>,
    ) -> Option<Vec2<I>> {
        match self {
            Direction::Up => pos.checked_add(Vec2::new([I::zero(), I::one()])),
            Direction::Down => pos.checked_sub(Vec2::new([I::zero(), I::one()])),
            Direction::Right => pos.checked_add(Vec2::new([I::one(), I::zero()])),
            Direction::Left => pos.checked_sub(Vec2::new([I::one(), I::zero()])),
        }
    }
}
