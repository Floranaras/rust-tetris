use super::types::TetrominoType;
use crate::config::BOARD_WIDTH;

#[derive(Clone)]
pub struct Tetromino {
    pub shape: Vec<Vec<bool>>,
    #[allow(dead_code)]
    pub piece_type: TetrominoType,
    pub x: i32,
    pub y: i32,
}

impl Tetromino {
    pub fn new(piece_type: TetrominoType) -> Self {
        Tetromino {
            shape: piece_type.get_shape(),
            piece_type,
            x: BOARD_WIDTH / 2 - 2,
            y: 0,
        }
    }

    pub fn rotate(&mut self) {
        let rows = self.shape.len();
        let cols = self.shape[0].len();
        let mut rotated = vec![vec![false; rows]; cols];

        for i in 0..rows {
            for j in 0..cols {
                rotated[j][rows - 1 - i] = self.shape[i][j];
            }
        }

        self.shape = rotated;
    }

    pub fn get_blocks(&self) -> Vec<(i32, i32)> {
        let mut blocks = Vec::new();
        for (row, line) in self.shape.iter().enumerate() {
            for (col, &filled) in line.iter().enumerate() {
                if filled {
                    blocks.push((self.x + col as i32, self.y + row as i32));
                }
            }
        }
        blocks
    }

    pub fn move_by(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    pub fn reset_position(&mut self) {
        self.x = BOARD_WIDTH / 2 - 2;
        self.y = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tetromino_creation() {
        let piece = Tetromino::new(TetrominoType::I);
        assert_eq!(piece.piece_type, TetrominoType::I);
        assert!(!piece.shape.is_empty());
    }

    #[test]
    fn test_tetromino_movement() {
        let mut piece = Tetromino::new(TetrominoType::T);
        let initial_x = piece.x;
        let initial_y = piece.y;

        piece.move_by(1, 2);
        assert_eq!(piece.x, initial_x + 1);
        assert_eq!(piece.y, initial_y + 2);
    }

    #[test]
    fn test_tetromino_rotation() {
        let mut piece = Tetromino::new(TetrominoType::I);
        let initial_shape = piece.shape.clone();

        piece.rotate();
        assert_ne!(piece.shape, initial_shape);

        piece.rotate();
        piece.rotate();
        piece.rotate();
        assert_eq!(piece.shape.len(), initial_shape.len());
    }

    #[test]
    fn test_get_blocks() {
        let piece = Tetromino::new(TetrominoType::O);
        let blocks = piece.get_blocks();

        assert_eq!(blocks.len(), 4);
    }
}
