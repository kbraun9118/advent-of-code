use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::io::BufRead;
use std::collections::HashMap;

fn main() {
    let reader = BufReader::new(
        File::open(Path::new("../input/2018/07/input.txt")).unwrap());

    let mut map: HashMap<Step, Vec<Step>> = HashMap::new();
    reader.lines()
        .map(|line| line.unwrap().to_string())
        .collect::<Vec<String>>().iter()
        .map(|line| line.split(' ').collect::<Vec<&str>>())
        .map(|strs: Vec<&str>| (Step::from(strs[1]), Step::from(strs[7])))
        .for_each(|(dependency, dependent)| {
            match map.get_mut(&dependent) {
                Some(vec) => vec.push(dependency),
                None => { map.insert(dependent, vec![dependency]); }
            }
            if let None = map.get(&dependency) {
                map.insert(dependency, vec![]);
            }
        });

    let mut step_order = vec![];
    let map2 = map.clone();


    while !map.is_empty() {
        let mut next: Vec<Step> = map.iter()
            .filter(|(_, dependents)| dependents.is_empty())
            .map(|(dependency, _)| dependency.clone())
            .collect();

        next.sort();
        map.remove(&next[0]);
        map.values_mut()
            .filter(|dependents| dependents.contains(&next[0]))
            .for_each(|dependents| { dependents.retain(|d| d != &next[0]); });

        step_order.push(next[0]);
    }


    step_order.iter().for_each(|step| print!("{}", step.id));
    part2(map2);
    
}

fn part2(mut steps: HashMap<Step, Vec<Step>>) {
    let mut time_taken = 0u32;
    let mut workers = Workers::new(5);
    let mut work_to_do: Vec<Step> = vec![];
    let mut work_completed: Vec<Step> = vec![];

    steps.iter()
        .filter(|(_, dependents)| dependents.is_empty())
        .for_each(|(&dependency, _)| work_to_do.push(dependency));
    
    for step in work_to_do.iter() {
        steps.remove(step);
    }
    
    work_to_do.sort();

    loop {
        
        if workers.all_idle() && work_to_do.is_empty() {
            break;
        };

        for status in workers.finished() {
            work_completed.push(status);
            steps.values_mut()
                .filter(|dependents| dependents.contains(&status))
                .for_each(|dependents| { dependents.retain(|d| d != &status); });
        };
        
        steps.iter()
            .filter(|(_, dependents)| dependents.is_empty())
            .for_each(|(&dependency, _)| work_to_do.push(dependency));

        for step in work_to_do.iter() {
            steps.remove(step);
        }
        
        work_to_do.sort();

        while workers.any_idle() && !work_to_do.is_empty() {
            if let Some(step) = work_to_do.pop() {
                workers.assign_work(step);
            };
        };
        
        workers.step();
        time_taken += 1;
    }
    
    println!("\nTime taken for part 2 = {}", time_taken - 1);
}

#[derive(Debug)]
struct Workers {
    status: Vec<Status>
}

impl Workers {
    fn new(count: usize) -> Self {
        Workers { status: vec![Status::Idle; count] }
    }

    fn assign_work(&mut self, step: Step) {
        let index = self.index_of_idle();
        self.status[index] = Status::Working { step, remaining: step.time_taken() }
    }

    fn index_of_idle(&self) -> usize {
        self.status.iter()
            .enumerate()
            .filter(|(_, &status)| status == Status::Idle)
            .map(|(index, _)| index)
            .nth(0)
            .unwrap()
    }

    fn any_idle(&self) -> bool {
        self.status.contains(&Status::Idle)
    }

    fn all_idle(&self) -> bool {
        self.status.iter()
            .filter(|&&status| status != Status::Idle)
            .collect::<Vec<&Status>>()
            .is_empty()
    }

    fn finished(&mut self) -> Vec<Step> {
        let mut finished = vec![];
        for worker in 0..self.status.len() {
            let mut is_done = false;
            if let Status::Working { step, ref remaining } = self.status[worker] {
                if *remaining == 0 {
                    finished.push(step);
                    is_done = true;
                }
            }
            if is_done {
                self.status[worker] = Status::Idle;
            }
        }
        finished
    }
    
    fn step(&mut self) {
        for worker in 0..self.status.len() {
            if let Status::Working {step: _, ref mut remaining } = self.status[worker] {
                *remaining -= 1;
            }
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Status {
    Idle,
    Working { step: Step, remaining: u32 },
}

#[derive(Debug, PartialOrd, PartialEq, Eq, Hash, Clone, Copy, Ord)]
struct Step {
    id: char
}

impl Step {
    fn time_taken(&self) -> u32 {
        (self.id as u8 - b'A' + 60 + 1) as u32
    }
}

impl From<&str> for Step {
    fn from(string: &str) -> Self {
        Step { id: string.as_bytes()[0] as char }
    }
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_step_time_taken() {
        assert_eq!(61, Step { id: 'A' }.time_taken());
        assert_eq!(62, Step { id: 'B' }.time_taken());
    }

    #[test]
    fn test_assign_work() {
        let mut workers = Workers::new(1);
        workers.assign_work(Step { id: 'A' });
        assert_eq!(workers.status[0], Status::Working { step: Step { id: 'A' }, remaining: 61 });
    }

    #[test]
    fn test_any_idle() {
        let mut workers = Workers::new(2);
        assert!(workers.any_idle());
        workers.assign_work(Step { id: 'A' });
        assert!(workers.any_idle());
        workers.assign_work(Step { id: 'A' });
        assert_eq!(workers.any_idle(), false)
    }

    #[test]
    fn test_all_idle() {
        let mut workers = Workers::new(5);
        assert!(workers.all_idle());
        workers.assign_work(Step { id: 'A' });
        assert_eq!(workers.all_idle(), false)
    }

    #[test]
    fn test_finished() {
        let mut workers = Workers::new(5);
        workers.assign_work(Step { id: 'A' });
        match workers.status[0] {
            Status::Idle => {}
            Status::Working { step, ref mut remaining } => *remaining -= 61
        }
        assert_eq!(workers.finished(), vec![Step { id: 'A' }])
    }
}