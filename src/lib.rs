#![warn(missing_docs)]
// Copyright 2018 by Aldrin J D'Souza.
// Licensed under the MIT License <https://opensource.org/licenses/MIT>
//! # Advent of Code
//!
//! `advent` is a collection of my solutions to the [Advent of Code] puzzles posted in December each
//! year. The solutions are implemented as functions in the modules (e.g. `y2018` below) and can be
//! executed using `cargo test` with the appropriate scope selection
//!
//! ```bash
//! $ cargo test 2018::day1
//! running 2 tests
//! test y2018::day1::examples ... ok
//! test y2018::day1::solution ... ok
//! ```
//!
//! [Advent of Code]: https://adventofcode.com/
pub mod y2017;
pub mod y2018;

/// Results and Optionals are unwrapped on trust
pub const TRUST: &str = "Input guarantees not met";

/// Results and Optionals that can be unwrapped obviously
pub const OBVIOUS: &str = "Wrong assumption";

/// Read non-empty lines from input
pub fn lines(input: &str) -> impl Iterator<Item = &str> {
    input.lines().filter_map(|s| {
        let i = s.trim();
        if i.is_empty() {
            None
        } else {
            Some(i)
        }
    })
}

/// Read lines from the input, parse them and collect them into a vector
pub fn parse_lines<T: std::str::FromStr>(input: &str) -> Vec<T> {
    lines(input).filter_map(|s| s.parse().ok()).collect()
}

/// Split the input into segments, parse them and collect them into a vector
pub fn parse_splits<T: std::str::FromStr>(input: &str, delimit: &str) -> Vec<T> {
    input
        .split(|c| delimit.contains(c))
        .filter_map(|s| s.parse().ok())
        .collect()
}
