use std::fmt::Debug;
use std::path::Path;
use std::str::FromStr;

pub fn read_input(day: u8) -> String {
    std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("inputs")
            .join(format!("day{day}.txt")),
    )
    .unwrap()
}

pub fn read_input_lines(day: u8, include_empty: bool) -> Vec<String> {
    read_input(day)
        .lines()
        .filter_map(|it| {
            if !include_empty && it.is_empty() {
                None
            } else {
                Some(it.to_string())
            }
        })
        .collect()
}

pub fn read_input_lines_as<T>(day: u8) -> Vec<T>
where
    T: FromStr,
    T::Err: Debug,
{
    read_input_lines(day, false)
        .into_iter()
        .map(|it| it.parse().unwrap())
        .collect()
}
