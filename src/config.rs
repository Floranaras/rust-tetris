use macroquad::prelude::Color;

pub const BOARD_WIDTH: i32 = 10;
pub const BOARD_HEIGHT: i32 = 20;

pub const BLOCK_SIZE: f32 = 24.0;
pub const BOARD_OFFSET_X: f32 = 60.0;
pub const BOARD_OFFSET_Y: f32 = 40.0;

pub const GB_DARK: Color = Color::new(0.06, 0.22, 0.06, 1.0);
pub const GB_MED_DARK: Color = Color::new(0.19, 0.38, 0.19, 1.0);
pub const GB_MED_LIGHT: Color = Color::new(0.55, 0.67, 0.06, 1.0);
pub const GB_LIGHT: Color = Color::new(0.68, 0.89, 0.18, 1.0);

pub const INITIAL_DROP_SPEED: f32 = 0.8;
pub const MIN_DROP_SPEED: f32 = 0.1;
pub const SPEED_INCREASE_PER_LEVEL: f32 = 0.07;
pub const LINES_PER_LEVEL: u32 = 10;

pub const SCORE_SINGLE: u32 = 40;
pub const SCORE_DOUBLE: u32 = 100;
pub const SCORE_TRIPLE: u32 = 300;
pub const SCORE_TETRIS: u32 = 1200;

pub const WINDOW_TITLE: &str = "Game Boy Tetris";
pub const WINDOW_WIDTH: i32 = 480;
pub const WINDOW_HEIGHT: i32 = 640;
