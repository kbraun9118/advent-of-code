use std::{collections::HashMap, hash::Hash};

use aoc_2025::{Error, print_output, read_input};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position3 {
    x: isize,
    y: isize,
    z: isize,
}

impl Position3 {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    fn distance_from(self, other: Self) -> isize {
        let x_offset = self.x - other.x;
        let y_offset = self.y - other.y;
        let z_offset = self.z - other.z;

        (x_offset * x_offset + y_offset * y_offset + z_offset * z_offset).isqrt()
    }
}

fn min_distance_pairs(coords: &Vec<Position3>) -> Vec<(Position3, Position3, isize)> {
    let mut out = Vec::new();
    for (i, left) in coords[..coords.len() - 1].iter().enumerate() {
        for right in &coords[i + 1..] {
            let current_distance = left.distance_from(*right);
            out.push((*left, *right, current_distance));
        }
    }

    out.sort_by_key(|(_, _, k)| *k);

    out
}

#[derive(Debug)]
struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] != i {
            self.parent[i] = self.find(self.parent[i]);
        }
        self.parent[i]
    }

    fn union(&mut self, a: usize, b: usize) {
        let mut ra = self.find(a);
        let mut rb = self.find(b);
        if ra == rb {
            return;
        }

        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }

        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
    }
}

fn part_1(nodes: &Vec<Position3>, distance_pairs: &Vec<(Position3, Position3, isize)>) -> usize {
    let mut indecies = HashMap::new();

    for (i, &node) in nodes.iter().enumerate() {
        indecies.insert(node, i);
    }

    let mut dsu = UnionFind::new(nodes.len());
    for &(l, r, _) in &distance_pairs[..1000] {
        let li = indecies[&l];
        let ri = indecies[&r];
        dsu.union(li, ri);
    }

    let mut out = HashMap::new();
    for i in 0..nodes.len() {
        let root = dsu.find(i);
        out.insert(root, dsu.size[root]);
    }

    let mut lens = out.values().collect::<Vec<_>>();
    lens.sort();
    lens.reverse();

    lens.into_iter().take(3).fold(1, |acc, &v| acc * v)
}

fn part_2(nodes: &Vec<Position3>, distance_pairs: &Vec<(Position3, Position3, isize)>) -> usize {
    let mut indecies = HashMap::new();

    for (i, &node) in nodes.iter().enumerate() {
        indecies.insert(node, i);
    }

    let mut dsu = UnionFind::new(nodes.len());
    for &(l, r, _) in distance_pairs {
        let li = indecies[&l];
        let ri = indecies[&r];
        dsu.union(li, ri);
        let current = dsu.find(0);

        if (1..nodes.len()).map(|i| dsu.find(i)).all(|p| p == current) {
            return l.x as usize * r.x as usize;
        }
    }

    0
}

fn main() -> Result<(), Error> {
    let input = read_input("08")?
        .into_iter()
        .map(|s| {
            s.split(",")
                .map(|p| p.parse::<isize>())
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|coords| Position3::new(coords[0], coords[1], coords[2]))
        .collect::<Vec<_>>();

    let distance_pairs = min_distance_pairs(&input);

    print_output!(
        part_1(&input, &distance_pairs),
        part_2(&input, &distance_pairs)
    );
    Ok(())
}
