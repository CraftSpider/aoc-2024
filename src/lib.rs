use chumsky::{text, Parser};
use numeric::compound::vector::Vec2;
use numeric::traits::class::Integral;
use numeric::traits::ops::checked::{CheckedAdd, CheckedSub};
use std::str::FromStr;
use chumsky::prelude::just;

pub mod fast_cartesian;
mod size_hint;
pub mod template;

// Use this file to add helper functions and additional modules.

#[macro_export]
macro_rules! Parser {
    ($lt:lifetime, $ty:ty) => {
        impl chumsky::Parser<$lt, &$lt str, $ty, chumsky::extra::Err<chumsky::error::Rich<$lt, char>>> + Clone
    };
}

pub fn int_u32<'a>() -> Parser!['a, u32] {
    text::int(10).map(u32::from_str).unwrapped()
}

pub fn int_u64<'a>() -> Parser!['a, u64] {
    text::int(10).map(u64::from_str).unwrapped()
}

pub fn int_i64<'a>() -> Parser!['a, i64] {
    just('-').or_not().then(text::int(10).map(i64::from_str).unwrapped())
        .map(|(neg, val)| if neg.is_some() { -val } else { val })
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Diagonal {
    UpLeft,
    UpRight,
    DownRight,
    DownLeft,
}

impl Diagonal {
    pub fn all() -> [Diagonal; 4] {
        [
            Diagonal::UpLeft,
            Diagonal::UpRight,
            Diagonal::DownRight,
            Diagonal::DownLeft,
        ]
    }

    pub const fn rotate_right(self) -> Diagonal {
        match self {
            Diagonal::UpLeft => Diagonal::UpRight,
            Diagonal::UpRight => Diagonal::DownRight,
            Diagonal::DownRight => Diagonal::DownLeft,
            Diagonal::DownLeft => Diagonal::UpLeft,
        }
    }

    pub fn cardinals(self) -> [Cardinal; 2] {
        match self {
            Diagonal::UpLeft => [Cardinal::Up, Cardinal::Left],
            Diagonal::UpRight => [Cardinal::Up, Cardinal::Right],
            Diagonal::DownRight => [Cardinal::Down, Cardinal::Right],
            Diagonal::DownLeft => [Cardinal::Down, Cardinal::Left],
        }
    }

    pub fn try_move<I: Integral + CheckedAdd<Output = I> + CheckedSub<Output = I>>(
        self,
        pos: Vec2<I>,
    ) -> Option<Vec2<I>> {
        match self {
            Diagonal::UpLeft => pos
                .checked_add(Vec2::new([I::zero(), I::one()]))?
                .checked_sub(Vec2::new([I::one(), I::zero()])),
            Diagonal::UpRight => pos.checked_add(Vec2::new([I::one(), I::one()])),
            Diagonal::DownRight => pos
                .checked_add(Vec2::new([I::one(), I::zero()]))?
                .checked_sub(Vec2::new([I::zero(), I::one()])),
            Diagonal::DownLeft => pos.checked_sub(Vec2::new([I::one(), I::one()])),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Cardinal {
    Up,
    Down,
    Right,
    Left,
}

impl Cardinal {
    pub fn all() -> [Cardinal; 4] {
        [
            Cardinal::Up,
            Cardinal::Right,
            Cardinal::Down,
            Cardinal::Left,
        ]
    }

    pub const fn rotate_right(self) -> Cardinal {
        match self {
            Cardinal::Up => Cardinal::Right,
            Cardinal::Down => Cardinal::Left,
            Cardinal::Left => Cardinal::Up,
            Cardinal::Right => Cardinal::Down,
        }
    }

    pub fn try_move<I: Integral + CheckedAdd<Output = I> + CheckedSub<Output = I>>(
        self,
        pos: Vec2<I>,
    ) -> Option<Vec2<I>> {
        match self {
            Cardinal::Up => pos.checked_add(Vec2::new([I::zero(), I::one()])),
            Cardinal::Down => pos.checked_sub(Vec2::new([I::zero(), I::one()])),
            Cardinal::Right => pos.checked_add(Vec2::new([I::one(), I::zero()])),
            Cardinal::Left => pos.checked_sub(Vec2::new([I::one(), I::zero()])),
        }
    }
}
