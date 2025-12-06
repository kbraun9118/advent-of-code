use aoc_2025::{Error, print_output, read_input, read_input_example};

fn part_1(rows: &Vec<Vec<usize>>, ops: &Vec<String>) -> Result<usize, Error> {
    let mut sum = 0;
    let row_length = rows.get(0).map(|r| r.len()).unwrap_or_default();
    for i in 0..row_length {
        let mut column_value = 0;
        let op = ops.get(i).ok_or("ops of incorrect length")?;
        if op == "*" {
            column_value = 1;
        }
        for row in rows {
            if op == "+" {
                column_value += row[i];
            }
            if op == "*" {
                column_value *= row[i];
            }
        }
        sum += column_value;
    }

    Ok(sum)
}

fn main() -> Result<(), Error> {
    let input = read_input("06")?
        .into_iter()
        .map(|l| l.split_whitespace().map(str::to_string).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (ops, nums) = if let Some((ops, nums)) = input.split_last() {
        let nums = nums
            .iter()
            .map(|l| {
                l.iter()
                    .map(|s| s.parse::<usize>())
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        (ops.clone(), nums)
    } else {
        return Err("invalid format".into());
    };

    print_output!(part_1(&nums, &ops)?);

    Ok(())
}
