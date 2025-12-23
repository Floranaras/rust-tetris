use crate::game::GameState;
use macroquad::prelude::*;

pub struct InputHandler;

impl InputHandler {
    pub fn handle_input(state: &mut GameState) {
        if state.game_over {
            Self::handle_game_over_input(state);
            return;
        }

        Self::handle_gameplay_input(state);
    }

    fn handle_gameplay_input(state: &mut GameState) {
        if Self::any_key_pressed() {
            state.hide_help();
        }

        if is_key_pressed(KeyCode::A) || is_key_pressed(KeyCode::Left) {
            state.try_move(-1, 0);
        }

        if is_key_pressed(KeyCode::D) || is_key_pressed(KeyCode::Right) {
            state.try_move(1, 0);
        }

        if is_key_pressed(KeyCode::S) || is_key_pressed(KeyCode::Down) {
            state.try_move(0, 1);
        }

        if is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Up) {
            state.try_rotate();
        }

        if is_key_pressed(KeyCode::Space) {
            state.hard_drop();
        }

        if is_key_pressed(KeyCode::H) {
            state.toggle_help();
        }
    }

    fn handle_game_over_input(state: &mut GameState) {
        if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) {
            state.reset();
        }
    }

    fn any_key_pressed() -> bool {
        is_key_pressed(KeyCode::A)
            || is_key_pressed(KeyCode::Left)
            || is_key_pressed(KeyCode::D)
            || is_key_pressed(KeyCode::Right)
            || is_key_pressed(KeyCode::S)
            || is_key_pressed(KeyCode::Down)
            || is_key_pressed(KeyCode::W)
            || is_key_pressed(KeyCode::Up)
            || is_key_pressed(KeyCode::Space)
    }
}
