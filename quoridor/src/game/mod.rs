mod board;
mod position;
pub mod fence;

use serde::Serialize;
use crate::error::{AppError, AppErrorType};
use crate::game::position::{Position, Direction};
use crate::game::fence::{Fence, PositionSquare, new_position_square};
use crate::game::board::{Board};


#[derive(Debug, Serialize, Clone)]
pub struct Pawn {
    position: Position,
    goal: Direction,
}

#[derive(Debug, Serialize, Clone)]
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

    pub fn get_next_pawn_turn(&mut self) -> u8 {
        let next = self.pawn_turn+1;
        if (next) as usize > self.pawns.len() {
            return 1
        }
        return next
    }

    pub fn add_fence(&mut self, fence: Fence) -> Result<Game, AppError> {
        if self.over {
            return Err(AppError {
                message: Some("Game is over, unable to add a fence.".to_string()),
                cause: None,
                error_type: AppErrorType::PlayError,
            });
        }
        let position_square = new_position_square(fence.n_w_square);
        if self.has_already_a_fence_at_the_same_position(fence.n_w_square) || self.has_neighbour_fence(fence.is_horizontal, position_square) {
            return Err(AppError {
                message: Some("The fence overlaps another on.".to_string()),
                cause: None,
                error_type: AppErrorType::PlayError,
            });
        }
        self.add_fence_if_crossable(fence);
        self.pawn_turn = self.get_next_pawn_turn();
        return Ok(self.clone());
    }

    pub fn add_fence_if_crossable(&mut self, fence: Fence) -> Result<Game, AppError> {
        if !self.is_crossable(fence) {
            return Err(AppError {
                message: Some("No more access to goal line.".to_string()),
                cause: None,
                error_type: AppErrorType::PlayError,
            });
        }
        self.fences.push(fence);
        return Ok(self.clone());
    }
    
    pub fn has_already_a_fence_at_the_same_position(&mut self, p: Position) -> bool {
        return false
    }
    
    pub fn has_neighbour_fence(&mut self, is_horizontal: bool, ps: PositionSquare) -> bool {
        return false
    }

    pub fn is_crossable(&mut self, fence: Fence) -> bool {
        return true
    }

}
