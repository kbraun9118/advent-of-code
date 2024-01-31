use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::io::BufRead;
use std::ops::Deref;
use std::ops::Index;
use std::ops::IndexMut;
use std::collections::HashMap;

fn main() {
    let reader = BufReader::new(
        File::open(Path::new("../input/2018/06/input.txt")).unwrap());

    let points: Vec<Point> = reader.lines()
        .map(|line| line.unwrap())
        .map(|line| Point::from(line))
        .collect();

    let max_x = points.iter()
        .map(|point| point.x)
        .max()
        .unwrap();

    let max_y = points.iter()
        .map(|point| point.y)
        .max()
        .unwrap();

    let mut grid = Grid::new(max_x, max_y);

    points.iter()
        .for_each(|point| grid.insert_point(point));

    points.iter().for_each(|point| grid.set_distances(point));

    let mut map: HashMap<Point, usize> = HashMap::new();

    grid.iter().for_each(|vec| vec.iter().for_each(|(point_vec, _)| {
        if point_vec.len() == 1 {
            match map.get_mut(point_vec.get(0).unwrap()) {
                Some(value) => *value += 1,
                None => { map.insert(*point_vec.get(0).unwrap(), 2); }
            }
        }
    }));

    for i in 0..grid.len() {
        if grid[i][0].0.len() == 1 {
            map.remove(grid[i][0].0.get(0).unwrap());
        }

        if grid[i][grid[i].len() - 1].0.len() == 1 {
            map.remove(grid[i][grid[i].len() - 1].0.get(0).unwrap());
        }
    }

    for i in 0..grid[1].len() {
        if grid[0][i].0.len() == 1 {
            map.remove(grid[0][i].0.get(0).unwrap());
        }

        if grid[grid.len() - 1][i].0.len() == 1 {
            map.remove(grid[grid.len() - 1][i].0.get(0).unwrap());
        }
    }
    
    let mut counter = 0;
    
    for i in 0..grid.len() {
        for j in 0..grid[1].len() {
            if points.iter()
                .map(|point| point.distance_from(i, j))
                .fold(0, |left, right| left + right) < 10_000 {
                counter += 1;
            }
        }
    }

    println!("{:?}", map.iter().max_by(|(_, value_l), (_, value_r)| value_l.cmp(value_r)));
    println!("{}", counter)
}

#[derive(Debug)]
struct Grid(Vec<Vec<(Vec<Point>, usize)>>);

impl Grid {
    fn new(x: usize, y: usize) -> Grid {
        Grid(vec![vec![(Vec::new(), 0); y + 1]; x + 1])
    }

    fn insert_point(&mut self, point: &Point) {
        self[point.x][point.y] = (vec![*point], 0)
    }

    fn set_distances(&mut self, point: &Point) {
        for i in 0..self.len() {
            for j in 0..self[i].len() {
                let distance = point.distance_from(i, j);
                if self[i][j].0.len() == 0 {
                    self[i][j] = (vec![*point], distance);
                } else {
                    if distance == self[i][j].1 {
                        self[i][j].0.push(*point)
                    } else if distance < self[i][j].1 {
                        self[i][j] = (vec![*point], distance);
                    }
                }
            }
        }
    }
}

impl Index<usize> for Grid {
    type Output = Vec<(Vec<Point>, usize)>;

    fn index(&self, index: usize) -> &<Self as Index<usize>>::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, index: usize) -> &mut <Self as Index<usize>>::Output {
        &mut self.0[index]
    }
}

impl Deref for Grid {
    type Target = Vec<Vec<(Vec<Point>, usize)>>;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }

    fn distance_from(&self, x: usize, y: usize) -> usize {
        ((self.x as i32 - x as i32).abs() + (self.y as i32 - y as i32).abs()) as usize
    }
}

impl From<String> for Point {
    fn from(string: String) -> Self {
        let split: Vec<&str> = string.split(',').collect();
        let (x, y): (&str, &str) = (split[0], split[1]);
        Point::new(x.trim().parse().unwrap(), y.trim().parse().unwrap())
    }
}