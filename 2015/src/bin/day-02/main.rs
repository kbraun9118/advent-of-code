use aoc_2015::lines_for_day;
use std::str::FromStr;

fn main() {
    let lines = lines_for_day("day-02");

    let lines = lines
        .iter()
        .map(|l| {
            let split = l.split('x').collect::<Vec<_>>();
            (
                i32::from_str(split[0]).unwrap(),
                i32::from_str(split[1]).unwrap(),
                i32::from_str(split[2]).unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let part_one: i32 = lines
        .iter()
        .map(|(x, y, z)| 2 * x * y + 2 * y * z + 2 * x * z + (x * y).min(y * z).min(z * x))
        .sum();

    let part_two: i32 = lines
        .iter()
        .map(|(x, y, z)| 2 * x + 2 * y + 2 * z - (2 * x).max(2 * y).max(2 * z) + x * y * z)
        .sum();

    println!("Part One: {}", part_one);
    println!("Part Two: {}", part_two);
}
