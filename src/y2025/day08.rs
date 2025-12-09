use core::fmt;
use std::{cmp::Reverse, collections::BinaryHeap, rc::Rc};

use itertools::Itertools;

use crate::utils::union_find::UnionFind;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
struct Coord {
    x: i64,
    y: i64,
    z: i64,
}

impl Coord {
    fn dist_sq(&self, other: &Self) -> usize {
        let dx = (self.x - other.x).abs() as usize;
        let dy = (self.y - other.y).abs() as usize;
        let dz = (self.z - other.z).abs() as usize;

        dx * dx + dy * dy + dz * dz
    }
}

impl TryFrom<&str> for Coord {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let parts = s.split(',').collect::<Vec<_>>();

        if parts.len() != 3 {
            return Err(());
        }

        let x = parts[0].parse::<i64>();
        let y = parts[1].parse::<i64>();
        let z = parts[2].parse::<i64>();

        match (x, y, z) {
            (Ok(x), Ok(y), Ok(z)) => Ok(Coord { x, y, z }),
            _ => Err(())
        }
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[derive(Eq, Ord)]
struct Edge {
    src: Coord,
    dst: Coord,
    dist_sq: usize,
}

impl Edge {
    fn new(src: &Coord, dst: &Coord) -> Self {
        Self {
            src: src.clone(),
            dst: dst.clone(),
            dist_sq: src.dist_sq(dst),
        }
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.dist_sq == other.dist_sq
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.dist_sq.partial_cmp(&other.dist_sq)
    }
}

fn parse_coordinates() -> Vec<UnionFind<Coord>> {
    std::io::stdin().lines().flatten()
        .filter_map(|line| {
            if let Ok(coord) = Coord::try_from(line.as_str()) {
                Some(UnionFind::new(coord))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

fn get_edges(coords: &Vec<UnionFind<Coord>>) -> BinaryHeap<Reverse<Edge>> {
    let mut res = BinaryHeap::new();

    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            res.push(Reverse(Edge::new(&coords[i].data(), &coords[j].data())));
        }
    }

    res
}

pub fn part1() -> i64 {
    let coords = parse_coordinates();

    let mut edges = get_edges(&coords);

    for _ in 0..1000 {
        if let Some(Reverse(Edge { src, dst, .. })) = edges.pop() {
            let mut src = UnionFind(Rc::clone(&coords.iter().find(|n| *n.data() == src).unwrap().0));
            let mut dst = UnionFind(Rc::clone(&coords.iter().find(|n| *n.data() == dst).unwrap().0));

            _ = src.union(&mut dst);
        }
    }

    let union_by_count = coords.iter()
        .map(|uf_node| *uf_node.find().data())
        .counts();

    let mut counts = union_by_count.into_values().collect::<Vec<_>>();

    counts.sort();

    counts.iter().cloned().rev().take(3).product::<usize>() as i64
}

pub fn part2() -> i64 {
    let coords = parse_coordinates();

    let mut edges = get_edges(&coords);

    let mut res = 0i64;

    while let Some(Reverse(Edge { src, dst, .. })) = edges.pop() {
        let mut src = UnionFind(Rc::clone(&coords.iter().find(|n| *n.data() == src).unwrap().0));
        let mut dst = UnionFind(Rc::clone(&coords.iter().find(|n| *n.data() == dst).unwrap().0));

        _ = src.union(&mut dst);

        let union_by_count = coords.iter()
            .map(|uf_node| *uf_node.find().data())
            .counts();

        res = src.data().x * dst.data().x;

        if union_by_count.len() == 1 {
            break;
        }
    }

    res as i64
}
