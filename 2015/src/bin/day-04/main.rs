use aoc_2015::lines_for_day;

fn main() {
    let input = lines_for_day("day-04").pop().unwrap();
    let mut five_zeros: i32 = 0;
    let mut six_zeros: i32 = 0;
    for i in 1.. {
        let digest = md5::compute(format!("{}{}", input, i));
        let as_string = format!("{:x}", digest);
        if as_string.starts_with("000000") {
            six_zeros = i;
            break;
        }
        if as_string.starts_with("00000") && five_zeros == 0 {
            five_zeros = i;
        }
    }

    println!("Part One: {}", five_zeros);
    println!("Part Two: {}", six_zeros);
}
