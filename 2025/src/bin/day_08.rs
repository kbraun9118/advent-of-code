use aoc_2025::{Error, print_output, read_input, read_input_example};

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

    print_output!(&distance_pairs[0..5]);
    Ok(())
}
