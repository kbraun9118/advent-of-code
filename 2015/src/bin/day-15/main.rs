use std::vec;

use aoc_2015::lines_for_day;

fn main() {
    let ingredients = lines_for_day("15")
        .into_iter()
        .map(Ingredient::from)
        .collect::<Vec<_>>();

    let ingredients = sum_100(ingredients.len() as u32)
        .into_iter()
        .map(|combo| {
            combo
                .into_iter()
                .map(|num| num as i32)
                .zip(ingredients.clone())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let part_1 = ingredients.iter().map(score).max();

    println!("Part 1: {:?}", part_1.unwrap());

    let part_2 = ingredients.iter().map(score_2).max();

    println!("Part 2: {:?}", part_2.unwrap());
}

fn score(recipe: &Vec<(i32, Ingredient)>) -> i32 {
    let capacity: i32 = recipe
        .iter()
        .map(|(i, ingredient)| i * ingredient.capacity)
        .sum();
    let durability: i32 = recipe
        .iter()
        .map(|(i, ingredient)| i * ingredient.durability)
        .sum();
    let flavor: i32 = recipe
        .iter()
        .map(|(i, ingredient)| i * ingredient.flavor)
        .sum();
    let texture: i32 = recipe
        .iter()
        .map(|(i, ingredient)| i * ingredient.texture)
        .sum();

    if capacity.is_negative()
        || durability.is_negative()
        || flavor.is_negative()
        || texture.is_negative()
    {
        0
    } else {
        capacity * durability * flavor * texture
    }
}

fn score_2(recipe: &Vec<(i32, Ingredient)>) -> i32 {
    if recipe.iter().map(|(n, i)| n * i.calories).sum::<i32>() != 500 {
        0
    } else {
        score(recipe)
    }
}

fn sum_100(n: u32) -> Vec<Vec<u32>> {
    (0..=(100u32.pow(n)))
        .into_iter()
        .map(|next| {
            let mut num = next;
            let mut current = vec![];
            while num >= 100 {
                current.push(num % 100);
                num /= 100;
            }
            current.push(num);
            while current.len() < n as usize {
                current.push(0);
            }
            current
        })
        .filter(|v| v.iter().sum::<u32>() == 100)
        .collect::<Vec<_>>()
}

#[derive(Debug, Clone, Copy)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl From<String> for Ingredient {
    fn from(value: String) -> Self {
        let split = value.split(' ').collect::<Vec<_>>();
        Self {
            capacity: split[2].strip_suffix(',').unwrap().parse().unwrap(),
            durability: split[4].strip_suffix(',').unwrap().parse().unwrap(),
            flavor: split[6].strip_suffix(',').unwrap().parse().unwrap(),
            texture: split[8].strip_suffix(',').unwrap().parse().unwrap(),
            calories: split[10].parse().unwrap(),
        }
    }
}
