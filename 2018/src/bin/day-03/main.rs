use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::io::BufRead;

fn main() {
    let reader = BufReader::new(
        File::open(Path::new("../input/2018/03/input.txt")).unwrap());

    let mut squares = Vec::new();
    reader.lines()
        .map(|line| line.unwrap())
        .map(|string| Square::from_string(string))
        .for_each(|square| squares.push(square));

    let (x_max, y_max) = Square::find_dimensions(&squares);
    let mut board = Vec::with_capacity(x_max);
    for i in 0..x_max {
        board.insert(i, Vec::<usize>::with_capacity(y_max));
        for j in 0..y_max {
            board[i].insert(j, 0);
        }
    }

    squares.iter().for_each(|square| square.insert_into_board(&mut board));

    let collected = board.iter()
        .map(|row| row.iter().filter(|&&value| value > 1).count())
        .fold(0, |left, right| left + right);

    let overlapped = squares.iter()
        .find(|&square| !square.overlaps(&board));
    
    println!("{}", collected);
    println!("{:?}", overlapped)
}

#[derive(Debug, PartialEq)]
struct Square {
    id: String,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl Square {
    fn new(id: String, x: usize, y: usize, width: usize, height: usize) -> Square {
        Square { id, x, y, width, height }
    }

    fn find_dimensions(vec: &Vec<Self>) -> (usize, usize) {
        let (mut max_x, mut max_y) = (0usize, 0usize);
        for square in vec {
            if (square.x + square.width) as usize > max_x {
                max_x = (square.x + square.width) as usize
            }
            if (square.y + square.height) as usize > max_y {
                max_y = (square.y + square.height) as usize
            }
        }

        (max_x, max_y)
    }

    fn insert_into_board(&self, board: &mut Vec<Vec<usize>>) {
        for i in self.x..self.x + self.width {
            for j in self.y..self.y + self.height {
                board[i][j] += 1;
            }
        }
    }

    fn overlaps(&self, board: &Vec<Vec<usize>>) -> bool {
        for i in self.x..self.x + self.width {
            for j in self.y..self.y + self.height {
                if board[i][j] != 1 { return true }
            }
        }
        false
    }

    fn from_string(string: String) -> Square {
        let split: Vec<&str> = string.split(" ").collect();
        let offset = split[2];
        let (offset, _) = offset.split_at(offset.len() - 1);
        let offset: Vec<&str> = offset.split(",").collect();
        let (x, y) = (offset[0].to_string().parse().unwrap(), offset[1].to_string().parse().unwrap());
        let dimension: Vec<&str> = split[3].split("x").collect();
        let (width, height) = (dimension[0].to_string().parse().unwrap(), dimension[1].to_string().parse().unwrap());
        Square::new(split[0].to_string(), x, y, width, height)
    }
}

#[cfg(test)]
mod tests {
    use crate::Square;

    #[test]
    fn test_square_from_string() {
        assert_eq!(Square::from_string("#1 @ 167,777: 23x12".to_string()),
                   Square::new("#1".to_string(), 167, 777, 23, 12));
    }

    #[test]
    fn test_find_dimension() {
        let vec = vec![Square::new("".to_string(), 0, 0, 1, 2),
                       Square::new("".to_string(),0, 0, 500, 2),
                       Square::new("".to_string(),1, 0, 500, 2),
                       Square::new("".to_string(),0, 0, 2, 500),
                       Square::new("".to_string(),0, 1, 2, 500)];

        assert_eq!((501, 501), Square::find_dimensions(&vec))
    }
}
