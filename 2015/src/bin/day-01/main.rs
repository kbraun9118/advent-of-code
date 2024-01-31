use aoc_2015::lines_for_day;
use std::isize;

fn main() {
    let lines = lines_for_day("01");
    let part_one = isize::try_from(lines[0].chars().filter(|c| *c == '(').count()).unwrap()
        - isize::try_from(lines[0].chars().filter(|c| *c == ')').count()).unwrap();

    println!("Part One: {}", part_one);

    let mut floor = 0;

    let dir = lines[0].chars().enumerate();
    for (i, c) in dir {
        if c == '(' {
            floor += 1;
        } else {
            floor -= 1;
        }
        if floor < 0 {
            println!("Part Two: {}", i + 1);
            break;
        }
    }
}
