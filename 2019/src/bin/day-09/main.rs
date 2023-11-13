use std::cell::RefCell;
use std::fmt::Display;
use std::rc::{Rc, Weak};
use std::fmt::Debug;

fn main() {
    let mut game = Game::new(452);
    for i in 1..=7078400 {
        game.insert_marble(i);
    }

    println!("{:?}", game.players.iter().max());
    drop(game);
}

#[derive(Debug)]
struct Game {
    game_board: LinkedList<usize>,
    players: Vec<usize>,
    players_turn: usize,
}

impl Game {
    fn new(num_of_players: usize) -> Game {
        let players = vec![0; num_of_players];
        Game {
            game_board: LinkedList::new(),
            players,
            players_turn: 0,
        }
    }

    fn insert_marble(&mut self, marble: usize) {
        if marble % 23 == 0 {
            self.insert_23(marble);
        } else {
            self.game_board.next();
            self.game_board.insert_after(marble);
            self.players_turn = (self.players_turn + 1) % self.players.len();
        }
    }

    fn insert_23(&mut self, marble: usize) {
        for _ in 0..7 {
            self.game_board.back();
        }
        let score = self.game_board.remove_current() + marble;
        self.players[self.players_turn] = self.players[self.players_turn] + score;
        self.players_turn = (self.players_turn + 1) % self.players.len();
    }
}

#[derive(Debug)]
struct Node<T>
    where T: Sized + Debug {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    previous: Option<Weak<RefCell<Node<T>>>>,
}

impl<T> Node<T>
    where T: Sized + Debug {
    fn new(value: T) -> Node<T> {
        Node { value, next: None, previous: None }
    }
}

#[derive(Debug)]
struct LinkedList<T>
    where T: Sized + Debug + Default {
    front: Rc<RefCell<Node<T>>>,
    back: Weak<RefCell<Node<T>>>,
    current: Weak<RefCell<Node<T>>>,
}

impl<T> LinkedList<T>
    where T: Sized + Debug + Default {
    fn new() -> LinkedList<T> {
        let front = Rc::new(RefCell::new(Node::new(T::default())));
        LinkedList { front: front.clone(), back: Rc::downgrade(&front), current: Rc::downgrade(&front) }
    }

    fn insert_after(&mut self, value: T) {
        let mut node = Node::new(value);

        node.previous = Some(self.current.clone());
        node.next = match self.current.upgrade() {
            Some(rc) => rc.borrow().next.clone(),
            None => panic!("Unreachable"),
        };

        let rc = Rc::new(RefCell::new(node));

        let new_current = Rc::downgrade(&rc);

        match self.current.upgrade() {
            Some(inner) => {
                match &inner.borrow_mut().next {
                    Some(next) => next.borrow_mut().previous = Some(new_current.clone()),
                    None => self.back = new_current.clone()
                }
                inner.borrow_mut().next = Some(rc)
            }
            None => panic!("Unreachable"),
        }
        self.current = new_current;
    }

    fn next(&mut self) {
        self.current = match self.current.upgrade() {
            Some(rc) => match &rc.borrow().next {
                Some(next) => Rc::downgrade(&next),
                None => Rc::downgrade(&self.front)
            },
            None => panic!("Unreachable")
        };
    }

    fn back(&mut self) {
        self.current = match &self.current.upgrade().expect("Unreachable").borrow().previous {
            Some(weak) => weak.clone(),
            None => self.back.clone()
        };
    }

    fn remove_current(&mut self) -> T {
        let current = self.current.upgrade().expect("Unreachable");
        let (previous, next) = (
            current.borrow().previous.clone().map(|inner| inner.upgrade().expect("Unreachable")),
            current.borrow().next.clone()
        );

        if previous.is_none() && next.is_none() {
            panic!("Cannot remove if only one node is in Linked List");
        }
        match previous.clone() {
            Some(rc) => rc.borrow_mut().next = next.clone(),
            None => self.front = next.clone().unwrap()
        }

        match next.clone() {
            Some(rc) => {
                rc.borrow_mut().previous = previous.map(|inner| Rc::downgrade(&inner));
                self.current = Rc::downgrade(&rc)
            }
            None => self.back = Rc::downgrade(&previous.unwrap())
        }
        Rc::try_unwrap(current).expect("Unreachable").into_inner().value
    }
}

impl<T: Debug + Default> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while let Some(next) = self.front.clone().borrow().next.clone() {
            self.front = next.clone()
        }
    }
}

impl<T> Display for LinkedList<T>
    where T: Display + Debug + Default {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[")?;
        let mut current = Some(Rc::downgrade(&self.front));
        loop {
            match current.clone() {
                Some(value) => match value.upgrade() {
                    Some(rc) => {
                        current = match &rc.borrow().next {
                            Some(inner) => {
                                write!(f, "{}, ", rc.borrow().value)?;
                                Some(Rc::downgrade(&inner))
                            }
                            None => {
                                write!(f, "{}", rc.borrow().value)?;
                                None
                            }
                        }
                    }
                    None => panic!("Unreachable"),
                },
                None => break,
            }
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn new_game_creates() {
        let game = Game::new(12);
        assert_eq!(game.players.len(), 12)
    }
}
