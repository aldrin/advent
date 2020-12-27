// Copyright 2018 by Aldrin J D'Souza.
// Licensed under the MIT License <https://opensource.org/licenses/MIT>
//! Chronal Caliberation ([Statement](https://adventofcode.com/2018/day/6)).

use std::prelude::v1::Vec;
use std::str::FromStr;

use crate::OBVIOUS;
use crate::parse_splits;
use crate::TRUST;

/// A two dimensional point
#[derive(Debug)]
pub struct Point(u16, u16);

impl FromStr for Point {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c: Vec<u16> = parse_splits(s, ", ");
        match c.len() {
            2 => Ok(Point(c[0], c[1])),
            _ => Err(crate::TRUST)
        }
    }
}

pub fn distance(p: &Point, x: u16, y: u16) -> i32 {
    (p.0 as i32 - x as i32).abs() + (p.1 as i32 - y as i32).abs()
}


pub fn part1(points: &[Point]) {
    let width = points.iter().map(|p| p.0).max().expect(OBVIOUS);
    let height = points.iter().map(|p| p.1).max().expect(OBVIOUS);

    let mut grid: Vec<Vec<i32>> = vec![vec![-1; height as usize]; width as usize];


    for x in 0..=width {
        for y in 0..=height {
            let distances: Vec<_> = points.iter().enumerate().map(|(i, p)| (i, distance(p, x, y))).collect();
            let least = distances.iter().min_by_key(|x| x.1).expect(OBVIOUS);
            let count = distances.iter().filter(|x| x.1 == least.1).count();
            if count == 1 { grid[x as usize][y as usize] = least.0 as i32 }
        }
    }

    dbg!(points);
    print_grid(&grid);
}

pub fn print_grid(grid: &Vec<Vec<i32>>) {
    for row in grid {
        for col in row {
            print!("{} ", match col
                {
                    x if *x >= 0 => (('a' as u8) + (*x as u8)) as char,
                    _ => '.'
                }
            )
        }
        println!()
    }
}


#[test]
pub fn examples() {
    let input = r"
    1, 1
    1, 6
    8, 3
    3, 4
    5, 5
    8, 9";
    let points = crate::parse_lines::<Point>(input);
    part1(&points);
    assert!(true);
}

#[test]
pub fn solution() {
    assert!(true);
}
