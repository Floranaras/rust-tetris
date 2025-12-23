use super::board::Board;
use super::scoring::Scoring;
use crate::tetromino::{PieceBag, Tetromino};

pub struct GameState {
    pub board: Board,
    pub current_piece: Tetromino,
    pub next_piece: Tetromino,
    pub scoring: Scoring,
    pub game_over: bool,
    pub show_help: bool,
    piece_bag: PieceBag,
    drop_timer: f32,
}

impl GameState {
    pub fn new() -> Self {
        let mut piece_bag = PieceBag::new();
        let current_piece_type = piece_bag.next_piece();
        let next_piece_type = piece_bag.next_piece();

        GameState {
            board: Board::new(),
            current_piece: Tetromino::new(current_piece_type),
            next_piece: Tetromino::new(next_piece_type),
            scoring: Scoring::new(),
            game_over: false,
            show_help: true,
            piece_bag,
            drop_timer: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.game_over {
            return;
        }

        self.drop_timer += dt;
        let drop_speed = self.scoring.get_drop_speed();

        if self.drop_timer >= drop_speed {
            if !self.try_move(0, 1) {
                self.lock_piece();
            }
            self.drop_timer = 0.0;
        }
    }

    pub fn try_move(&mut self, dx: i32, dy: i32) -> bool {
        let mut new_piece = self.current_piece.clone();
        new_piece.move_by(dx, dy);

        if self.board.is_valid_position(&new_piece) {
            self.current_piece = new_piece;
            true
        } else {
            false
        }
    }

    pub fn try_rotate(&mut self) -> bool {
        let mut new_piece = self.current_piece.clone();
        new_piece.rotate();

        if self.board.is_valid_position(&new_piece) {
            self.current_piece = new_piece;
            true
        } else {
            false
        }
    }

    pub fn hard_drop(&mut self) {
        while self.try_move(0, 1) {
            // Keep dropping
        }
        self.lock_piece();
    }

    fn lock_piece(&mut self) {
        self.board.place_piece(&self.current_piece);

        if self.current_piece.y <= 0 {
            self.game_over = true;
            return;
        }

        let lines_cleared = self.board.clear_lines();
        self.scoring.add_lines(lines_cleared);

        self.spawn_next_piece();
    }

    fn spawn_next_piece(&mut self) {
        self.current_piece = self.next_piece.clone();
        self.current_piece.reset_position();

        let next_type = self.piece_bag.next_piece();
        self.next_piece = Tetromino::new(next_type);
    }

    pub fn get_ghost_piece(&self) -> Tetromino {
        let mut ghost = self.current_piece.clone();
        while self.board.is_valid_position(&ghost) {
            ghost.y += 1;
        }
        ghost.y -= 1;
        ghost
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    pub fn hide_help(&mut self) {
        self.show_help = false;
    }

    pub fn reset(&mut self) {
        *self = GameState::new();
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_state_creation() {
        let state = GameState::new();
        assert!(!state.game_over);
        assert_eq!(state.scoring.level, 1);
    }

    #[test]
    fn test_piece_movement() {
        let mut state = GameState::new();
        let initial_x = state.current_piece.x;
        
        assert!(state.try_move(1, 0));
        assert_eq!(state.current_piece.x, initial_x + 1);
    }

    #[test]
    fn test_piece_rotation() {
        let mut state = GameState::new();
        let initial_shape = state.current_piece.shape.clone();
        
        state.try_rotate();
        assert_ne!(state.current_piece.shape, initial_shape);
    }

    #[test]
    fn test_reset() {
        let mut state = GameState::new();
        state.game_over = true;
        state.scoring.score = 1000;
        
        state.reset();
        assert!(!state.game_over);
        assert_eq!(state.scoring.score, 0);
    }
}
