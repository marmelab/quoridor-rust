mod board;
mod position;
mod fence;

use serde::Serialize;
use crate::error::{AppError};
use crate::game::position::{Position, Direction};
use crate::game::fence::{Fence};
use crate::game::board::{Board};


#[derive(Debug, Serialize)]
pub struct Pawn {
    position: Position,
    goal: Direction,
}

#[derive(Debug, Serialize)]
pub struct Game {
    id: String,
    over: bool,
    pawn_turn: u8,
    pawns: Vec<Pawn>,
    fences: Vec<Fence>,
    board: Board,
}

impl Game {
    pub fn new(board_size: u8) -> Result<Game, AppError> {
        let id = "anUID".to_string();
        let over = false;
        let pawn_turn = 0;
        let mut pawns = Vec::new();
        let fences = Vec::new();
        let board_result = Board::new(board_size);

        let board = match board_result {
            Ok(board) => board,
            Err(e) => return Err(e),
        };

        let line_center: u8 = (board_size - 1) / 2;

        pawns.push(Pawn {
            position: Position {
                column: 0,
                row: line_center,
            },
            goal: Direction::EAST,
        });
        pawns.push(Pawn {
            position: Position {
                column: (board_size - 1),
                row: line_center,
            },
            goal: Direction::WEST,
        });
        Ok(Game {
            id,
            over,
            pawn_turn,
            pawns,
            fences,
            board,
        })
    }
}
