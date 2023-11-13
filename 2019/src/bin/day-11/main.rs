//use std::io::BufReader;
//use std::fs::File;
//use std::path::Path;
//use std::io::BufRead;
use std::sync::Mutex;
use std::sync::Arc;

const SERIAL_NUMBER: i32 = 5468;

fn main() {
//    let reader = BufReader::new(
//        File::open(Path::new("input/input.txt")).unwrap());

    let mut grid = vec![vec![0; 301]; 301];

    for y in 1..=300 {
        for x in 1..=300 {
            grid[x][y] = get_power_level(x as i32, y as i32);
        }
    }

    print_grid(&grid);

    let (x, y, grid_size) = find_grid_coordinate(&grid);
    println!("x: {}, y: {}, grid size: {}", x, y, grid_size);
}

fn get_hundreds(value: i32) -> i32 {
    (value / 100) - ((value / 1000) * 10)
}

fn get_power_level(x: i32, y: i32) -> i32 {
    let rack_id = x + 10;
    get_hundreds((rack_id * y + SERIAL_NUMBER) * rack_id) - 5
}

fn print_grid(grid: &Vec<Vec<i32>>) {
    for y in 0..=300 {
        for x in 0..=300 {
            if x == 0 && y == 0 {
                print!("{:3}    ", 0)
            } else if x == 0 {
                print!("{:3}    ", y)
            } else if y == 0 {
                print!("{:3}    ", x)
            } else {
                print!("{:3}    ", grid[x][y])
            }
        }
        println!()
    }
}

struct Answer {
    max_x: i32,
    max_y: i32,
    max: i32,
    max_grid_size: i32,
}

fn find_grid_coordinate(grid: &Vec<Vec<i32>>) -> (i32, i32, i32) {
    let answer_mutex = Arc::new(Mutex::new(Answer {
        max_x: 0,
        max_y: 0,
        max: 0,
        max_grid_size: 0,
    }));

    let pool = rayon::ThreadPoolBuilder::new().num_threads(20).build().unwrap();


    pool.install(||
        rayon::scope(|s|
            {
                for grid_size in 1..=300 {
                    println!("Testing grid size: {}", grid_size);

                    let answer_mutex = answer_mutex.clone();
                    s.spawn(move |_|
                        {
                            for y in 1..300 - (grid_size - 1) {
                                for x in 1..300 - (grid_size - 1) {
                                    let mut sum = 0;
                                    for inner_x in x..x + grid_size {
                                        for inner_y in y..y + grid_size {
                                            sum += grid[inner_x][inner_y];
                                        };
                                    };
                                    {
                                        let mut answer_mutex = answer_mutex.lock().unwrap();

                                        if sum > answer_mutex.max {
                                            answer_mutex.max_x = x as i32;
                                            answer_mutex.max_y = y as i32;
                                            answer_mutex.max = sum;
                                            answer_mutex.max_grid_size = grid_size as i32;
                                        };
                                    }
                                }
                            }
                        }
                    );
                }
            }
        )
    );

    let answer_mutex = answer_mutex.clone();

    let answer = answer_mutex.lock().unwrap();
    (answer.max_x, answer.max_y, answer.max_grid_size)
}

