type Grid = aoc::Grid<Terrain>;

#[derive(Debug)]
enum Terrain {
    Ash,
    Rock,
}

impl From<char> for Terrain {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Rock,
            '.' => Self::Ash,
            _ => panic!("invalid character"),
        }
    }
}

fn main() {
    let input = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#."#
        .lines()
        .map(String::from)
        .collect::<Vec<_>>();

    let grid = Grid::from(
        input
            .into_iter()
            .map(|r| r.chars().map(Terrain::from).collect())
            .collect::<Vec<_>>(),
    );

    println!("{grid:#?}");
}
