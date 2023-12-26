use std::collections::HashSet;

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

    fn is_above(&self, other: Bar) -> bool {
        self.coords().iter().map(|c| Coord { z: c.z - 1, ..*c}).any(|c| other.contains(c))
    }

    fn is_below(&self, other: Bar) -> bool {
        self.coords().iter().map(|c| Coord { z: c.z + 1, ..*c}).any(|c| other.contains(c))
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
}

impl From<Vec<String>> for Tower {
    fn from(value: Vec<String>) -> Self {
        Self {
            bars: value.into_iter().map(|s| Bar::from(s.as_str())).collect(),
        }
    }
}

fn main() {
    let input = r"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"
        .lines()
        .map(String::from)
        .collect::<Vec<_>>();

    let tower = Tower::from(input);

    println!("{tower:#?}");
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

        assert!(above.is_above(below));
        assert!(below.is_below(above));
    }

    #[test]
    fn test_part_1() {
        assert_eq!(1, 1);
    }
}
