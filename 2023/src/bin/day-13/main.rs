type Grid = aoc::Grid<Terrain>;

#[derive(Debug, Clone, Copy)]
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
        .map(|r| r.chars().map(Terrain::from).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let grid = Grid::new(input.len(), input[0].len(), |aoc::Coord {x, y}| input[y][x]);

    println!("{grid:#?}");
}
