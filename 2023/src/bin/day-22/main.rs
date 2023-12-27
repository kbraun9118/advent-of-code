use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Coord {
    x: usize,
    y: usize,
    z: usize,
}

impl From<&str> for Coord {
    fn from(value: &str) -> Self {
        let mut split = value.split(",");
        Self {
            x: split.next().unwrap().parse().unwrap(),
            y: split.next().unwrap().parse().unwrap(),
            z: split.next().unwrap().parse().unwrap(),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Bar {
    from: Coord,
    to: Coord,
}

impl Bar {
    fn coords(&self) -> Vec<Coord> {
        if self.from.x == self.to.x && self.from.y == self.to.y {
            (self.from.z..=self.to.z)
                .into_iter()
                .map(|z| Coord {
                    x: self.from.x,
                    y: self.from.y,
                    z,
                })
                .collect()
        } else if self.from.x == self.to.x && self.from.z == self.to.z {
            (self.from.y..=self.to.y)
                .into_iter()
                .map(|y| Coord {
                    x: self.from.x,
                    y,
                    z: self.from.z,
                })
                .collect()
        } else {
            (self.from.x..=self.to.x)
                .into_iter()
                .map(|x| Coord {
                    x,
                    y: self.from.y,
                    z: self.from.z,
                })
                .collect()
        }
    }

    fn contains(&self, other: Coord) -> bool {
        if (self.from.x..=self.to.x).contains(&other.x)
            && (self.from.y..=self.to.y).contains(&other.y)
            && (self.from.z..=self.to.z).contains(&other.z)
        {
            true
        } else {
            false
        }
    }

    fn is_below(&self, other: Bar) -> bool {
        if *self == other {
            false
        } else {
            self.coords()
                .iter()
                .map(|c| Coord { z: c.z + 1, ..*c })
                .any(|c| other.contains(c))
        }
    }

    fn move_down(&self) -> Self {
        Self {
            from: Coord {
                z: self.from.z - 1,
                ..self.from
            },
            to: Coord {
                z: self.to.z - 1,
                ..self.to
            },
        }
    }
}

impl From<&str> for Bar {
    fn from(value: &str) -> Self {
        let (left, right) = value.split_once("~").unwrap();
        let left = Coord::from(left);
        let right = Coord::from(right);
        Self {
            from: Coord {
                x: left.x.min(right.x),
                y: left.y.min(right.y),
                z: left.z.min(right.z),
            },
            to: Coord {
                x: left.x.max(right.x),
                y: left.y.max(right.y),
                z: left.z.max(right.z),
            },
        }
    }
}

#[derive(Debug, Clone)]
struct Tower {
    bars: HashSet<Bar>,
    max_z: usize,
}

impl Tower {
    fn layer(&self, z: usize) -> Vec<Bar> {
        self.bars
            .iter()
            .cloned()
            .filter(|c| c.from.z == z)
            .collect()
    }

    fn move_bar(&mut self, bar: Bar) -> bool {
        let mut bar = bar;
        self.bars.remove(&bar);
        let mut moved = false;

        while bar.from.z > 0 && !self.bars.iter().any(|b| b.is_below(bar)) {
            moved = true;
            bar = bar.move_down();
        }

        self.bars.insert(bar);

        self.max_z = self
            .bars
            .iter()
            .map(|c| c.to.z)
            .max()
            .unwrap();

        moved
    }

    fn settle(&mut self) -> usize {
        let mut i = 0;
        for z in 0..=self.max_z {
            for bar in self.layer(z) {
                if self.move_bar(bar) {
                    i += 1;
                }
            }
        }
        i
    }

    fn remove(&mut self, bar: Bar) {
        self.bars.remove(&bar);
    }

    fn supports(&self) -> HashMap<Bar, Vec<Bar>> {
        self.bars
            .iter()
            .map(|b| {
                (
                    b.clone(),
                    self.bars
                        .iter()
                        .cloned()
                        .filter(|other| b.is_below(*other))
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<HashMap<_, _>>()
    }

    fn supported_by(&self) -> HashMap<Bar, Vec<Bar>> {
        let mut supported_by = HashMap::new();
        let supports = self.supports();
        for (key, _) in supports.iter() {
            let supported = supports
                .iter()
                .filter(|(_, v)| v.contains(key))
                .map(|(k, _)| (*k).clone())
                .collect::<Vec<_>>();
            supported_by.insert(*key, supported);
        }

        supported_by
    }
}

impl From<Vec<String>> for Tower {
    fn from(value: Vec<String>) -> Self {
        let bars = value
            .into_iter()
            .map(|s| Bar::from(s.as_str()))
            .collect::<HashSet<_>>();
        let max_z = bars.iter().map(|c| c.from.z.max(c.to.z)).max().unwrap();

        Self { bars, max_z }
    }
}

fn part_1(tower: &Tower) -> usize {
    let mut tower = tower.clone();
    tower.settle();

    let supports = tower.supports();
    let supported_by = tower.supported_by();

    supports
        .into_iter()
        .filter(|(_, v)| v.into_iter().all(|s| supported_by[s].len() > 1))
        .count()
}

fn part_2(tower: &Tower) -> usize {
    let mut tower = tower.clone();
    tower.settle();
    let mut count = 0;
    
    for i in 0..tower.max_z {
        for bar in tower.layer(i) {
            let mut current = tower.clone();
            current.remove(bar);
            count += current.settle();
        }
    }

    count
}

fn main() {
    let input = aoc::read_input_lines("22");
    aoc::benchmark(|| {
        let tower = Tower::from(input);

        aoc::print_part_1(part_1(&tower));
        aoc::print_part_2(part_2(&tower));
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_contains() {
        let x_bar = Bar::from("0,0,0~4,0,0");
        let y_bar = Bar::from("0,0,0~0,4,0");
        let z_bar = Bar::from("0,0,0~0,0,4");
        for i in 0..5 {
            assert!(x_bar.contains(Coord { x: i, y: 0, z: 0 }));
            assert!(y_bar.contains(Coord { x: 0, y: i, z: 0 }));
            assert!(z_bar.contains(Coord { x: 0, y: 0, z: i }));
        }

        assert!(!x_bar.contains(Coord { x: 5, y: 0, z: 0 }));
        assert!(!y_bar.contains(Coord { x: 0, y: 5, z: 0 }));
        assert!(!z_bar.contains(Coord { x: 0, y: 0, z: 5 }));
    }

    #[test]
    fn test_is_above_below() {
        let above = Bar::from("1,0,1~1,2,1");
        let below = Bar::from("0,1,0~2,1,0");

        assert!(below.is_below(above));
    }

    fn get_test_input() -> Tower {
        r"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"
            .lines()
            .map(String::from)
            .collect::<Vec<_>>()
            .into()
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_test_input()), 5);
    }
    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&get_test_input()), 7);
    }
}
