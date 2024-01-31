use aoc_2015::lines_for_day;

fn main() {
    let mut grid = Grid::new(100);

    lines_for_day("18")
        .iter()
        .enumerate()
        .for_each(|(y, r)| {
            r.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .for_each(|(x, _)| grid.insert((x, y)))
        });

    let mut part_1 = grid.clone();

    for _ in 0..100 {
        part_1 = part_1.step();
    }

    println!(
        "Part 1: {}",
        part_1
            .inner
            .iter()
            .fold(0, |acc, r| acc + r.iter().filter(|c| c.is_some()).count())
    );

    for _ in 0..100 {
        grid = grid.step_2();
    }

    println!(
        "Part 2: {}",
        grid.inner
            .iter()
            .fold(0, |acc, r| acc + r.iter().filter(|c| c.is_some()).count())
    );
}

#[derive(Clone)]
struct Grid {
    size: usize,
    inner: Vec<Vec<Option<()>>>,
}

type Point = (usize, usize);

impl Grid {
    fn new(size: usize) -> Self {
        Self {
            size,
            inner: vec![vec![None; size]; size],
        }
    }

    fn step(&self) -> Self {
        let mut new = Self::new(self.size);
        self.inner.iter().enumerate().for_each(|(y, r)| {
            r.iter().enumerate().for_each(|(x, c)| {
                let neighbors = self
                    .neighbors((x, y))
                    .iter()
                    .filter(|o| o.is_some())
                    .count();

                if c.is_some() && (neighbors == 2 || neighbors == 3) {
                    new.insert((x, y));
                } else if c.is_none() && neighbors == 3 {
                    new.insert((x, y));
                }
            });
        });
        new
    }

    fn step_2(&mut self) -> Self {
        self.insert((0, 0));
        self.insert((0, self.size - 1));
        self.insert((self.size - 1, self.size - 1));
        self.insert((self.size - 1, 0));

        let mut next = self.step();

        next.insert((0, 0));
        next.insert((0, next.size - 1));
        next.insert((next.size - 1, next.size - 1));
        next.insert((next.size - 1, 0));

        next
    }

    #[allow(dead_code)]
    fn visual(&self) {
        for row in self.inner.iter() {
            for column in row {
                print!("{}", if column.is_some() { '#' } else { '.' })
            }
            println!("")
        }
    }

    fn insert(&mut self, (x, y): Point) {
        self.inner[y][x] = Some(());
    }

    fn neighbors(&self, (x, y): Point) -> Vec<&Option<()>> {
        let y_bound = if y == 0 {
            y..=y + 1
        } else if y == self.size - 1 {
            y - 1..=y
        } else {
            y - 1..=y + 1
        };
        let x_bound = if x == 0 {
            x..=x + 1
        } else if x == self.size - 1 {
            x - 1..=x
        } else {
            x - 1..=x + 1
        };
        let mut neighbors = vec![];
        for y_diff in y_bound {
            for x_diff in x_bound.clone() {
                if x_diff == x && y_diff == y {
                    continue;
                }
                neighbors.push(&self.inner[y_diff][x_diff])
            }
        }
        neighbors
    }
}
