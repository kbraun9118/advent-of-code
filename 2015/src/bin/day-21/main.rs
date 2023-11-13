#![allow(dead_code)]

use aoc_2015::lines_for_day;

fn main() {
    let lines = lines_for_day("day-21")
        .into_iter()
        .map(|l| l.split_once(": ").unwrap().1.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let boss = Boss {
        hp: lines[0],
        damage: lines[1],
        armor: lines[2],
    };

    let part1 = construct_players(100)
        .into_iter()
        .filter_map(|p| figth_to_death(p, boss.clone()))
        .min();

    println!("Part 1: {}", part1.unwrap());

    let part2 = construct_players(100)
        .into_iter()
        .filter_map(|p| figth_to_death_2(p, boss.clone()))
        .max();

    println!("Part 2: {}", part2.unwrap());
}

fn figth_to_death(mut player: Player, mut boss: Boss) -> Option<i32> {
    if player.equipment.ring1.is_some() && player.equipment.ring1 == player.equipment.ring2 {
        return None;
    }
    while player.hp > 0 {
        boss.fight(&player);
        if boss.hp <= 0 {
            return Some(player.cost());
        }
        player.fight(&boss)
    }
    None
}

fn figth_to_death_2(mut player: Player, mut boss: Boss) -> Option<i32> {
    if player.equipment.ring1.is_some() && player.equipment.ring1 == player.equipment.ring2 {
        return None;
    }
    while player.hp > 0 {
        boss.fight(&player);
        if boss.hp <= 0 {
            return None;
        }
        player.fight(&boss)
    }
    Some(player.cost())
}

fn construct_players(hp: i32) -> Vec<Player> {
    weapons()
        .into_iter()
        .flat_map(|w| armor().into_iter().map(move |a| (w.clone(), a)))
        .flat_map(|(w, a)| rings().into_iter().map(move |r| (w.clone(), a.clone(), r)))
        .flat_map(|(w, a, r1)| {
            rings()
                .into_iter()
                .map(move |r2| (w.clone(), a.clone(), r1.clone(), r2))
        })
        .map(|(weapon, armor, ring1, ring2)| Player {
            hp,
            equipment: Equipment {
                weapon,
                armor,
                ring1,
                ring2,
            },
        })
        .collect()
}

fn armor() -> Vec<Option<Armor>> {
    vec![
        None,
        Some(Armor {
            name: "Leather".into(),
            cost: 13,
            armor: 1,
        }),
        Some(Armor {
            name: "Chainmail".into(),
            cost: 31,
            armor: 2,
        }),
        Some(Armor {
            name: "Splintmail".into(),
            cost: 53,
            armor: 3,
        }),
        Some(Armor {
            name: "Bandedmail".into(),
            cost: 75,
            armor: 4,
        }),
        Some(Armor {
            name: "Platemail".into(),
            cost: 102,
            armor: 5,
        }),
    ]
}

fn rings() -> Vec<Option<Ring>> {
    vec![
        None,
        None,
        Some(Ring {
            name: "Damage +1".into(),
            cost: 25,
            damage: 1,
            armor: 0,
        }),
        Some(Ring {
            name: "Damage +2".into(),
            cost: 50,
            damage: 2,
            armor: 0,
        }),
        Some(Ring {
            name: "Damage +3".into(),
            cost: 100,
            damage: 3,
            armor: 0,
        }),
        Some(Ring {
            name: "Defense +1".into(),
            cost: 20,
            damage: 0,
            armor: 1,
        }),
        Some(Ring {
            name: "Defense +2".into(),
            cost: 40,
            damage: 0,
            armor: 2,
        }),
        Some(Ring {
            name: "Defense +3".into(),
            cost: 80,
            damage: 0,
            armor: 3,
        }),
    ]
}

fn weapons() -> Vec<Weapon> {
    vec![
        Weapon {
            name: "Dagger".into(),
            cost: 8,
            damage: 4,
        },
        Weapon {
            name: "Shortsword".into(),
            cost: 10,
            damage: 5,
        },
        Weapon {
            name: "Warhammer".into(),
            cost: 25,
            damage: 6,
        },
        Weapon {
            name: "Longsword".into(),
            cost: 40,
            damage: 7,
        },
        Weapon {
            name: "Greataxe".into(),
            cost: 74,
            damage: 8,
        },
    ]
}

trait Fightable {
    fn armor(&self) -> i32;

    fn damage(&self) -> i32;

    fn hp(&mut self) -> &mut i32;

    fn fight(&mut self, other: &impl Fightable) {
        let damage_done = other.damage() - self.armor();

        if damage_done < 1 {
            *self.hp() -= 1;
        } else {
            *self.hp() -= damage_done;
        }
    }
}

#[derive(Debug)]
struct Player {
    hp: i32,
    equipment: Equipment,
}

impl Fightable for Player {
    fn armor(&self) -> i32 {
        self.equipment
            .armor
            .as_ref()
            .map(|r| r.armor)
            .unwrap_or_default()
            + self
                .equipment
                .ring1
                .as_ref()
                .map(|r| r.armor)
                .unwrap_or_default()
            + self
                .equipment
                .ring2
                .as_ref()
                .map(|r| r.armor)
                .unwrap_or_default()
    }

    fn damage(&self) -> i32 {
        self.equipment.weapon.damage
            + self
                .equipment
                .ring1
                .as_ref()
                .map(|r| r.damage)
                .unwrap_or_default()
            + self
                .equipment
                .ring2
                .as_ref()
                .map(|r| r.damage)
                .unwrap_or_default()
    }

    fn hp(&mut self) -> &mut i32 {
        &mut self.hp
    }
}

impl Player {
    fn cost(&self) -> i32 {
        self.equipment.weapon.cost
            + self
                .equipment
                .armor
                .as_ref()
                .map(|e| e.cost)
                .unwrap_or_default()
            + self
                .equipment
                .ring1
                .as_ref()
                .map(|e| e.cost)
                .unwrap_or_default()
            + self
                .equipment
                .ring2
                .as_ref()
                .map(|e| e.cost)
                .unwrap_or_default()
    }
}

#[derive(Debug, Clone)]
struct Boss {
    hp: i32,
    armor: i32,
    damage: i32,
}

impl Fightable for Boss {
    fn armor(&self) -> i32 {
        self.armor
    }

    fn damage(&self) -> i32 {
        self.damage
    }

    fn hp(&mut self) -> &mut i32 {
        &mut self.hp
    }
}

#[derive(Debug, Clone)]

struct Equipment {
    weapon: Weapon,
    armor: Option<Armor>,
    ring1: Option<Ring>,
    ring2: Option<Ring>,
}

#[derive(Debug, Clone)]
struct Weapon {
    name: String,
    cost: i32,
    damage: i32,
}

#[derive(Debug, Clone)]
struct Armor {
    name: String,
    cost: i32,
    armor: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Ring {
    name: String,
    cost: i32,
    damage: i32,
    armor: i32,
}
