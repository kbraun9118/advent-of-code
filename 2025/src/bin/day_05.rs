use std::ops::Range;

use aoc_2025::{Error, print_output, read_input};

fn part_1(ranges: &Vec<Range<usize>>, ids: &Vec<usize>) -> usize {
    ids.iter()
        .filter(|id| ranges.iter().any(|r| r.contains(&id)))
        .count()
}

fn part_2(ranges: &Vec<Range<usize>>) -> usize {
    ranges.iter().map(|r| r.len()).sum()
}

fn main() -> Result<(), Error> {
    let input = read_input("05")?;

    let mut ranges = input
        .clone()
        .into_iter()
        .take_while(|s| s != "")
        .map(|range| {
            if let Some((lower, upper)) = range.split_once("-") {
                let lower = lower.parse::<usize>().unwrap_or(0);
                let upper = upper.parse::<usize>().unwrap_or(0);
                lower..upper + 1
            } else {
                0..0
            }
        })
        .collect::<Vec<_>>();
    ranges.sort_by_key(|r| r.start);

    let mut combined_ranges = Vec::new();
    let mut ranges = ranges.into_iter();
    let mut current = ranges.next().unwrap_or_default();

    for r in ranges {
        if r.start < current.end {
            current.end = current.end.max(r.end)
        } else {
            combined_ranges.push(current);
            current = r;
        }
    }
    combined_ranges.push(current);
    let ranges = combined_ranges;

    let ids = input
        .into_iter()
        .skip_while(|s| s != "")
        .skip(1)
        .map(|id| id.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()?;

    print_output!(part_1(&ranges, &ids), part_2(&ranges));
    Ok(())
}
