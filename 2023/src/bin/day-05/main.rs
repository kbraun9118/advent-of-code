use rayon::prelude::*;

struct ConversionMap {
    conversions: Vec<Conversion>,
}

impl ConversionMap {
    fn convert(&self, source: usize) -> usize {
        self.conversions
            .iter()
            .find_map(|c| c.convert(source))
            .unwrap_or(source)
    }
}

struct Conversion {
    destination_start: usize,
    source_start: usize,
    range_length: usize,
}

impl Conversion {
    fn new(destination_start: usize, source_start: usize, range_length: usize) -> Self {
        Self {
            destination_start,
            source_start,
            range_length,
        }
    }

    fn convert(&self, source: usize) -> Option<usize> {
        if source >= self.source_start && source < self.source_start + self.range_length {
            Some(self.destination_start + (source - self.source_start))
        } else {
            None
        }
    }
}

impl From<String> for Conversion {
    fn from(value: String) -> Self {
        let split = value.split(" ").collect::<Vec<_>>();

        Self::new(
            split[0].parse().unwrap(),
            split[1].parse().unwrap(),
            split[2].parse().unwrap(),
        )
    }
}

struct Almanac {
    seeds: Vec<usize>,
    conversion_maps: Vec<ConversionMap>,
}

impl From<Vec<String>> for Almanac {
    fn from(value: Vec<String>) -> Self {
        let chunks = value.split(|s| s == "").collect::<Vec<_>>();
        let seeds = chunks[0][0]
            .split_once(": ")
            .unwrap()
            .1
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let conversion_maps = chunks[1..]
            .iter()
            .map(|s| ConversionMap {
                conversions: s[1..].iter().map(|s| Conversion::from(s.clone())).collect(),
            })
            .collect();

        Self {
            seeds,
            conversion_maps,
        }
    }
}

fn part_1(almanac: &Almanac) -> usize {
    almanac
        .seeds
        .iter()
        .map(|seed| {
            almanac
                .conversion_maps
                .iter()
                .fold(*seed, |acc, next| next.convert(acc))
        })
        .min()
        .unwrap()
}

fn part_2(almanac: &Almanac) -> usize {
    almanac
        .seeds
        .chunks(2)
        .map(|range| range[0]..range[0] + range[1])
        .par_bridge()
        .flat_map(|range| {
            range.par_bridge().map(|seed| {
                almanac
                    .conversion_maps
                    .iter()
                    .fold(seed, |acc, next| next.convert(acc))
            })
        })
        .min()
        .unwrap()
}

fn main() {
    let almanac: Almanac = aoc::read_input_lines("05").into();

    aoc::print_part_1(part_1(&almanac));
    aoc::print_part_2(part_2(&almanac));
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> Almanac {
        Almanac::from(
            r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#
                .lines()
                .map(String::from)
                .collect::<Vec<_>>(),
        )
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&test_input()), 35);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&test_input()), 46);
    }
}
