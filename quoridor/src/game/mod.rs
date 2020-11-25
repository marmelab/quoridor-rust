use serde::{Serialize};

#[derive(Debug, Serialize)]
pub struct Position {
    column: u8,
    row: u8,
}

#[derive(Debug, Serialize)]
pub struct Board {
    size: u8,
	squares: Vec<Position>
}

impl Board {
    pub fn new(size: u8) -> Board {
        if size < 3 {
            panic!("The board size must be at least 3.");
        }
        if size % 2 == 0 {
            panic!("The board size must be an odd number.");
        }
        let mut squares = Vec::new();
        for row in 0..size {
            for column in 0..size {
                squares.push(Position{column, row});
            }
        }
        Board { size, squares }
    }

}
