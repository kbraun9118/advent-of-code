use std::collections::{BinaryHeap, HashMap};

type Grid = aoc::Grid<usize>;
type Coord = aoc::Coord<usize>;

fn parse_input(input: Vec<String>) -> Grid {
    Grid::from(
        input
            .into_iter()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

impl From<(Coord, Coord)> for Direction {
    fn from((from, to): (Coord, Coord)) -> Self {
        match (
            from.x as isize - to.x as isize,
            from.y as isize - to.y as isize,
        ) {
            (-1, _) => Direction::Right,
            (1, _) => Direction::Left,
            (_, -1) => Direction::Down,
            (_, 1) => Direction::Up,
            _ => panic!("invalid movement: From: {from:?}, To: {to:?}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct StepCoord {
    coord: Coord,
    direction: Direction,
    steps: usize,
}

impl StepCoord {
    fn new(coord: Coord, steps: usize, direction: Direction) -> Self {
        Self {
            coord,
            steps,
            direction,
        }
    }

    fn path_to(self, to: Coord) -> Self {
        let direction = Direction::from((self.coord, to));
        Self {
            coord: to,
            steps: if direction == self.direction {
                self.steps + 1
            } else {
                0
            },
            direction,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct StepCoordState {
    step_coord: StepCoord,
    state: usize,
}

impl Ord for StepCoordState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.state.cmp(&self.state)
    }
}

impl PartialOrd for StepCoordState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.state.partial_cmp(&self.state)
    }
}

fn neighbor_filter_part_1(current: StepCoord, neighbor: StepCoord) -> bool {
    if neighbor.steps > 2 {
        return false;
    }
    if neighbor.direction == current.direction.opposite() {
        return false;
    }
    true
}

fn neighbor_filter_part_2(current: StepCoord, neighbor: StepCoord) -> bool {
    if current.steps < 3 && current.direction != neighbor.direction {
        return false;
    }
    if neighbor.steps > 9 {
        return false;
    }
    if neighbor.direction == current.direction.opposite() {
        return false;
    }
    true
}

fn lowest_heat<F: Fn(StepCoord, StepCoord) -> bool>(
    grid: &Grid,
    neighbor_filter: F,
) -> Option<usize> {
    let mut dist = HashMap::new();
    let mut queue = BinaryHeap::new();
    let step_coord = StepCoord::new((0, 0).into(), 0, Direction::Right);
    dist.insert(step_coord, 0);
    queue.push(StepCoordState {
        step_coord,
        state: 0,
    });
    let step_coord = StepCoord::new((0, 0).into(), 0, Direction::Down);
    dist.insert(step_coord, 0);
    queue.push(StepCoordState {
        step_coord,
        state: 0,
    });

    let end = Coord::from((grid.width() - 1, grid.height() - 1));
    while let Some(u) = queue.pop() {
        if u.step_coord.coord == end {
            return Some(u.state);
        }

        let neighbors = grid
            .cardinal_neighbors_coords(u.step_coord.coord)
            .into_iter()
            .map(|c| u.step_coord.path_to(c))
            .filter(|n| neighbor_filter(u.step_coord, *n));

        for neighbor in neighbors {
            let alt = u.state + grid[neighbor.coord];

            if alt < *dist.get(&neighbor).unwrap_or(&usize::MAX) {
                dist.insert(neighbor, alt);
                queue.push(StepCoordState {
                    step_coord: neighbor,
                    state: alt,
                });
            }
        }
    }

    None
}

fn part_1(grid: &Grid) -> usize {
    lowest_heat(grid, neighbor_filter_part_1).unwrap()
}

fn part_2(grid: &Grid) -> usize {
    lowest_heat(grid, neighbor_filter_part_2).unwrap()
}

fn main() {
    let input = aoc::read_input_lines("17");

    let grid = parse_input(input);

    aoc::print_part_1(part_1(&grid));
    aoc::print_part_2(part_2(&grid));
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_test_input() -> Grid {
        let input = r"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"
            .lines()
            .map(String::from)
            .collect::<Vec<_>>();

        parse_input(input)
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_test_input()), 102);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&get_test_input()), 94);
    }

    #[test]
    fn from_direction() {
        assert_eq!(
            Direction::from((Coord { x: 0, y: 1 }, Coord { x: 0, y: 0 })),
            Direction::Up
        );
    }
}
