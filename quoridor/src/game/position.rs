use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub struct Position {
    pub column: u8,
    pub row: u8,
}

impl Position {
	pub fn translate_column(&self, delta: i8) -> Position {
		let mut position = self.clone();
		position.column = (position.column as i8).wrapping_add(delta) as u8;
		position
	}

	pub fn translate_row(&self, delta: i8) -> Position {
		let mut position = self.clone();
		position.row = (position.row as i8).wrapping_add(delta) as u8;
		position
	}

	pub fn translate(&self, delta_column: i8, delta_row: i8) -> Position {
		let mut position = self.clone();
		position.column = (position.column as i8).wrapping_add(delta_column) as u8;
		position.row = (position.row as i8).wrapping_add(delta_row) as u8;
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
