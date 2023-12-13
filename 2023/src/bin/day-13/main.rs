fn main() {
    let input = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#."#
        .lines()
        .map(String::from)
        .collect::<Vec<_>>();
    let left = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    for i in 1..left.len() {
        println!(
            "{:?}",
            left[..i]
                .iter()
                .rev()
                .zip(left[i..].iter())
                .collect::<Vec<_>>()
        );
    }
}
