#![allow(unused)]

use std::collections::VecDeque;
use std::fmt::Display;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;

use crossbeam_utils::thread;

fn main() {
    let reader = BufReader::new(
        File::open(Path::new("../input/2018/15/input.txt")).unwrap());
    let tiles: Map = reader.lines()
        .map(|line| line.unwrap())
        .map(|string| string.chars()
            .map(|character| Tile::from(character))
            .collect::<Vec<Tile>>())
        .collect::<Vec<Vec<Tile>>>()
        .into();

    let mut game = Game::from(tiles);
//    println!("{:?}", game.map.shortest_distance((7, 1), (4, 4)));

//    for i in 1..=35 {
//        println!("Turn {}", i);
//        game.map.tick();
//        println!();
//    }

    println!("{:?}", game.resolve_2());

//    game.map.get_units().iter().for_each(|(_, unit)| println!("{:?}", unit));

//    game.print_map();
}

#[derive(Debug, Clone)]
struct Game {
    map: Map,
}

impl Game {
    fn print_map(&self) {
        println!("{}", self.map);
    }

    fn get_units_mut(&mut self) -> VecDeque<((usize, usize), &mut Unit)> {
        self.map.get_units_mut()
    }

    fn get_units(&self) -> VecDeque<((usize, usize), &Unit)> {
        self.map.get_units()
    }
    
    fn resolve(&mut self) -> usize {
        let mut i = 0;
        while self.map.contains_both() {
            println!("Turn {}", i);
            self.map.tick();
            i += 1;
            println!();
        }
        
        println!("Finished after {} rounds", i);
        self.get_units().iter()
            .map(|(_, unit)| unit.hp as usize)
            .sum::<usize>() * (i - 1)
    }
    
    fn resolve_2(&mut self) -> usize {
        let beginning_count = self.map.count_elves();
        let mut count = 0;

        loop {
            let mut game = self.clone();
            game.get_units_mut()
                .iter_mut()
                .filter(|(_, unit)| unit.unit_type == UnitType::Elf)
                .for_each(|(_, unit)| unit.attack_power += count);
            
            let returned = game.resolve();
            
            if beginning_count == game.map.count_elves() { 
                game.print_map();
                println!("Beginning elf count: {}, end elf count: {}", beginning_count, game.map.count_elves());
                println!("Attack power needed is {}", count + 3);
                return returned;
            } else { 
                count += 1;
            }
        }
    }
}

impl From<Map> for Game {
    fn from(map: Map) -> Self {
        Game { map }
    }
}

impl From<Vec<Vec<Tile>>> for Map {
    fn from(map: Vec<Vec<Tile>>) -> Self {
        Map(map)
    }
}

#[derive(Debug, Clone)]
struct Map(Vec<Vec<Tile>>);

impl Map {
    fn count_elves(&self) -> usize {
        self.get_units()
            .iter()
            .filter(|(_, unit)| unit.unit_type == UnitType::Elf)
            .count()
    }
    
    fn contains_both(&self) -> bool {
        let units = self.get_units();
        units.iter().any(|(_, unit)| unit.unit_type == UnitType::Goblin) &&
            units.iter().any(|(_, unit)| unit.unit_type == UnitType::Elf)
    }
    
    fn tick(&mut self) {
        let units: Vec<((usize, usize), UnitType, i16)> = self.get_units().iter()
            .map(|(position, unit)| (*position, unit.unit_type.clone(), unit.attack_power.clone()))
            .collect();

        units.into_iter()
            .for_each(|(position, unit_type, attack_power)| {
                if let Tile::Open(Some(_)) = &self[position.1][position.0] {
                    self.take_turn(position, unit_type, attack_power)
                }
            });
    }


    fn take_turn(&mut self, position: (usize, usize), unit_type: UnitType, attack_power: i16) {
        if let Some(unit) = self.get_adjacent_enemy(position, unit_type) {
            self.deal_damage(unit, attack_power);
        } else {
            if let Some(mut vec) = self.find_nearest(position, unit_type.get_opposite()) {
                if let Some(to_position) = vec.pop_back() {
                    self.move_unit(position, to_position);

                    if let Some((x, y)) = self.get_adjacent_enemy(to_position, unit_type) {
                        self.deal_damage((x, y), attack_power)
                    }
                }
            }
        }
    }

    fn deal_damage(&mut self, (x, y): (usize, usize), attack_power: i16) {
        let mut removed = false;
        if let Tile::Open(Some(unit)) = &mut self[y][x] {
            unit.hp -= attack_power;
            if unit.hp <= 0 {
                removed = true
            }
            println!("Dealt damage to unit {:?}, hp now {:?}", (x, y), unit.hp);
        }
        if removed {
            println!("Unit at {:?} has been killed", (x, y));
            self.remove_unit((x, y));
        }
    }

    fn get_adjacent_enemy(&self, (x, y): (usize, usize), unit_type: UnitType) -> Option<(usize, usize)> {
        let mut adjacent = vec![];
        if let Tile::Open(Some(unit)) = &self[y - 1][x] {
            if unit.unit_type.is_opposite(unit_type) {
                adjacent.push((x, y - 1, unit.hp));
            }
        }
        if let Tile::Open(Some(unit)) = &self[y][x - 1] {
            if unit.unit_type.is_opposite(unit_type) {
                adjacent.push((x - 1, y, unit.hp));
            }
        }
        if let Tile::Open(Some(unit)) = &self[y][x + 1] {
            if unit.unit_type.is_opposite(unit_type) {
                adjacent.push((x + 1, y, unit.hp));
            }
        }
        if let Tile::Open(Some(unit)) = &self[y + 1][x] {
            if unit.unit_type.is_opposite(unit_type) {
                adjacent.push((x, y + 1, unit.hp));
            }
        }
        adjacent.iter()
            .min_by_key(|(_, _, hp)| hp)
            .map(|(x, y, _)| (*x, *y))
    }

    fn move_unit(&mut self, (from_x, from_y): (usize, usize), (to_x, to_y): (usize, usize)) -> Tile {
        self[from_y].push(Tile::Open(None));
        let moved = self[from_y].swap_remove(from_x);
        self[to_y].push(moved);
        self[to_y].swap_remove(to_x)
    }

    fn remove_unit(&mut self, (x, y): (usize, usize)) {
        self[y].push(Tile::Open(None));
        self[y].swap_remove(x);
    }

    fn get_units_mut(&mut self) -> VecDeque<((usize, usize), &mut Unit)> {
        let mut list = VecDeque::new();
        for (y, row) in self.iter_mut().enumerate() {
            for (x, value) in row.iter_mut().enumerate() {
                if let Tile::Open(Some(unit)) = value {
                    list.push_back(((x, y), unit));
                }
            }
        }

        list
    }

    fn get_units(&self) -> VecDeque<((usize, usize), &Unit)> {
        let mut list = VecDeque::new();
        for (y, row) in self.iter().enumerate() {
            for (x, value) in row.iter().enumerate() {
                if let Tile::Open(Some(unit)) = value {
                    list.push_back(((x, y), unit));
                }
            }
        }

        list
    }

    fn find_nearest(&self, from: (usize, usize), unit_type: UnitType) -> Option<VecDeque<(usize, usize)>> {
        let units = self.get_units();
        let enemies: VecDeque<&((usize, usize), &Unit)> = units.iter()
            .filter(|(_, unit)| unit.unit_type == unit_type)
            .collect();

        let mut distances: Vec<Option<VecDeque<(usize, usize)>>> = vec![];

        thread::scope(|scope| {
            let mut inner_distance = vec![];

            enemies.iter().for_each(|((enemy_x, enemy_y), _)| {
                inner_distance.push(scope.spawn(move |_| {
                    self.shortest_distance(from, ((*enemy_x), (*enemy_y)))
                }));
            });

            inner_distance.into_iter()
                .for_each(|distance| distances.push(distance.join().unwrap_or(None)));
        }).unwrap();

//        distances.iter()
//            .for_each(|distance| println!("{:?}", distance));

        distances
            .into_iter()
            .filter(|value| value.is_some())
            .map(|value| value.unwrap())
            .min_by_key(|value| (value.len(), value[0].1, value[0].0))
    }

    fn shortest_distance(&self, (from_x, from_y): (usize, usize), (to_x, to_y): (usize, usize)) -> Option<VecDeque<(usize, usize)>> {
        let mut path = Vec::new();
        path.push((to_x, to_y, 0));

        let mut i = 1;
        let mut next = VecDeque::new();

        self.add_adjacent(&mut next, &path, (to_x, to_y, 1));

        while !next.is_empty() && !path.iter().any(|(x, y, _)| self.is_adjacent((*x, *y), (from_x, from_y))) {
            let drain = next.drain(0..next.len()).collect::<VecDeque<(usize, usize, usize)>>();

            drain
                .iter()
                .for_each(|(x, y, i)| {
                    if !path.iter().any(|(current_x, current_y, _)| current_x == x && current_y == y) {
                        path.push((*x, *y, *i));
                    }
                    self.add_adjacent(&mut next, &path, (*x, *y, *i + 1));
                })
        };


        if path.iter().any(|(x, y, _)| self.is_adjacent((*x, *y), (from_x, from_y))) {
            path.sort_by(|(x1, y1, z1), (x2, y2, z2)| (z1, y1, x1).cmp(&(z2, y2, x2)));
            path.remove(0);

//            path.iter().for_each(|inner| println!("Path values: {:?}", inner));

            Some(self.create_path(path, (from_x, from_y), (to_x, to_y)))
        } else {
            None
        }
    }

    fn create_path(
        &self,
        mut vec: Vec<(usize, usize, usize)>,
        (from_x, from_y): (usize, usize),
        (to_x, to_y): (usize, usize),
    ) -> VecDeque<(usize, usize)> {
        let mut returned = VecDeque::new();

        let (x, y, i) = vec.iter()
            .filter(|(x, y, _)| self.is_adjacent((*x, *y), (from_x, from_y)))
            .nth(0)
            .unwrap();

        returned.push_front((*x, *y));

        vec = vec.iter()
            .filter(|(_, _, inner)| i != inner)
            .map(|(x, y, i)| (*x, *y, *i))
            .collect();

        while !vec.is_empty() {
            let (from_x, from_y) = returned.front().unwrap();

            let (x, y, i) = vec.iter()
                .filter(|(x, y, _)| self.is_adjacent((*x, *y), (*from_x, *from_y)))
                .nth(0)
                .unwrap();

            returned.push_front((*x, *y));

            vec = vec.iter()
                .filter(|(_, _, inner)| i != inner)
                .map(|(x, y, i)| (*x, *y, *i))
                .collect();
        }

        returned
    }

    fn is_adjacent(&self, (x, y): (usize, usize), (from_x, from_y): (usize, usize)) -> bool {
        (x + 1 == from_x && y == from_y) ||
            (x - 1 == from_x && y == from_y) ||
            (x == from_x && y + 1 == from_y) ||
            (x == from_x && y - 1 == from_y)
    }

    fn add_adjacent(&self,
                    deque: &mut VecDeque<(usize, usize, usize)>,
                    vec: &Vec<(usize, usize, usize)>,
                    (to_x, to_y, i): (usize, usize, usize)) {
        self.add_if_valid(deque, vec, (to_x + 1, to_y, i));
        self.add_if_valid(deque, vec, (to_x - 1, to_y, i));
        self.add_if_valid(deque, vec, (to_x, to_y + 1, i));
        self.add_if_valid(deque, vec, (to_x, to_y - 1, i));
    }

    fn add_if_valid(&self,
                    deque: &mut VecDeque<(usize, usize, usize)>,
                    vec: &Vec<(usize, usize, usize)>,
                    (to_x, to_y, i): (usize, usize, usize),
    ) {
        if !vec.iter().any(|(x, y, _)| *x == to_x && *y == to_y) &&
            !deque.iter().any(|(x, y, _)| *x == to_x && *y == to_y) {
            if let Tile::Open(None) = self[to_y][to_x] {
                deque.push_front((to_x, to_y, i))
            }
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for row in self.iter() {
            for value in row.iter() {
                write!(f, "{}", value)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Deref for Map {
    type Target = Vec<Vec<Tile>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Map {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Index<usize> for Map {
    type Output = Vec<Tile>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Map {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[derive(Debug, Clone)]
struct Unit {
    attack_power: i16,
    hp: i16,
    unit_type: UnitType,
}

impl Unit {
    fn is_goblin(&self) -> bool {
        use crate::UnitType::*;

        match self.unit_type {
            Goblin => true,
            _ => false
        }
    }

    fn is_elf(&self) -> bool {
        !self.is_goblin()
    }
}

impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        use crate::UnitType::*;

        match self.unit_type {
            Goblin => write!(f, "G")?,
            Elf => write!(f, "E")?
        }

        Ok(())
    }
}

impl From<UnitType> for Unit {
    fn from(unit_type: UnitType) -> Self {
        Unit { attack_power: 3, hp: 200, unit_type }
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Copy)]
enum UnitType { Goblin, Elf }

impl UnitType {
    fn get_opposite(self) -> Self {
        match self {
            UnitType::Goblin => UnitType::Elf,
            UnitType::Elf => UnitType::Goblin
        }
    }

    fn is_opposite(self, other: UnitType) -> bool {
        self == other.get_opposite()
    }
}

#[derive(Debug, Clone)]
enum Tile {
    Wall,
    Open(Option<Unit>),
}

impl From<char> for Tile {
    fn from(character: char) -> Self {
        match character {
            '#' => Tile::Wall,
            '.' => Tile::Open(None),
            'G' => Tile::Open(Some(Unit::from(UnitType::Goblin))),
            'E' => Tile::Open(Some(Unit::from(UnitType::Elf))),
            _ => panic!("Unexpected character in input")
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        use crate::Tile::*;

        match self {
            Wall => write!(f, "#")?,
            Open(None) => write!(f, ".")?,
            Open(Some(unit)) => write!(f, "{}", unit)?
        }

        Ok(())
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_from_char() {
        assert!(
            match Tile::from('#') {
                Tile::Wall => true,
                _ => false
            });
        assert!(
            match Tile::from('.') {
                Tile::Open(None) => true,
                _ => false
            });
        assert!(
            match Tile::from('G') {
                Tile::Open(Some(unit)) => unit.is_goblin(),
                _ => false
            });
        assert!(
            match Tile::from('E') {
                Tile::Open(Some(unit)) => unit.is_elf(),
                _ => false
            });
    }
}