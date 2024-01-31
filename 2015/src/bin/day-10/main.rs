use aoc_2015::lines_for_day;

fn main() {
    let line = lines_for_day("10").pop().expect("No Input");
    let part_one_answer = part_one(line);
    println!("Part One: {}", part_one_answer.len());
    println!("Part One: {}", part_two(part_one_answer).len());
}

fn look_and_say(nums: String) -> String {
    let mut nums = nums.chars();
    let mut current = nums.next().expect("No Input");
    let mut output = "".to_string();
    let mut count = 1;
    for num in nums {
        if current == num {
            count += 1;
        } else {
            output.push_str(&count.to_string());
            output.push(current);
            current = num;
            count = 1;
        }
    }
    output.push_str(&count.to_string());
    output.push(current);

    output
}

fn part_one(input: String) -> String {
    let mut input = input;
    for _ in 0..40 {
        input = look_and_say(input);
    }
    input
}

fn part_two(input: String) -> String {
    let mut input = input;
    for _ in 0..10 {
        input = look_and_say(input);
    }
    input
}
