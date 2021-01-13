use serde::{Deserialize, Serialize};
use crate::game::position::{Position};

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Fence {
    pub n_w_square: Position,
    pub is_horizontal: bool,
}

pub struct PositionSquare {
	north_position: Position,
	east_position: Position,
	south_positio: Position,
	west_position: Position,
}

pub fn new_position_square(center: Position) -> PositionSquare {
	let north_position = center.translate(0, -1);
	let east_position = center.translate(1, 0);
	let south_positio = center.translate(0, 1);
	let west_position = center.translate(-1, 0);
	PositionSquare{north_position, east_position, south_positio, west_position}
}
