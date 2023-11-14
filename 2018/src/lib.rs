// use std::fs::File;
// use std::io::{BufRead, BufReader};
// use std::path::PathBuf;

// pub fn lines_for_day(day: &'static str) -> Vec<String> {
//     BufReader::new(
//         File::open(PathBuf::from(format!("./src/bin/{}/input.txt", day)))
//             .expect("Could not open file"),
//     )
//     .lines()
//     .collect::<Result<Vec<_>, _>>()
//     .unwrap()
// }

// pub fn lines_for_day_test(day: &'static str) -> Vec<String> {
//     BufReader::new(
//         File::open(PathBuf::from(format!("./src/bin/{}/test.txt", day)))
//             .expect("Could not open file"),
//     )
//     .lines()
//     .collect::<Result<Vec<_>, _>>()
//     .unwrap()
// }
