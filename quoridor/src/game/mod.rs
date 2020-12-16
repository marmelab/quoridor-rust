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
