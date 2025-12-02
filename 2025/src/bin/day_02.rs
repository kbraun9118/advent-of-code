use aoc_2025::{Error, read_example_input};

fn main() -> Result<(), Error> {
    let input = read_example_input("02")?;
    let ranges = input.get(0)?.split(",");

    Ok(())
}
