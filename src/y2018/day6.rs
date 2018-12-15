// Copyright 2018 by Aldrin J D'Souza.
// Licensed under the MIT License <https://opensource.org/licenses/MIT>

use std::str::FromStr;

use super::super::{parse_lines, parse_splits, TRUST};

#[derive(Debug)]
pub struct Point(i16, i16);

/// A 2-d tree node
pub enum KDTree {
    /// Leaf node with or without a point
    Leaf(Option<Point>),
    /// A non-leaf node with two subtrees
    Node(Point, Option<Box<KDTree>>, Option<Box<KDTree>>),
}

pub fn read(_points: Vec<Point>) -> KDTree {
//    let mut axis = 0;
//    let mut range = 0..points.len();
//    let n = points.len();
//
//    // Add each point
//    for i in 0..n {
//        // Pick the dimension for the tree layer
//        axis += 1;
//        axis %= 2;
//
//        let median = (range.end - range.start) / 2;
//        points[range].sort_by_key(*dimensions[current_dimension]);
//
//        println!("{:#?}", points);
//    }

    KDTree::Leaf(None)
}

impl FromStr for Point {
    type Err = &'static str;

    /// Read a point from a string input
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n = parse_splits(s, ", ");
        if n.len() == 2 {
            Ok(Point(n[0], n[1]))
        } else {
            Err(TRUST)
        }
    }
}

pub fn part1(input: &str) -> usize {
    let points = parse_lines::<Point>(input);
    let _tree = read(points);

    0
}

#[test]
fn examples() {
    let input = r"
    1, 1
    1, 6
    8, 3
    3, 4
    5, 5
    8, 9
    ";
    assert_eq!(10, part1(input));
}

#[test]
fn solution() {
    let input = include_str!("input/6");
    assert_eq!(11310, part1(input));
}
