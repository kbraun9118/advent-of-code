use aoc_2025::{Error, print_output, read_input};

fn part_solve(input: Vec<String>) -> Result<(usize, usize), Error> {
    let mut current = 50;
    let (mut part_1, mut part_2) = (0, 0);

    for ref mut i in input {
        let sign = match i {
            _ if i.starts_with("L") => -1,
            _ if i.starts_with("R") => 1,
            value => return Err(format!("Invalid variant: {}", value).into()),
        };
        let offset: usize = i.split_off(1).parse()?;

        for _ in 0..offset {
            current += sign;
            current = ((current % 100) + 100) % 100;
            if current == 0 {
                part_2 += 1;
            }
        }
        if current == 0 {
            part_1 += 1;
        }
    }
    Ok((part_1, part_2))
}

fn main() -> Result<(), Error> {
    let input = read_input("01")?;

    let (part_1, part_2) = part_solve(input)?;

    print_output!(part_1, part_2);

    Ok(())
}
