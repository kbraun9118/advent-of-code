use aoc_2025::{Error, print_output, read_input, read_input_example};

fn part_1(input: &Vec<String>) -> Result<usize, Error> {
    let input = input
        .iter()
        .map(|l| l.split_whitespace().map(str::to_string).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (ops, rows) = if let Some((ops, nums)) = input.split_last() {
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
    let mut sum = 0;

    let row_length = rows.get(0).map(|r| r.len()).unwrap_or_default();
    for i in 0..row_length {
        let mut column_value = 0;
        let op = ops.get(i).ok_or("ops of incorrect length")?;
        if op == "*" {
            column_value = 1;
        }
        for row in &rows {
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

fn transpose<T: Default>(mut vec: Vec<Vec<T>>) -> Result<Vec<Vec<T>>, Error> {
    let mut out = Vec::new();
    let row_len = vec.len();
    let col_len = vec.get(0).map(|r| r.len()).ok_or("No inner vector")?;

    for x in 0..col_len {
        let mut inner = Vec::new();
        for y in 0..row_len {
            let value = vec
                .get_mut(y)
                .ok_or("no row")?
                .get_mut(x)
                .ok_or("no value")?;

            inner.push(std::mem::take(value));
        }
        out.push(inner);
    }

    Ok(out)
}

fn part_2(input: &Vec<String>) -> Result<usize, Error> {
    let (ops, nums) = input.split_last().ok_or("could not split")?;

    let input = nums
        .iter()
        .map(|l| l.chars().rev().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let input = transpose(input)?;

    let nums = input
        .into_iter()
        .map(|num| {
            num.into_iter()
                .filter(char::is_ascii_digit)
                .collect::<String>()
        })
        .collect::<Vec<_>>();

    let nums = nums
        .split(String::is_empty)
        .map(|l| {
            l.into_iter()
                .map(|s| s.parse::<usize>())
                .collect::<Result<Vec<_>, _>>()
        })
        // .zip(ops.chars().into_iter())
        .collect::<Result<Vec<_>, _>>()?;

    let ops = ops.split_whitespace().collect::<Vec<_>>();

    let mut sum = 0;
    let row_length = nums.get(0).map(|r| r.len()).unwrap_or_default();
    for i in 0..row_length {
        let mut column_value = 0;
        let op = ops.get(i).ok_or("ops of incorrect length")?;
        if *op == "*" {
            column_value = 1;
        }
        for row in &nums {
            if *op == "+" {
                println!("adding: {}", row[i]);
                column_value += row[i];
            }
            if *op == "*" {
                println!("multiplying: {}", row[i]);
                column_value *= row[i];
            }
        }
        sum += column_value;
    }

    Ok(sum)
}

fn main() -> Result<(), Error> {
    let input = read_input_example("06")?;

    print_output!(part_1(&input)?, part_2(&input)?);

    Ok(())
}
