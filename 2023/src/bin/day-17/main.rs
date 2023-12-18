// use std::collections::{HashMap, HashSet};
//
// type Grid = aoc::Grid<usize>;
// type Coord = aoc::Coord<usize>;
//
// fn parse_input(input: Vec<String>) -> Grid {
//     Grid::from(
//         input
//             .into_iter()
//             .map(|l| {
//                 l.chars()
//                     .map(|c| c.to_digit(10).unwrap() as usize)
//                     .collect::<Vec<_>>()
//             })
//             .collect::<Vec<_>>(),
//     )
// }
//
// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// enum Direction {
//     Up,
//     Down,
//     Left,
//     Right,
// }
//
// impl From<(Coord, Coord)> for Direction {
//     fn from((from, to): (Coord, Coord)) -> Self {
//         match (
//             from.x as isize - to.x as isize,
//             from.y as isize - to.y as isize,
//         ) {
//             (-1, _) => Direction::Right,
//             (1, _) => Direction::Left,
//             (_, -1) => Direction::Down,
//             (_, 1) => Direction::Up,
//             _ => panic!("invalid movement: From: {from:?}, To: {to:?}"),
//         }
//     }
// }
//
// #[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
// struct StepCoord {
//     coord: Coord,
//     direction: Direction,
//     steps: usize,
// }
//
// impl StepCoord {
//     fn new(coord: Coord, steps: usize, direction: Direction) -> Self {
//         Self {
//             coord,
//             steps,
//             direction,
//         }
//     }
//
//     fn path_to(self, to: Coord) -> Self {
//         let direction = Direction::from((self.coord, to));
//         Self {
//             coord: to,
//             steps: if direction == self.direction {
//                 self.steps + 1
//             } else {
//                 0
//             },
//             direction,
//         }
//     }
// }
//
// fn lowest_heat(grid: &Grid) -> usize {
//     let mut dist = HashMap::new();
//     let step_coord = StepCoord::new((0, 0).into(), 0, Direction::Up);
//     dist.insert(step_coord, 0);
//     let mut queue = HashSet::new();
//     queue.insert(step_coord);
//
//     let end = Coord::from((grid.width() - 1, grid.height() - 1));
//     while !queue.is_empty() {
//         let u = queue.iter().min_by_key(|u| dist[u]).unwrap().clone();
//         let dist_u = dist[&u];
//         queue.remove(&u);
//
//         // if u.coord == end {
//         //     break;
//         // }
//
//         let neighbors = grid
//             .cardinal_neighbors_coords(u.coord)
//             .into_iter()
//             .map(|c| u.path_to(c))
//             .filter(|n| n.steps < 3);
//
//         for neighbor in neighbors {
//             let alt = dist_u + grid[neighbor.coord];
//
//             if alt < *dist.get(&neighbor).unwrap_or(&usize::MAX) {
//                 dist.insert(neighbor, alt);
//                 queue.insert(neighbor);
//             }
//         }
//     }
//
//     print_direction(grid, &dist);
//
//     let step_coords = dist.keys().filter(|d| d.coord == end).collect::<Vec<_>>();
//     dist[&step_coords.into_iter().min_by_key(|d| dist[*d]).unwrap()]
// }
//
// fn print_direction(grid: &Grid, dist: &HashMap<StepCoord, usize>) {
//     for y in 0..grid.height() {
//         for x in 0..grid.width() {
//             let step_coords = dist
//                 .keys()
//                 .filter(|d| d.coord == (x, y).into())
//                 .collect::<Vec<_>>();
//             let coord = step_coords.into_iter().min_by_key(|d| dist[*d]).unwrap();
//             print!(
//                 "{}",
//                 match coord.direction {
//                     Direction::Up => '^',
//                     Direction::Down => 'v',
//                     Direction::Left => '<',
//                     Direction::Right => '>',
//                 }
//             );
//         }
//         println!();
//     }
// }
//
// fn main() {
//     let input = aoc::read_input_lines("17");
//
//     let grid = parse_input(input);
//
//     aoc::print_part_1(lowest_heat(&grid));
// }
//
// #[cfg(test)]
// mod test {
//     use super::*;
//
//     fn get_test_input() -> Grid {
//         let input = r"2413432311323
// 3215453535623
// 3255245654254
// 3446585845452
// 4546657867536
// 1438598798454
// 4457876987766
// 3637877979653
// 4654967986887
// 4564679986453
// 1224686865563
// 2546548887735
// 4322674655533"
//             .lines()
//             .map(String::from)
//             .collect::<Vec<_>>();
//
//         parse_input(input)
//     }
//
//     #[test]
//     fn test_part_1() {
//         assert_eq!(lowest_heat(&get_test_input()), 102);
//     }
//
//     #[test]
//     fn from_direction() {
//         assert_eq!(
//             Direction::from((Coord { x: 0, y: 1 }, Coord { x: 0, y: 0 })),
//             Direction::Up
//         );
//     }
// }
//

//TODO Reimplement idk man

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::str::FromStr;

// advent_of_code::solution!(17);

const GRID_SIZE: usize = 141;

#[derive(Copy, Clone, Eq, Debug, Ord, PartialEq, PartialOrd)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_left(self) -> Self {
        match self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
        }
    }

    fn turn_right(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct JourneyState {
    position: usize,
    facing: Direction,
    heat_loss: u32,
}

impl Ord for JourneyState {
    fn cmp(&self, other: &Self) -> Ordering {
        // for use in max heap, so less heat loss = better/greater
        match self.heat_loss.cmp(&other.heat_loss) {
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => (self.position, self.facing).cmp(&(other.position, other.facing)),
        }
    }
}

impl PartialOrd for JourneyState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // for use in max heap, so less heat_loss = better/greater
        match self.heat_loss.cmp(&other.heat_loss) {
            Ordering::Greater => Some(Ordering::Less),
            Ordering::Less => Some(Ordering::Greater),
            Ordering::Equal => {
                (self.position, self.facing).partial_cmp(&(other.position, other.facing))
            }
        }
    }
}

#[derive(Debug)]
struct JourneyVisitTracker {
    visited: [u32; GRID_SIZE * GRID_SIZE * 4],
}

impl JourneyVisitTracker {
    fn new() -> Self {
        Self {
            visited: [u32::MAX; GRID_SIZE * GRID_SIZE * 4],
        }
    }

    fn minimum(&self, pos: usize) -> Option<u32> {
        let base = pos * 4;
        self.visited[base..base + 4].iter().min().copied()
    }

    fn visit(&mut self, state: &JourneyState) -> bool {
        let dir = match state.facing {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 2,
            Direction::West => 3,
        };
        let key = (4 * state.position) + dir;

        if self.visited[key] <= state.heat_loss {
            true
        } else {
            self.visited[key] = state.heat_loss;
            false
        }
    }
}

#[derive(Debug, PartialEq)]
struct City {
    grid: [u32; GRID_SIZE * GRID_SIZE],
}

impl City {
    fn minimal_heat_loss(&self, min_dist: usize, max_dist: usize) -> Option<u32> {
        let mut visited = JourneyVisitTracker::new();
        let mut queue = BinaryHeap::new();
        for state in self.initial_states(min_dist, max_dist) {
            if !visited.visit(&state) {
                queue.push(state);
            }
        }

        while let Some(state) = queue.pop() {
            for reachable in self.reachable_states(&state, min_dist, max_dist) {
                if !visited.visit(&reachable) {
                    queue.push(reachable);
                }
            }
        }

        visited.minimum((GRID_SIZE * GRID_SIZE) - 1)
    }

    fn states_in_directions<'a>(
        &'a self,
        position: usize,
        heat_loss: u32,
        directions: impl Iterator<Item = Direction> + 'a,
        min_dist: usize,
        max_dist: usize,
    ) -> impl Iterator<Item = JourneyState> + 'a {
        directions.flat_map(move |facing| {
            let mut states = Vec::new();
            let mut extra_loss = 0;
            for dist in 1..=max_dist {
                if let Some(position) = City::step(position, facing, dist) {
                    extra_loss += self.grid[position];
                    if dist >= min_dist {
                        states.push(JourneyState {
                            position,
                            facing,
                            heat_loss: heat_loss + extra_loss,
                        });
                    }
                }
            }
            states
        })
    }

    fn initial_states(
        &self,
        min_dist: usize,
        max_dist: usize,
    ) -> impl Iterator<Item = JourneyState> + '_ {
        self.states_in_directions(
            0,
            0,
            [Direction::East, Direction::South].into_iter(),
            min_dist,
            max_dist,
        )
    }

    fn reachable_states<'a>(
        &'a self,
        state: &'a JourneyState,
        min_dist: usize,
        max_dist: usize,
    ) -> impl Iterator<Item = JourneyState> + '_ {
        self.states_in_directions(
            state.position,
            state.heat_loss,
            [state.facing.turn_left(), state.facing.turn_right()].into_iter(),
            min_dist,
            max_dist,
        )
    }

    fn step(pos: usize, dir: Direction, dist: usize) -> Option<usize> {
        let row = pos / GRID_SIZE;
        let col = pos % GRID_SIZE;
        match dir {
            Direction::North => pos.checked_sub(GRID_SIZE * dist),
            Direction::East => {
                if (col + dist) < GRID_SIZE {
                    Some(pos + dist)
                } else {
                    None
                }
            }
            Direction::South => {
                if (row + dist) < GRID_SIZE {
                    Some(pos + (GRID_SIZE * dist))
                } else {
                    None
                }
            }
            Direction::West => {
                if col.checked_sub(dist).is_some() {
                    Some(pos - dist)
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
struct ParseCityError;

impl FromStr for City {
    type Err = ParseCityError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut grid = [0; GRID_SIZE * GRID_SIZE];

        for (row, line) in input.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if let Some(value) = ch.to_digit(10) {
                    grid[(row * GRID_SIZE) + col] = value;
                } else {
                    return Err(ParseCityError);
                }
            }
        }

        Ok(City { grid })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    if let Ok(city) = City::from_str(input) {
        city.minimal_heat_loss(1, 3)
    } else {
        None
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    if let Ok(city) = City::from_str(input) {
        city.minimal_heat_loss(4, 10)
    } else {
        None
    }
}

fn main() {
    let lines = aoc::read_input_lines("17").join("\n");

    aoc::print_part_1(part_one(&lines).unwrap());
    aoc::print_part_2(part_two(&lines).unwrap());
}
