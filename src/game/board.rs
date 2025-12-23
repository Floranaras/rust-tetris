use crate::config::{BOARD_HEIGHT, BOARD_WIDTH};
use crate::tetromino::Tetromino;

#[derive(Clone, Copy, PartialEq)]
pub struct Block {
    pub filled: bool,
}

pub struct Board {
    grid: Vec<Vec<Option<Block>>>,
}

impl Board {
    pub fn new() -> Self {
        Board {
            grid: vec![vec![None; BOARD_WIDTH as usize]; BOARD_HEIGHT as usize],
        }
    }

    pub fn is_valid_position(&self, piece: &Tetromino) -> bool {
        for (x, y) in piece.get_blocks() {
            // Check boundaries
            if x < 0 || x >= BOARD_WIDTH || y >= BOARD_HEIGHT {
                return false;
            }

            // Check collision with placed blocks
            if y >= 0 && self.grid[y as usize][x as usize].is_some() {
                return false;
            }
        }
        true
    }

    pub fn place_piece(&mut self, piece: &Tetromino) {
        for (x, y) in piece.get_blocks() {
            if y >= 0 && y < BOARD_HEIGHT && x >= 0 && x < BOARD_WIDTH {
                self.grid[y as usize][x as usize] = Some(Block { filled: true });
            }
        }
    }

    pub fn clear_lines(&mut self) -> u32 {
        let mut lines_to_clear = Vec::new();

        for y in 0..BOARD_HEIGHT {
            if self.is_line_full(y as usize) {
                lines_to_clear.push(y as usize);
            }
        }

        for &line in lines_to_clear.iter().rev() {
            self.grid.remove(line);
            self.grid.insert(0, vec![None; BOARD_WIDTH as usize]);
        }

        lines_to_clear.len() as u32
    }

    fn is_line_full(&self, y: usize) -> bool {
        self.grid[y].iter().all(|block| block.is_some())
    }

    pub fn get_block(&self, x: i32, y: i32) -> Option<Block> {
        if x >= 0 && x < BOARD_WIDTH && y >= 0 && y < BOARD_HEIGHT {
            self.grid[y as usize][x as usize]
        } else {
            None
        }
    }

    pub fn has_block(&self, x: i32, y: i32) -> bool {
        self.get_block(x, y).is_some()
    }

    #[allow(dead_code)]
    pub fn grid(&self) -> &Vec<Vec<Option<Block>>> {
        &self.grid
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.grid = vec![vec![None; BOARD_WIDTH as usize]; BOARD_HEIGHT as usize];
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tetromino::types::TetrominoType;
    use crate::tetromino::piece::Tetromino;

    #[test]
    fn test_board_creation() {
        let board = Board::new();
        assert_eq!(board.grid.len(), BOARD_HEIGHT as usize);
        assert_eq!(board.grid[0].len(), BOARD_WIDTH as usize);
    }

    #[test]
    fn test_valid_position() {
        let board = Board::new();
        let piece = Tetromino::new(TetrominoType::I);
        assert!(board.is_valid_position(&piece));
    }

    #[test]
    fn test_place_piece() {
        let mut board = Board::new();
        let mut piece = Tetromino::new(TetrominoType::O);
        piece.y = BOARD_HEIGHT - 2;

        board.place_piece(&piece);
        assert!(board.has_block(piece.x, piece.y));
    }

    #[test]
    fn test_line_clearing() {
        let mut board = Board::new();
        
        for x in 0..BOARD_WIDTH {
            board.grid[(BOARD_HEIGHT - 1) as usize][x as usize] = Some(Block { filled: true });
        }

        let cleared = board.clear_lines();
        assert_eq!(cleared, 1);
        assert!(!board.has_block(0, BOARD_HEIGHT - 1));
    }
}
