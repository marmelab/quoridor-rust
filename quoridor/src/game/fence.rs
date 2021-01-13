use serde::Serialize;
use crate::game::position::{Position};

#[derive(Debug, Serialize)]
pub struct Fence {
    n_w_square: Position,
    is_horizontal: bool,
}
