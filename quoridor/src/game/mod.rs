use serde::{Serialize};
use crate::error::{AppError, AppErrorType};

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
    pub fn new(size: u8) -> Result<Board, AppError> {
        if size < 3 {
            return Err(AppError{
                message: Some("The board size must be at least 3.".to_string()), 
                cause: None, 
                error_type: AppErrorType::IllegalArgumentError});
        }
        if size % 2 == 0 {
            return Err(AppError{
                message: Some("The board size must be an odd number.".to_string()), 
                cause: None, 
                error_type: AppErrorType::IllegalArgumentError});
        }
        let mut squares = Vec::new();
        for row in 0..size {
            for column in 0..size {
                squares.push(Position{column, row});
            }
        }
        Ok(Board { size, squares })
    }

}


pub const NORTH: u8 = 1;
pub const EAST: u8 = 2;
pub const SOUTH: u8 = 3;
pub const WEST: u8 = 4;
pub const UNKNOWN: u8 = 0;

#[derive(Debug, Serialize)]
pub struct Pawn {
    position: Position,
    goal: u8,
}

#[derive(Debug, Serialize)]
pub struct Fence {
    n_w_square: Position,
    is_horizontal: bool,
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
    pub fn new(board_size: u8) -> Game {
        let id = "titi".to_string();
        let mut over = false;
        let mut pawn_turn = 0;
        let mut pawns = Vec::new();
        let mut fences = Vec::new();
        let mut board = Board::new(board_size);

        let line_center: u8 = (board_size - 1) / 2;

        pawns.push(Pawn{position:Position{column:0, row:line_center}, goal:EAST});
        pawns.push(Pawn{position:Position{column:(board_size - 1), row:line_center}, goal:WEST});
        Game {  id, over, pawn_turn, pawns, fences, board }
    }

}
