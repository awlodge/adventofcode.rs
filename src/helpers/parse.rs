use std::{fmt::Debug, str::FromStr};

pub fn parse_lines<T>(input: &str) -> impl Iterator<Item = T>
where
    T: FromStr,
    T::Err: Debug,
{
    input.split('\n').enumerate().map(|(idx, x)| {
        let y = x.trim();
        match y.parse::<T>() {
            Ok(r) => r,
            Err(e) => panic!("Error parsing line {idx} '{y}': {e:?}"),
        }
    })
}
