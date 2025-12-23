pub mod config;
pub mod game;
pub mod input;
pub mod renderer;
pub mod tetromino;

pub use game::board::Board;
pub use game::scoring::Scoring;
pub use game::state::GameState;
pub use tetromino::bag::PieceBag;
pub use tetromino::piece::Tetromino;
pub use tetromino::types::TetrominoType;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_creation() {
        let game = GameState::new();
        assert!(!game.game_over);
    }

    #[test]
    fn test_board_creation() {
        let board = Board::new();
        assert!(!board.has_block(0, 0));
    }

    #[test]
    fn test_tetromino_types() {
        let types = TetrominoType::all();
        assert_eq!(types.len(), 7);
    }
}
