// Copyright 2018 by Aldrin J D'Souza.
// Licensed under the MIT License <https://opensource.org/licenses/MIT>
//! Chronal Caliberation ([Statement](https://adventofcode.com/2018/day/6)).

use std::ops::Index;
use std::str::FromStr;

use crate::{parse_lines, parse_splits, TRUST};

/// A point in the tree
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Point(u16, u16);

impl FromStr for Point {
    type Err = &'static str;

    /// Read a point from a string input
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n = parse_splits(s, ", ");
        match n.len() as u8 {
            2 => Ok(Point(n[0], n[1])),
            _ => Err(TRUST),
        }
    }
}

impl Index<u8> for Point {
    type Output = u16;

    /// Get the coordinate value at the given index
    fn index(&self, index: u8) -> &u16 {
        match index {
            0 => &self.0,
            _ => &self.1
        }
    }
}

/// Get the distance between two points
pub fn distance(p: Point, q: Point) -> u16 {
    (p.0 as i32 - q.0 as i32).abs() as u16 + (p.1 as i32 - q.1 as i32).abs() as u16
}

/// A k-dimensional search tree
#[derive(Debug)]
pub enum KdTree {
    /// Empty
    Nil,
    /// A leaf node
    Leaf(Point),
    /// An intermediate node
    Node(u8, Point, Box<KdTree>, Box<KdTree>),
}

/// Build a  k-d tree with the given points
pub fn kd_tree(points: &mut [Point]) -> KdTree {
    build(points, 0)
}

/// Build a tree recursively
fn build(points: &mut [Point], dimension: u8) -> KdTree {
    match points.len() {
        0 => KdTree::Nil,

        1 => KdTree::Leaf(points[0]),

        n => {
            // Sort the points by the chosen dimension
            points[..].sort_by_key(|p| p[dimension]);

            // The root is the median
            let median = n / 2;

            // Advance to the next dimension and build the children subtrees
            let next = (dimension + 1) % 2;
            let left = Box::new(build(&mut points[..median], next));
            let right = Box::new(build(&mut points[median + 1..], next));

            KdTree::Node(dimension, points[median], left, right)
        }
    }
}

fn nearest(tree: &KdTree, candidate: Option<Point>, query: Point) -> Option<Point> {
    match tree {
        KdTree::Nil => None,

        KdTree::Leaf(p) => match candidate {
            Some(q) if distance(*p, query) > distance(q, query) => Some(q),
            _ => Some(*p)
        },

        KdTree::Node(dimension, this, left, right) => None
    }
}

//
///// Point ranges
//fn ranges(points: &[Point]) -> [Range<u16>; DIMENSIONS] {
//    let mut ranges = [0..0; DIMENSIONS];
//
//    for d in 0..2 {
//        let min = points.iter().map(|p| p[d]).max().expect(TRUST);
//        let max = points.iter().map(|p| p[d]).min().expect(TRUST);
//        ranges[d] = min..max
//    }
//
//    ranges
//}

/// Check if the query point is contained in the tree
pub fn contains(tree: &KdTree, query: Point) -> bool {
    match tree {
        KdTree::Nil => false,

        KdTree::Leaf(p) => query == *p,

        KdTree::Node(_, p, _, _) if query == *p => true,

        KdTree::Node(dimension, p, left, right) => {
            if query[*dimension] < p[*dimension] {
                contains(left, query)
            } else {
                contains(right, query)
            }
        }
    }
}


pub fn nearest_neighbor(tree: &KdTree, query: Point) -> Option<Point> {
    nearest(tree, None, query)
}


/// Find the point with the largest finite area
pub fn part1(input: &str) -> usize {
    let mut points = parse_lines::<Point>(input);

    let tree = kd_tree(&mut points);
    println!("{:#?}", tree);
    assert!(!contains(&tree, Point(0, 0)));
    assert!(contains(&tree, Point(2, 3)));
    assert_eq!(Some(Point(5, 4)), nearest_neighbor(&tree, Point(4, 4)));
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
    //assert_eq!(10, part1(input));

    let input = r"
    2,3
    5,4
    9,6
    4,7
    8,1
    7,2
    ";

    println!("{}", part1(input));
}

#[test]
fn solution() {
    let input = include_str!("input/6");
    assert_eq!(11310, part1(input));
}
