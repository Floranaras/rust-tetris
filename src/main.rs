use macroquad::prelude::*;
use macroquad::rand::srand;

mod config;
mod game;
mod input;
mod renderer;
mod tetromino;

use config::*;
use game::GameState;
use input::InputHandler;
use renderer::Renderer;

fn window_conf() -> Conf {
    Conf {
        window_title: WINDOW_TITLE.to_owned(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    srand(macroquad::miniquad::date::now() as u64);

    let mut game_state = GameState::new();
    let mut renderer = Renderer::new();

    loop {
        let dt = get_frame_time();

        InputHandler::handle_input(&mut game_state);

        game_state.update(dt);

        renderer.update(dt);

        renderer.draw(&game_state);

        next_frame().await;
    }
}
