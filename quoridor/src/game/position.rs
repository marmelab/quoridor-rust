use serde::{Serialize};

#[derive(Debug, Serialize, Copy, Clone)]
pub struct Position {
    pub column: u8,
    pub row: u8,
}

impl Position {
	pub fn translate_column(&mut self, delta: u8) -> Position {
		let mut position = self.clone();
		position.column += delta; 
		position
	}

	pub fn translate_row(&mut self, delta: u8) -> Position {
		let mut position = self.clone();
		position.row += delta; 
		position
	}

	pub fn translate(&mut self, delta_column: u8, delta_row: u8) -> Position {
		let mut position = self.clone();
		position.column += delta_column; 
		position.row += delta_row; 
		position
	}
}

#[derive(Debug, Serialize, Copy, Clone)]
pub enum Direction {
	NORTH,
	EAST,
	SOUTH,
	WEST,
	UNKNOWN

}

pub fn get_direction(from: Position, to: Position) -> Direction {
	if from.row == to.row {
		if from.column+1 == to.column {
			return Direction::EAST;
		}
		if from.column-1 == to.column {
			return Direction::WEST;
		}
	}
	if from.column == to.column {
		if from.row-1 == to.row {
			return Direction::NORTH;
		}
		if from.row+1 == to.row {
			return Direction::SOUTH;
		}
	}
	return  Direction::UNKNOWN;
}
