use aoc_2025::{Error, print_output, read_input};

fn invalid_id(id: usize) -> bool {
    let num_str = id.to_string();
    if num_str.len() % 2 == 1 {
        return false;
    }

    let (left, right) = num_str.split_at(num_str.len() / 2);

    left == right
}

fn invalid_id_2(id: usize) -> bool {
    let num_str = id.to_string();
    let num_bytes = num_str.as_bytes();

    for chunk_size in 1..num_bytes.len() {
        let chunks = num_bytes.chunks(chunk_size).collect::<Vec<_>>();
        if chunks.windows(2).all(|w| w[0] == w[1]) {
            return true;
        }
    }
    false
}

fn main() -> Result<(), Error> {
    let input = read_input("02")?;
    let ranges = input
        .get(0)
        .ok_or("expected one row")?
        .split(",")
        .filter_map(|ranges| ranges.split_once("-"))
        .map(|(left, right)| (left.parse::<usize>(), right.parse::<usize>()))
        .map(|(left, right)| left.and_then(|l| right.map(|r| (l, r))))
        .map(|range| range.map(|(l, r)| l..=r))
        .collect::<Result<Vec<_>, _>>()?;

    let part_1 = ranges
        .iter()
        .map(|r| r.clone().filter(|id| invalid_id(*id)).sum::<usize>())
        .sum::<usize>();

    let part_2 = ranges
        .iter()
        .map(|r| r.clone().filter(|id| invalid_id_2(*id)).sum::<usize>())
        .sum::<usize>();

    print_output!(part_1, part_2);
    Ok(())
}
