use aoc_2025::{Error, print_output, read_input};

#[derive(Debug, PartialEq, PartialOrd)]
enum Diagram {
    Emtpy,
    Roll,
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
    diagram: Diagram,
}

fn find_removable(grid: &Vec<Vec<Position>>) -> Vec<&Position> {
    let mut removable = Vec::new();
    for position in grid.iter().flatten().filter(|p| p.diagram == Diagram::Roll) {
        let mut neighbors = Vec::new();
        for offset_y in -1..=1 {
            for offset_x in -1..=1 {
                if offset_x == 0 && offset_y == 0 {
                    continue;
                }

                let y = position.y + offset_y;
                let x = position.x + offset_x;
                if x < 0 || y < 0 {
                    continue;
                }

                if let Some(neighbor) = grid.get(y as usize).and_then(|row| row.get(x as usize)) {
                    neighbors.push(neighbor);
                }
            }
        }

        if neighbors
            .iter()
            .filter(|p| p.diagram == Diagram::Roll)
            .count()
            < 4
        {
            removable.push(position);
        }
    }

    removable
}

fn part_2(grid: &mut Vec<Vec<Position>>) -> usize {
    let mut removable = find_removable(&grid);
    let mut removed = removable.len();

    while removable.len() > 0 {
        let to_remove = removable
            .clone()
            .into_iter()
            .map(|p| (p.x as usize, p.y as usize))
            .collect::<Vec<_>>();

        for (x, y) in to_remove {
            grid[y][x].diagram = Diagram::Emtpy;
        }

        removable = find_removable(&grid);
        removed += removable.len()
    }

    removed
}

fn main() -> Result<(), Error> {
    let input = read_input("04")?;
    let mut grid = Vec::new();

    for (y, s) in input.into_iter().enumerate() {
        let mut row = Vec::new();
        for (x, c) in s.chars().enumerate() {
            let diagram = if c == '@' {
                Diagram::Roll
            } else {
                Diagram::Emtpy
            };

            row.push(Position {
                x: x as i32,
                y: y as i32,
                diagram,
            });
        }
        grid.push(row);
    }

    let part_1 = find_removable(&grid).len();

    print_output!(part_1, part_2(&mut grid));
    Ok(())
}
