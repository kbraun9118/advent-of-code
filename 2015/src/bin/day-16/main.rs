use aoc_2015::lines_for_day;
use regex::Regex;

fn main() {
    let ticker = Sue::from(
        r"children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1"
            .to_string(),
    );
    let sues = lines_for_day("16")
        .into_iter()
        .map(Sue::from)
        .collect::<Vec<_>>();

    let part1 = sues.iter().filter(|s| **s == ticker).nth(0);

    println!("Part 1: {}", part1.unwrap().number.unwrap());

    let part2 = sues.iter().filter(|s| s.part_2(&ticker)).nth(0);

    println!("Part 1: {}", part2.unwrap().number.unwrap());

}

#[derive(Debug)]
struct Sue {
    number: Option<u32>,
    children: Option<u32>,
    cats: Option<u32>,
    samoyeds: Option<u32>,
    pomeranians: Option<u32>,
    akitas: Option<u32>,
    vizslas: Option<u32>,
    goldfish: Option<u32>,
    trees: Option<u32>,
    cars: Option<u32>,
    perfumes: Option<u32>,
}

impl PartialEq for Sue {
    fn eq(&self, other: &Self) -> bool {
        (self.children.is_none() || other.children.is_none() || self.children == other.children)
            && (self.cats.is_none() || other.cats.is_none() || self.cats == other.cats)
            && (self.samoyeds.is_none()
                || other.samoyeds.is_none()
                || self.samoyeds == other.samoyeds)
            && (self.pomeranians.is_none()
                || other.pomeranians.is_none()
                || self.pomeranians == other.pomeranians)
            && (self.akitas.is_none() || other.akitas.is_none() || self.akitas == other.akitas)
            && (self.vizslas.is_none() || other.vizslas.is_none() || self.vizslas == other.vizslas)
            && (self.goldfish.is_none()
                || other.goldfish.is_none()
                || self.goldfish == other.goldfish)
            && (self.trees.is_none() || other.trees.is_none() || self.trees == other.trees)
            && (self.cars.is_none() || other.cars.is_none() || self.cars == other.cars)
            && (self.perfumes.is_none()
                || other.perfumes.is_none()
                || self.perfumes == other.perfumes)
    }
}

impl Sue {
    fn part_2(&self, other: &Self) -> bool {
        (self.children.is_none() || other.children.is_none() || self.children == other.children)
            && (self.cats.is_none() || other.cats.is_none() || self.cats > other.cats)
            && (self.samoyeds.is_none()
                || other.samoyeds.is_none()
                || self.samoyeds == other.samoyeds)
            && (self.pomeranians.is_none()
                || other.pomeranians.is_none()
                || self.pomeranians < other.pomeranians)
            && (self.akitas.is_none() || other.akitas.is_none() || self.akitas == other.akitas)
            && (self.vizslas.is_none() || other.vizslas.is_none() || self.vizslas == other.vizslas)
            && (self.goldfish.is_none()
                || other.goldfish.is_none()
                || self.goldfish < other.goldfish)
            && (self.trees.is_none() || other.trees.is_none() || self.trees > other.trees)
            && (self.cars.is_none() || other.cars.is_none() || self.cars == other.cars)
            && (self.perfumes.is_none()
                || other.perfumes.is_none()
                || self.perfumes == other.perfumes)
    }
}

impl From<String> for Sue {
    fn from(value: String) -> Self {
        Self {
            number: Regex::new(r"Sue (\d+)")
                .unwrap()
                .captures(&value)
                .map(|i| i.get(1).unwrap().as_str().parse().unwrap()),
            children: Regex::new(r"children: (\d+)")
                .unwrap()
                .captures(&value)
                .map(|i| i.get(1).unwrap().as_str().parse().unwrap()),
            cats: Regex::new(r"cats: (\d+)")
                .unwrap()
                .captures(&value)
                .map(|i| i.get(1).unwrap().as_str().parse().unwrap()),
            samoyeds: Regex::new(r"samoyeds: (\d+)")
                .unwrap()
                .captures(&value)
                .map(|i| i.get(1).unwrap().as_str().parse().unwrap()),
            pomeranians: Regex::new(r"pomeranians: (\d+)")
                .unwrap()
                .captures(&value)
                .map(|i| i.get(1).unwrap().as_str().parse().unwrap()),
            akitas: Regex::new(r"akitas: (\d+)")
                .unwrap()
                .captures(&value)
                .map(|i| i.get(1).unwrap().as_str().parse().unwrap()),
            vizslas: Regex::new(r"vizslas: (\d+)")
                .unwrap()
                .captures(&value)
                .map(|i| i.get(1).unwrap().as_str().parse().unwrap()),
            goldfish: Regex::new(r"goldfish: (\d+)")
                .unwrap()
                .captures(&value)
                .map(|i| i.get(1).unwrap().as_str().parse().unwrap()),
            trees: Regex::new(r"trees: (\d+)")
                .unwrap()
                .captures(&value)
                .map(|i| i.get(1).unwrap().as_str().parse().unwrap()),
            cars: Regex::new(r"cars: (\d+)")
                .unwrap()
                .captures(&value)
                .map(|i| i.get(1).unwrap().as_str().parse().unwrap()),
            perfumes: Regex::new(r"perfumes: (\d+)")
                .unwrap()
                .captures(&value)
                .map(|i| i.get(1).unwrap().as_str().parse().unwrap()),
        }
    }
}
