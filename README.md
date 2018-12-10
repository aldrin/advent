## Advent of Code

[![Crates.io](https://img.shields.io/crates/v/advent.svg)](https://crates.io/crates/advent)

[Advent of Code] puzzles are excellent problems for putting Rust iterators and related features to use. This repository 
holds my solutions to the puzzles I've attempted so far. The library documentation (at [docs.rs/advent]) is a convenient 
way to browse the code (e.g. see [2018 solutions])

```bash
$ cargo -q test 2017::day3  # to run a specific puzzle

running 2 tests
test y2017::day3::examples ... ok
test y2017::day3::solution ... ok

$ cargo test --release # to run all puzzles
```

[docs.rs/advent]: https://docs.rs/advent/
[Advent of Code]: https://adventofcode.com/
[2018 solutions]: https://docs.rs/crate/advent/0.2.2/source/src/y2018/mod.rs