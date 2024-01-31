use aoc_2015::lines_for_day ;

const DIST: u32 = 2503;

fn main() {
    let reindeer = lines_for_day("14")
        .into_iter()
        .map(Reindeer::from)
        .collect::<Vec<_>>();

    println!(
        "Part 1: {:?}",
        reindeer
            .iter()
            .map(|r| r.distance_after(DIST))
            .max()
            .unwrap()
    );
    println!("Part 2: {:?}", max_score_game(reindeer, DIST));
}

fn max_score_game(reindeer: Vec<Reindeer>, distance: u32) -> u32 {
    let mut scores = vec![0; reindeer.len()];

    for i in 1..=distance {
        let current = reindeer
            .iter()
            .map(|r| r.distance_after(i))
            .collect::<Vec<_>>();

        let current_max = current.iter().max().unwrap();

        current
            .iter()
            .enumerate()
            .filter(|(_, r)| *r == current_max)
            .for_each(|(j, _)| scores[j] += 1)
    }

    scores.into_iter().max().unwrap()
}

#[derive(Debug)]
struct Reindeer {
    speed: u32,
    duration: u32,
    rest: u32,
}

impl Reindeer {
    fn distance_after(&self, time: u32) -> u32 {
        let full_time = time / (self.duration + self.rest);
        let remaining = time - (full_time * (self.duration + self.rest));
        let additional = if remaining > self.duration {
            self.duration
        } else {
            remaining
        };
        full_time * self.speed * self.duration + additional * self.speed
    }
}

impl From<String> for Reindeer {
    fn from(value: String) -> Self {
        let split = value.split(' ').collect::<Vec<_>>();
        Reindeer {
            speed: split[3].parse().unwrap(),
            duration: split[6].parse().unwrap(),
            rest: split[13].parse().unwrap(),
        }
    }
}
