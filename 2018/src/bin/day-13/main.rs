use std::cell::RefCell;
use std::fmt::{Display, Error, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::rc::{Rc, Weak};

fn main() {
    let reader = BufReader::new(
        File::open(Path::new("../input/2018/13/input.txt")).unwrap());

    let lines = reader.lines()
        .map(|line| line.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let (mut tracks, mut cars) = TrackSystem::initialize(lines);

    while cars.len() > 1 {
        cars.sort_by_key(|car| car.borrow().location);
        cars = tracks.tick(cars);
    }

    println!("{}", tracks);

    println!("{:?}", cars)
}

#[derive(Debug)]
struct TrackSystem(Vec<Vec<Option<Track>>>);

impl TrackSystem {
    fn initialize(characters: Vec<Vec<char>>) -> (Self, Vec<Rc<RefCell<Car>>>) {
        let mut tracks = TrackSystem(vec![vec![None; characters.len()]; characters[0].len()]);

        let mut cars = vec![];

        for (y, line) in characters.iter().enumerate() {
            for (x, character) in line.iter().enumerate() {
                tracks[x][y] = match *character {
                    '-' => Some(Track::Horizontal(None)),
                    '|' => Some(Track::Vertical(None)),
                    '/' => if x == 0 {
                        Some(Track::UpAndRight(None))
                    } else {
                        match tracks[x - 1][y] {
                            Some(Track::Horizontal(_)) | Some(Track::Intersection(_)) => Some(Track::DownAndLeft(None)),
                            _ => Some(Track::UpAndRight(None))
                        }
                    },
                    '\\' => if x == 0 {
                        Some(Track::LeftAndUp(None))
                    } else {
                        match tracks[x - 1][y] {
                            Some(Track::Horizontal(_)) | Some(Track::Intersection(_)) => Some(Track::RightAndDown(None)),
                            _ => Some(Track::LeftAndUp(None))
                        }
                    },
                    '+' => Some(Track::Intersection(None)),
                    direction @ '>' | direction @ '<' => {
                        let car = Rc::new(RefCell::new(Car::new(direction, (x, y))));
                        let weak = Rc::downgrade(&car);
                        cars.push(car);
                        Some(Track::Horizontal(Some(weak)))
                    }
                    direction @ 'v' | direction @ '^' => {
                        let car = Rc::new(RefCell::new(Car::new(direction, (x, y))));
                        let weak = Rc::downgrade(&car);
                        cars.push(car);
                        Some(Track::Vertical(Some(weak)))
                    }
                    _ => None
                }
            }
        }

        (tracks, cars)
    }

    fn tick(&mut self, cars: Vec<Rc<RefCell<Car>>>) -> Vec<Rc<RefCell<Car>>> {
        let mut cars = cars;
        let mut cars_to_remove = vec![];
        for car in cars.clone() {
            if let Err((car1, car2)) = self.move_car(car) {
                cars_to_remove.push(car1);
                cars_to_remove.push(car2);
            }
        }
        for car in cars_to_remove {
            cars.retain(|inner| inner.borrow().location != car.borrow().location);
            let (x, y) = car.borrow().location;
            if let Some(track) = &mut self[x][y] {
                track.remove_car();
            }
        }
        cars
    }

    fn move_car(&mut self, car: Rc<RefCell<Car>>) -> Result<(), (Rc<RefCell<Car>>, Rc<RefCell<Car>>)> {
        let (x, y) = car.borrow().location;
        let (new_x, new_y) = match car.borrow().direction {
            Direction::Up => (x, y - 1),
            Direction::Left => (x - 1, y),
            Direction::Down => (x, y + 1),
            Direction::Right => (x + 1, y),
        };

        let new_location = &mut self[new_x][new_y];

        if let Some(track) = new_location {
            if track.has_car() {
                let next_car = track.get_car();
                
                return Err((car.clone(), next_car));
            }
        }

        {
            let mut car = car.borrow_mut();

            match new_location {
                Some(location) => {
                    car.new_direction(&location);
                }
                None => panic!("Track must be at this location")
            };

            car.location = (new_x, new_y);
        }

        if let Some(location) = new_location {
            location.put_car(Rc::downgrade(&car))
        }

        if let Some(remove) = &mut self[x][y] {
            remove.remove_car()
        }

        Ok(())
    }
}

impl Deref for TrackSystem {
    type Target = Vec<Vec<Option<Track>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TrackSystem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::ops::Index<usize> for TrackSystem {
    type Output = Vec<Option<Track>>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for TrackSystem {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Display for TrackSystem {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let y_len = self.len();
        let x_len = self[0].len();

        for y in 0..x_len {
            for x in 0..y_len {
                match self[x][y] {
                    Some(ref track) => write!(f, "{}", track)?,
                    None => write!(f, " ")?
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

type RcCar = Option<Weak<RefCell<Car>>>;

#[derive(Clone, Debug)]
enum Track {
    Vertical(RcCar),
    Horizontal(RcCar),
    UpAndRight(RcCar),
    RightAndDown(RcCar),
    DownAndLeft(RcCar),
    LeftAndUp(RcCar),
    Intersection(RcCar),
}

impl Track {
    fn put_car(&mut self, car: Weak<RefCell<Car>>) {
        match self {
            Track::Vertical(rc_car) => *rc_car = Some(car),
            Track::Horizontal(rc_car) => *rc_car = Some(car),
            Track::UpAndRight(rc_car) => *rc_car = Some(car),
            Track::RightAndDown(rc_car) => *rc_car = Some(car),
            Track::DownAndLeft(rc_car) => *rc_car = Some(car),
            Track::LeftAndUp(rc_car) => *rc_car = Some(car),
            Track::Intersection(rc_car) => *rc_car = Some(car),
        }
    }

    fn remove_car(&mut self) {
        match self {
            Track::Vertical(rc_car) => *rc_car = None,
            Track::Horizontal(rc_car) => *rc_car = None,
            Track::UpAndRight(rc_car) => *rc_car = None,
            Track::RightAndDown(rc_car) => *rc_car = None,
            Track::DownAndLeft(rc_car) => *rc_car = None,
            Track::LeftAndUp(rc_car) => *rc_car = None,
            Track::Intersection(rc_car) => *rc_car = None,
        }
    }

    fn has_car(&self) -> bool {
        match self {
            Track::Vertical(rc_car) => rc_car.is_some(),
            Track::Horizontal(rc_car) => rc_car.is_some(),
            Track::UpAndRight(rc_car) => rc_car.is_some(),
            Track::RightAndDown(rc_car) => rc_car.is_some(),
            Track::DownAndLeft(rc_car) => rc_car.is_some(),
            Track::LeftAndUp(rc_car) => rc_car.is_some(),
            Track::Intersection(rc_car) => rc_car.is_some(),
        }
    }
    
    fn get_car(&self) -> Rc<RefCell<Car>> {
        match self {
            Track::Vertical(Some(rc_car)) => rc_car.upgrade().expect("Car should be here"),
            Track::Horizontal(Some(rc_car)) => rc_car.upgrade().expect("Car should be here"),
            Track::UpAndRight(Some(rc_car)) => rc_car.upgrade().expect("Car should be here"),
            Track::RightAndDown(Some(rc_car)) => rc_car.upgrade().expect("Car should be here"),
            Track::DownAndLeft(Some(rc_car)) => rc_car.upgrade().expect("Car should be here"),
            Track::LeftAndUp(Some(rc_car)) => rc_car.upgrade().expect("Car should be here"),
            Track::Intersection(Some(rc_car)) => rc_car.upgrade().expect("Car should be here"),
            _ => panic!("Cannot get if no car is available")
        }
    }
}

impl Display for Track {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", match self {
            Track::Vertical(Some(car)) => if let Some(car) = car.upgrade() {
                car.borrow().as_char()
            } else {
                panic!("Rc has been dropped");
            },
            Track::Vertical(None) => '|',
            Track::Horizontal(Some(car)) => if let Some(car) = car.upgrade() {
                car.borrow().as_char()
            } else {
                panic!("Rc has been dropped");
            },
            Track::Horizontal(None) => '-',
            Track::UpAndRight(Some(car)) => if let Some(car) = car.upgrade() {
                car.borrow().as_char()
            } else {
                panic!("Rc has been dropped");
            },
            Track::UpAndRight(None) => '/',
            Track::RightAndDown(Some(car)) => if let Some(car) = car.upgrade() {
                car.borrow().as_char()
            } else {
                panic!("Rc has been dropped");
            },
            Track::RightAndDown(None) => '\\',
            Track::DownAndLeft(Some(car)) => if let Some(car) = car.upgrade() {
                car.borrow().as_char()
            } else {
                panic!("Rc has been dropped");
            },
            Track::DownAndLeft(None) => '/',
            Track::LeftAndUp(Some(car)) => if let Some(car) = car.upgrade() {
                car.borrow().as_char()
            } else {
                panic!("Rc has been dropped");
            },
            Track::LeftAndUp(None) => '\\',
            Track::Intersection(Some(car)) => if let Some(car) = car.upgrade() {
                car.borrow().as_char()
            } else {
                panic!("Rc has been dropped");
            },
            Track::Intersection(None) => '+',
        })
    }
}

#[derive(Debug)]
struct Car {
    direction: Direction,
    location: (usize, usize),
    next_direction: Direction,
}

impl Car {
    fn new(direction: char, location: (usize, usize)) -> Self {
        let direction = match direction {
            '^' => Direction::Up,
            '>' => Direction::Right,
            '<' => Direction::Left,
            'v' => Direction::Down,
            _ => panic!("Invalid direction supplied")
        };
        Car { direction, location, next_direction: Direction::default() }
    }

    fn as_char(&self) -> char {
        match self.direction {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }

    fn new_direction(&mut self, track: &Track) {
        match track {
            Track::Intersection(_) => {
                match (&self.direction, &self.next_direction) {
                    (Direction::Right, Direction::Left) => {
                        self.direction = Direction::Up;
                        self.next_direction = Direction::Up
                    }
                    (_, Direction::Up) => self.next_direction = Direction::Right,
                    (Direction::Right, Direction::Right) => {
                        self.direction = Direction::Down;
                        self.next_direction = Direction::Left;
                    }
                    (Direction::Down, Direction::Left) => {
                        self.direction = Direction::Right;
                        self.next_direction = Direction::Up;
                    }
                    (Direction::Down, Direction::Right) => {
                        self.direction = Direction::Left;
                        self.next_direction = Direction::Left;
                    }

                    (Direction::Left, Direction::Left) => {
                        self.direction = Direction::Down;
                        self.next_direction = Direction::Up;
                    }
                    (Direction::Left, Direction::Right) => {
                        self.direction = Direction::Up;
                        self.next_direction = Direction::Left;
                    }

                    (Direction::Up, Direction::Left) => {
                        self.direction = Direction::Left;
                        self.next_direction = Direction::Up;
                    }
                    (Direction::Up, Direction::Right) => {
                        self.direction = Direction::Right;
                        self.next_direction = Direction::Left;
                    }
                    _ => panic!("Invalid Direction combination")
                }
            }
            Track::LeftAndUp(_) => {
                match self.direction {
                    Direction::Left => self.direction = Direction::Up,
                    Direction::Down => self.direction = Direction::Right,
                    _ => panic!("Invalid Direction combination")
                }
            }
            Track::DownAndLeft(_) => {
                match self.direction {
                    Direction::Down => self.direction = Direction::Left,
                    Direction::Right => self.direction = Direction::Up,
                    _ => panic!("Invalid Direction combination")
                }
            }
            Track::RightAndDown(_) => {
                match self.direction {
                    Direction::Right => self.direction = Direction::Down,
                    Direction::Up => self.direction = Direction::Left,
                    _ => panic!("Invalid Direction combination")
                }
            }
            Track::UpAndRight(_) => {
                match self.direction {
                    Direction::Up => self.direction = Direction::Right,
                    Direction::Left => self.direction = Direction::Down,
                    _ => panic!("Invalid Direction combination")
                }
            }
            _ => ()
        }
    }
}

impl Display for Car {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.as_char())
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Left
    }
}