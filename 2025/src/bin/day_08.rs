use aoc_2025::{Error, print_output, read_input_example};

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

fn min_distance_pair(coords: &Vec<Position3>) -> Option<(Position3, Position3)> {
    let mut min_distance = isize::MAX;
    let mut min_positions = None;
    for (i, left) in coords[..coords.len() - 1].iter().enumerate() {
        for right in &coords[i + 1..] {
            let current_distance = left.distance_from(*right);
            if current_distance < min_distance {
                min_distance = current_distance;
                min_positions = Some((*left, *right))
            }
        }
    }

    min_positions
}

fn main() -> Result<(), Error> {
    let input = read_input_example("08")?
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

    print_output!(min_distance_pair(&input));
    Ok(())
}
