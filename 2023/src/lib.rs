use core::fmt;
use std::{
    fmt::{Debug, Display},
    ops::{Index, IndexMut},
};

pub fn read_input_lines(day: &'static str) -> Vec<String> {
    let file = std::fs::read_to_string(format!("./src/bin/day-{}/input.txt", day))
        .expect("Could not find file");

    let mut lines = file.lines().map(|s| s.to_string()).collect::<Vec<_>>();
    if lines.last().unwrap() == "" {
        lines.pop();
        lines
    } else {
        lines
    }
}

pub fn print_part_1<T: Display>(part_1: T) {
    println!("Part 1: {}", part_1);
}

pub fn print_part_2<T: Display>(part_2: T) {
    println!("Part 2: {}", part_2);
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coord<T> {
    pub x: T,
    pub y: T,
}

impl<T> From<(T, T)> for Coord<T> {
    fn from((x, y): (T, T)) -> Self {
        Self { x, y }
    }
}

#[derive(Hash)]
pub struct Grid<T> {
    height: usize,
    width: usize,
    grid: Vec<T>,
}

impl<T> Grid<T> {
    pub fn new<F: Fn(Coord<usize>) -> T>(height: usize, width: usize, f: F) -> Self {
        let mut grid = Vec::with_capacity(height * width);
        for y in 0..height {
            for x in 0..width {
                grid.push(f(Coord { x, y }));
            }
        }
        Self {
            height,
            width,
            grid,
        }
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn index_iter<'a>(&'a self) -> impl Iterator<Item = Coord<usize>> + 'a {
        self.grid
            .iter()
            .enumerate()
            .map(|(i, _)| Coord::from((i % self.width, i / self.width)))
    }

    pub fn row(&self, index: usize) -> Vec<&T> {
        let mut vec = vec![];
        for x in 0..self.width {
            vec.push(&self[(x, index)]);
        }

        vec
    }

    pub fn column(&self, index: usize) -> Vec<&T> {
        let mut vec = vec![];
        for y in 0..self.height {
            vec.push(&self[(index, y)]);
        }

        vec
    }
}

impl<T: Default> Grid<T> {
    pub fn new_default(height: usize, width: usize) -> Self {
        let mut grid = Vec::with_capacity(height * width);
        for _ in 0..height * width {
            grid.push(T::default());
        }
        Self {
            height,
            width,
            grid,
        }
    }
}

impl<T> From<Vec<Vec<T>>> for Grid<T> {
    fn from(value: Vec<Vec<T>>) -> Self {
        let height = value.len();
        let width = value[0].len();
        let mut grid = Vec::with_capacity(height * width);
        for row in value {
            for item in row {
                grid.push(item);
            }
        }

        Self {
            height,
            width,
            grid,
        }
    }
}

impl<T: Clone> Clone for Grid<T> {
    fn clone(&self) -> Self {
        Self {
            height: self.height,
            width: self.width,
            grid: self.grid.clone(),
        }
    }
}

impl<T, E: Into<Coord<usize>>> Index<E> for Grid<T> {
    type Output = T;

    fn index(&self, index: E) -> &Self::Output {
        let index = index.into();
        assert!(index.x < self.width, "Cannot index past width");
        assert!(index.y < self.height, "Cannot index past height");
        &self.grid[index.y * self.width + index.x]
    }
}

impl<T, E: Into<Coord<usize>>> IndexMut<E> for Grid<T> {
    fn index_mut(&mut self, index: E) -> &mut Self::Output {
        let index = index.into();
        assert!(index.x < self.width, "Cannot index past width");
        assert!(index.y < self.height, "Cannot index past height");
        &mut self.grid[index.y * self.width + index.x]
    }
}

impl<T: fmt::Debug> fmt::Debug for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut rows = vec![];
        for y in 0..self.height {
            rows.push((
                y,
                self.grid[y * self.width..y * self.width + self.width]
                    .iter()
                    .collect::<Vec<_>>(),
            ));
        }
        f.debug_struct("Grid")
            .field("height", &self.height)
            .field("width", &self.width)
            .field("grid", &rows)
            .finish()
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self[(x, y)])?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl<T: PartialEq> PartialEq for Grid<T> {
    fn eq(&self, other: &Self) -> bool {
        self.grid == other.grid
    }
}

impl<T: Eq> Eq for Grid<T> {}

// impl<T: Hash> Hash for Grid<T> {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn grid_debug() {
        let grid = Grid::from(vec![vec![1, 0], vec![0, 1]]);
        let grid_debug = format!("{grid:?}");

        assert_eq!(
            grid_debug,
            "Grid { height: 2, width: 2, grid: [(0, [1, 0]), (1, [0, 1])] }".to_string()
        );
    }

    #[test]
    fn indexing_grid() {
        let grid = Grid::from(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);

        assert_eq!(grid[(0, 0)], 1);
        assert_eq!(grid[(0, 1)], 4);
        assert_eq!(grid[(0, 2)], 7);
        assert_eq!(grid[(1, 0)], 2);
        assert_eq!(grid[(1, 1)], 5);
        assert_eq!(grid[(1, 2)], 8);
        assert_eq!(grid[(2, 0)], 3);
        assert_eq!(grid[(2, 1)], 6);
        assert_eq!(grid[(2, 2)], 9);
    }

    #[test]
    fn grid_index_iter() {
        let grid = Grid::from(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
        let mut index_iter = grid.index_iter();

        assert_eq!(index_iter.next(), Some((0, 0).into()));
        assert_eq!(index_iter.next(), Some((1, 0).into()));
        assert_eq!(index_iter.next(), Some((2, 0).into()));
        assert_eq!(index_iter.next(), Some((0, 1).into()));
        assert_eq!(index_iter.next(), Some((1, 1).into()));
        assert_eq!(index_iter.next(), Some((2, 1).into()));
        assert_eq!(index_iter.next(), Some((0, 2).into()));
        assert_eq!(index_iter.next(), Some((1, 2).into()));
        assert_eq!(index_iter.next(), Some((2, 2).into()));
        assert_eq!(index_iter.next(), None);
    }

    #[test]
    fn test_index_throws_assert_errors() {
        let grid = Grid::from(vec![vec![0]]);

        let result = std::panic::catch_unwind(|| grid[(1, 0)]);
        assert!(result.is_err());
        let message: &str = *result.unwrap_err().downcast().unwrap();
        assert_eq!(message, "Cannot index past width");

        let result = std::panic::catch_unwind(|| grid[(0, 1)]);
        assert!(result.is_err());
        let message: &str = *result.unwrap_err().downcast().unwrap();
        assert_eq!(message, "Cannot index past height");
    }

    #[test]
    fn grid_row() {
        let grid = Grid::from(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);

        assert_eq!(grid.row(0), vec![&1, &2, &3]);
        assert_eq!(grid.row(1), vec![&4, &5, &6]);
        assert_eq!(grid.row(2), vec![&7, &8, &9]);
    }

    #[test]
    fn grid_column() {
        let grid = Grid::from(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);

        assert_eq!(grid.column(0), vec![&1, &4, &7]);
        assert_eq!(grid.column(1), vec![&2, &5, &8]);
        assert_eq!(grid.column(2), vec![&3, &6, &9]);
    }
}
