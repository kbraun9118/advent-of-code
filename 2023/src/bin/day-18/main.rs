use std::isize;

type Coord = aoc::Coord<isize>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        use Direction::*;
        match value {
            'R' => Right,
            'L' => Left,
            'U' => Up,
            'D' => Down,
            _ => panic!("invalid char"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Dig {
    direction: Direction,
    steps: isize,
    rest: String,
}

impl Dig {
    fn hex_dig(&self) -> Self {
        let steps = isize::from_str_radix(&self.rest[2..7], 16).unwrap();
        let direction = match &self.rest[7..8] {
            "0" => Direction::Right,
            "1" => Direction::Down,
            "2" => Direction::Left,
            "3" => Direction::Up,
            _ => panic!("invalid value, {}", &self.rest[7..8]),
        };

        Self {
            steps,
            direction,
            rest: "".to_string(),
        }
    }
}

impl From<String> for Dig {
    fn from(value: String) -> Self {
        let split = value.split(" ").collect::<Vec<_>>();

        Self {
            direction: split[0].chars().next().unwrap().into(),
            steps: split[1].parse().unwrap(),
            rest: split[2].to_string(),
        }
    }
}

fn dig_area(digs: &Vec<Dig>) -> isize {
    let mut current = (0, 0).into();
    let mut coords: Vec<Coord> = vec![current];
    for dig in digs {
        match dig.direction {
            Direction::Right => current.x += dig.steps,
            Direction::Left => current.x -= dig.steps,
            Direction::Up => current.y -= dig.steps,
            Direction::Down => current.y += dig.steps,
        }

        coords.push(current);
    }
    coords.push((0, 0).into());

    coords
        .windows(2)
        .map(|w| w[0].x * w[1].y - w[0].y * w[1].x)
        .sum::<isize>()
        .abs()
        / 2
        + digs.iter().map(|d| d.steps).sum::<isize>() / 2
        + 1
}

fn part_1(digs: &Vec<Dig>) -> isize {
    dig_area(digs)
}

fn part_2(digs: &Vec<Dig>) -> isize {
    dig_area(&digs.iter().map(Dig::hex_dig).collect())
}

fn main() {
    let input = aoc::read_input_lines("18");

    let digs = input.into_iter().map(Dig::from).collect::<Vec<_>>();

    aoc::print_part_1(part_1(&digs));
    aoc::print_part_2(part_2(&digs));
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_test_input() -> Vec<Dig> {
        r"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"
            .lines()
            .map(String::from)
            .map(Dig::from)
            .collect()
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_test_input()), 62);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&get_test_input()), 952408144115);
    }
}
