use aoc_2025::{Error, Position, print_output, read_input, read_input_example};

fn area(l: Position, r: Position) -> usize {
    (l.x.max(r.x) - l.x.min(r.x) + 1) * (l.y.max(r.y) - l.y.min(r.y) + 1)
}

fn main() -> Result<(), Error> {
    let input = read_input("09")?
        .into_iter()
        .map(|s| {
            s.split_once(",").map(|(x, y)| {
                x.parse::<usize>()
                    .and_then(|x| y.parse::<usize>().map(|y| (x, y)))
            })
        })
        .collect::<Option<Result<Vec<_>, _>>>()
        .ok_or("no splits")??
        .into_iter()
        .map(|(x, y)| Position::new(x, y))
        .collect::<Vec<_>>();

    let mut pairs = Vec::new();
    for (i, &l) in input[0..input.len() - 1].iter().enumerate() {
        for &r in &input[i + 1..] {
            pairs.push((l, r));
        }
    }

    let part_1 = pairs
        .iter()
        // .inspect(|&(l, r)| println!("{:?} - {:?}", l, r))
        .map(|&(l, r)| area(l, r))
        // .inspect(|a| println!("{}", a))
        .max()
        .unwrap_or_default();

    print_output!(part_1);
    Ok(())
}
