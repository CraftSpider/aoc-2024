use chumsky::{text, Parser};
use std::str::FromStr;

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
