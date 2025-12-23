#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TetrominoType {
    I = 0,
    O = 1,
    T = 2,
    S = 3,
    Z = 4,
    J = 5,
    L = 6,
}

impl TetrominoType {
    pub fn all() -> [TetrominoType; 7] {
        [
            TetrominoType::I,
            TetrominoType::O,
            TetrominoType::T,
            TetrominoType::S,
            TetrominoType::Z,
            TetrominoType::J,
            TetrominoType::L,
        ]
    }

    pub fn get_shape(&self) -> Vec<Vec<bool>> {
        match self {
            TetrominoType::I => vec![vec![true, true, true, true]],
            TetrominoType::O => vec![vec![true, true], vec![true, true]],
            TetrominoType::T => vec![vec![false, true, false], vec![true, true, true]],
            TetrominoType::S => vec![vec![false, true, true], vec![true, true, false]],
            TetrominoType::Z => vec![vec![true, true, false], vec![false, true, true]],
            TetrominoType::J => vec![vec![true, false, false], vec![true, true, true]],
            TetrominoType::L => vec![vec![false, false, true], vec![true, true, true]],
        }
    }
}
