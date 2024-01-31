#![allow(unused)]

use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::ops::Index;
use std::ops::IndexMut;
use std::path::Path;

fn main() {
    let reader = BufReader::new(
        File::open(Path::new("../input/2018/16/input.txt")).unwrap());

    let lines: Vec<String> = reader.lines()
        .into_iter()
        .map(|line| line.unwrap())
        .collect();

    let (dictionary, opts) = split_strings(lines);

    let operations: Vec<Operation> = opts
        .iter()
        .map(|op| Operation::from(op.as_str()))
        .collect();
//    println!("{:?}", dictionary);

//    dictionary.iter().for_each(|line| println!("{}", line));
    let dictionary = Dictionary::new(dictionary);

    let over_three = dictionary.dictionary.iter()
        .map(|entry| entry.test_entry().len())
        .filter(|count| *count >= 3)
        .count();

    println!("{:?} operations have over 3 possible ops.", over_three);

//    dictionary.dictionary[5].test_entry().iter()
//        .for_each(|op| println!("{:?}", operation_to_string(*op)));

    let codes = Dictionary::determine_op_code();


    let mut register = Register::default();

    operations.iter()
        .for_each(|operation| {
            let op = *codes.get(&operation.op_code).unwrap();
            register = op(&operation, register);
        });

    println!("{:?}", register);
}

fn split_strings(strings: Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut index = 0;
    for i in 0..strings.len() {
        if strings[i].is_empty() && strings[i + 1].is_empty() && strings[i + 2].is_empty() {
            index = i;
        }
    }
    let dictionary = strings[0..index]
        .to_vec()
        .into_iter()
        .filter(|line| !line.is_empty())
        .collect();
    let opts = strings[index + 3..strings.len()].to_vec();

    (dictionary, opts)
}

#[derive(Debug)]
struct Dictionary {
    dictionary: Vec<DictionaryEntry>
}

impl Dictionary {
    fn new(dictionary: Vec<String>) -> Self {
        let mut entries = vec![];
        for i in (0..dictionary.len()).step_by(3) {
            entries.push(DictionaryEntry::new(Register::from(&*dictionary[i]),
                                              Operation::from(&*dictionary[i + 1]),
                                              Register::from(&*dictionary[i + 2])));
        }

        Dictionary { dictionary: entries }
    }

    fn determine_op_code() -> HashMap<u8, fn(&Operation, Register) -> Register> {
        let mut map: HashMap<u8, fn(&Operation, Register) -> Register> = HashMap::new();
        map.insert(0, Operation::muli);
        map.insert(1, Operation::bani);
        map.insert(2, Operation::addi);
        map.insert(3, Operation::seti);
        map.insert(4, Operation::eqrr);
        map.insert(5, Operation::eqir);
        map.insert(6, Operation::setr);
        map.insert(7, Operation::bori);
        map.insert(8, Operation::gtri);
        map.insert(9, Operation::eqri);
        map.insert(10, Operation::gtir);
        map.insert(11, Operation::borr);
        map.insert(12, Operation::addr);
        map.insert(13, Operation::gtrr);
        map.insert(14, Operation::mulr);
        map.insert(15, Operation::banr);
        
        map
    }
}

fn operation_to_string(operation: fn(&Operation, Register) -> Register) -> String {
    if operation as usize == Operation::addr as usize {
        "addr".to_string()
    } else if operation as usize == Operation::addi as usize {
        "addi".to_string()
    } else if operation as usize == Operation::mulr as usize {
        "mulr".to_string()
    } else if operation as usize == Operation::muli as usize {
        "muli".to_string()
    } else if operation as usize == Operation::banr as usize {
        "banr".to_string()
    } else if operation as usize == Operation::bani as usize {
        "bani".to_string()
    } else if operation as usize == Operation::borr as usize {
        "borr".to_string()
    } else if operation as usize == Operation::bori as usize {
        "bori".to_string()
    } else if operation as usize == Operation::setr as usize {
        "setr".to_string()
    } else if operation as usize == Operation::seti as usize {
        "seti".to_string()
    } else if operation as usize == Operation::gtir as usize {
        "gtir".to_string()
    } else if operation as usize == Operation::gtri as usize {
        "gtri".to_string()
    } else if operation as usize == Operation::gtrr as usize {
        "gtrr".to_string()
    } else if operation as usize == Operation::eqir as usize {
        "eqir".to_string()
    } else if operation as usize == Operation::eqri as usize {
        "eqri".to_string()
    } else {
        "eqrr".to_string()
    }
}

#[derive(Debug)]
struct DictionaryEntry {
    before: Register,
    operation: Operation,
    after: Register,
}

impl DictionaryEntry {
    fn new(before: Register, operation: Operation, after: Register) -> Self {
        DictionaryEntry { before, operation, after }
    }

    fn test_entry(&self) -> Vec<fn(&Operation, Register) -> Register> {
        self.operation.test_all_operations(self.before, self.after)
    }
}

#[derive(Debug, Copy, Clone)]
struct Operation {
    op_code: u8,
    val_1: u16,
    val_2: u16,
    val_3: u16,
}

impl Operation {
    fn new(
        op_code: u8,
        val_1: u16,
        val_2: u16,
        val_3: u16,
    ) -> Self {
        Operation {
            op_code,
            val_1,
            val_2,
            val_3,
        }
    }

    fn addr(&self, before: Register) -> Register {
        let mut new = before.clone();
        new[self.val_3] = new[self.val_1] + new[self.val_2];
        new
    }

    fn addi(&self, before: Register) -> Register {
        let mut new = before.clone();
        new[self.val_3] = new[self.val_1] + self.val_2;
        new
    }

    fn mulr(&self, before: Register) -> Register {
        let mut new = before.clone();
        new[self.val_3] = new[self.val_1] * new[self.val_2];
        new
    }

    fn muli(&self, before: Register) -> Register {
        let mut new = before.clone();
        new[self.val_3] = new[self.val_1] * self.val_2;
        new
    }

    fn banr(&self, before: Register) -> Register {
        let mut new = before.clone();
        new[self.val_3] = new[self.val_1] & new[self.val_2];
        new
    }

    fn bani(&self, before: Register) -> Register {
        let mut new = before.clone();
        new[self.val_3] = new[self.val_1] & self.val_2;
        new
    }

    fn borr(&self, before: Register) -> Register {
        let mut new = before.clone();
        new[self.val_3] = new[self.val_1] | new[self.val_2];
        new
    }

    fn bori(&self, before: Register) -> Register {
        let mut new = before.clone();
        new[self.val_3] = new[self.val_1] | self.val_2;
        new
    }

    fn setr(&self, before: Register) -> Register {
        let mut new = before.clone();
        new[self.val_3] = new[self.val_1];
        new
    }

    fn seti(&self, before: Register) -> Register {
        let mut new = before.clone();
        new[self.val_3] = self.val_1;
        new
    }

    fn gtir(&self, before: Register) -> Register {
        let mut new = before.clone();
        new[self.val_3] = if self.val_1 > new[self.val_2] {
            1
        } else {
            0
        };
        new
    }

    fn gtri(&self, before: Register) -> Register {
        let mut new = before.clone();
        new[self.val_3] = if new[self.val_1] > self.val_2 {
            1
        } else {
            0
        };
        new
    }

    fn gtrr(&self, before: Register) -> Register {
        let mut new = before.clone();
        new[self.val_3] = if new[self.val_1] > new[self.val_2] {
            1
        } else {
            0
        };
        new
    }

    fn eqir(&self, before: Register) -> Register {
        let mut new = before.clone();
        new[self.val_3] = if self.val_1 == new[self.val_2] {
            1
        } else {
            0
        };
        new
    }

    fn eqri(&self, before: Register) -> Register {
        let mut new = before.clone();
        new[self.val_3] = if self.val_2 == new[self.val_1] {
            1
        } else {
            0
        };
        new
    }

    fn eqrr(&self, before: Register) -> Register {
        let mut new = before.clone();
        new[self.val_3] = if new[self.val_1] == new[self.val_2] {
            1
        } else {
            0
        };
        new
    }

    fn test_all_operations(&self, before: Register, after: Register) -> Vec<fn(&Self, Register) -> Register> {
        let mut fns: Vec<fn(&Self, Register) -> Register> = vec![
            Operation::addr, Operation::addi, Operation::mulr, Operation::muli,
            Operation::banr, Operation::bani, Operation::borr, Operation::bori,
            Operation::setr, Operation::seti, Operation::gtir, Operation::gtri,
            Operation::gtrr, Operation::eqir, Operation::eqri, Operation::eqrr];

        let mut returned = vec![];

        for function in fns.into_iter() {
            if self.test_operation(before, after, function) {
                returned.push(function);
            }
        }

        returned
    }

    fn test_operation(&self, before: Register, after: Register, operation: fn(&Self, Register) -> Register) -> bool {
        operation(self, before) == after
    }
}

// Don't look at this, it's bad
impl From<&str> for Operation {
    fn from(string: &str) -> Self {
        if string.len() == 7 {
            let op_code = string.get(0..1).unwrap().parse().unwrap();
            let val_1 = string.get(2..3).unwrap().parse().unwrap();
            let val_2 = string.get(4..5).unwrap().parse().unwrap();
            let val_3 = string.get(6..7).unwrap().parse().unwrap();

            Operation::new(op_code, val_1, val_2, val_3)
        } else {
            let op_code = string.get(0..2).unwrap().parse().unwrap();
            let val_1 = string.get(3..4).unwrap().parse().unwrap();
            let val_2 = string.get(5..6).unwrap().parse().unwrap();
            let val_3 = string.get(7..8).unwrap().parse().unwrap();

            Operation::new(op_code, val_1, val_2, val_3)
        }
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
struct Register {
    register_a: u16,
    register_b: u16,
    register_c: u16,
    register_d: u16,
}

impl Register {
    fn new(
        register_a: u16,
        register_b: u16,
        register_c: u16,
        register_d: u16,
    ) -> Self {
        Register {
            register_a,
            register_b,
            register_c,
            register_d,
        }
    }
}

// Very bad implementation, but I don't care
impl From<&str> for Register {
    fn from(string: &str) -> Self {
        let mut string = &*string.replace(",", "");
        if string.len() == 17 {
            string = &string[9..17];
        }
        let a = string.get(0..1).unwrap().parse().unwrap();
        let b = string.get(2..3).unwrap().parse().unwrap();
        let c = string.get(4..5).unwrap().parse().unwrap();
        let d = string.get(6..7).unwrap().parse().unwrap();
        Register::new(a, b, c, d)
    }
}

impl Index<u16> for Register {
    type Output = u16;

    fn index(&self, index: u16) -> &Self::Output {
        match index {
            0 => &self.register_a,
            1 => &self.register_b,
            2 => &self.register_c,
            3 => &self.register_d,
            _ => panic!("Index out of range")
        }
    }
}

impl IndexMut<u16> for Register {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        match index {
            0 => &mut self.register_a,
            1 => &mut self.register_b,
            2 => &mut self.register_c,
            3 => &mut self.register_d,
            _ => panic!("Index out of range")
        }
    }
}

