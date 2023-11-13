use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::io::BufRead;
use chrono::prelude::*;
use std::collections::HashMap;

fn main() {
    let reader = BufReader::new(
        File::open(Path::new("src/bin/day-04/input.txt")).unwrap());

    let mut sorted = reader.lines().map(|results| results.unwrap()).collect::<Vec<String>>();
    sorted.sort();

    sorted.iter();
    let mut map = HashMap::new();

    let sorted: Vec<Data> = sorted.iter()
        .map(|string| {
            if string.contains("Guard") {
                let mut split: Vec<String> = string[18..string.len()].split(" ").map(|temp| temp.to_string()).collect();
                split[2].remove(0);
                Data::Id(split[2].parse::<u32>().unwrap())
            } else {
                Data::DateTime(read_date_time(string[1..18].as_ref()))
            }
        }).collect();
    let mut i = 0;

    while i < sorted.len() {
        if let Data::Id(id) = sorted[i] {
            i += 1;
            'durations: loop {
                if let (Data::DateTime(ref started), Data::DateTime(ref ended)) = (&sorted[i], &sorted[i + 1]) {
                    let duration = ended.signed_duration_since(*started);
                    if let Some(map_duration) = map.get(&id) {
                        map.insert(id, *map_duration + duration);
                    } else {
                        map.insert(id, duration);
                    }
                } else {
                    break 'durations
                }
                i += 2;
                if i >= sorted.len() {
                    break 'durations;
                }
                match sorted[i] {
                    Data::DateTime(_) => continue,
                    Data::Id(_) => break 'durations
                }
            }
        }
    }
    println!("{:?}", map);
    
    let mut i = 0;

    let mut minutes_sleep = HashMap::new();

    map.keys().for_each(|&key| {minutes_sleep.insert(key, vec![0; 60]);});    

    while i < sorted.len() {
        if let Data::Id(id) = sorted[i] {
            i += 1;
            'new: loop {
                if let (Data::DateTime(ref started), Data::DateTime(ref ended)) = (&sorted[i], &sorted[i + 1]) {
                    for j in started.minute()..ended.minute() {
                        minutes_sleep.get_mut(&id).unwrap()[j as usize] += 1;
                    };
                } else {
                    break 'new;
                };
                i += 2;
                if i >= sorted.len() {
                    break;
                };
                match sorted[i] {
                    Data::DateTime(_) => continue,
                    Data::Id(_) => break 'new
                };
            };
        } else {
            i += 1;
        };
    };

    let answer = minutes_sleep.iter()
        .map(|(id, vec)| (id, vec.iter().enumerate().max_by(|(_, left), (_, right)| left.cmp(right)).unwrap()))
        .max_by(|(_, (_, times_l)), (_, (_, times_r))| times_l.cmp(times_r)).unwrap();

    println!("{:?}", map.iter().max_by(|(_, &duration_l), (_, &duration_r)| duration_l.cmp(&duration_r)));
    println!("{:?}", answer);
    

//    sorted.iter().for_each(|data| println!("{:?}", data))

//    while let Some(string) = iter.next() {
//        let mut split: Vec<String> = string[18..string.len()].split(" ").map(|temp| temp.to_string()).collect();
//        split[2].remove(0);
//        let id = split[2].parse::<u32>().unwrap();
//        if map.get(&id) == None {
//            map.insert(id, Duration::new(0, 0));
//        }
//        println!("{}", split[2])
//    };

//    sorted.iter().map(|string| read_date_time(string[1..18].as_ref())).for_each(|string| println!("{}", string));
}

fn read_date_time(input: &str) -> DateTime<Utc> {
    let string = input.to_string();
    let date_time = Utc.ymd(string[0..4].parse().unwrap(),
            string[5..7].parse().unwrap(),
            string[8..10].parse().unwrap())
        .and_hms(string[11..13].parse().unwrap(),
                 string[14..16].parse().unwrap(),
                 0);
    if date_time.hour() == 23 { }
    
    date_time
}

#[derive(Debug, PartialOrd, PartialEq)]
enum Data {
    Id(u32),
    DateTime(DateTime<Utc>),
}

#[cfg(test)]
mod tests {
    use crate::*;
    use chrono::prelude::*;

    #[test]
    fn test_read_date_time() {
        assert_eq!(read_date_time("1518-06-03 00:32"),
                   Utc.ymd(1518, 6, 3).and_hms(0, 32, 0))
    }
}