use crate::config::*;
use crate::game::GameState;
use macroquad::prelude::*;

pub struct Renderer {
    blink_timer: f32,
}

impl Renderer {
    pub fn new() -> Self {
        Renderer { blink_timer: 0.0 }
    }

    pub fn update(&mut self, dt: f32) {
        self.blink_timer += dt;
    }

    pub fn draw(&self, state: &GameState) {
        clear_background(GB_LIGHT);

        self.draw_gb_border();
        self.draw_board(state);
        self.draw_ghost_piece(state);
        self.draw_current_piece(state);
        self.draw_ui(state);

        if state.show_help {
            self.draw_help();
        }

        if state.game_over {
            self.draw_game_over();
        }
    }

    fn draw_gb_border(&self) {
        let border_thickness = 8.0;
        let board_width = BOARD_WIDTH as f32 * BLOCK_SIZE;
        let board_height = BOARD_HEIGHT as f32 * BLOCK_SIZE;

        draw_rectangle(
            BOARD_OFFSET_X - border_thickness,
            BOARD_OFFSET_Y - border_thickness,
            board_width + border_thickness * 2.0,
            board_height + border_thickness * 2.0,
            GB_DARK,
        );

        draw_rectangle(
            BOARD_OFFSET_X - border_thickness + 2.0,
            BOARD_OFFSET_Y - border_thickness + 2.0,
            board_width + (border_thickness - 4.0) * 2.0,
            board_height + (border_thickness - 4.0) * 2.0,
            GB_MED_DARK,
        );

        draw_rectangle(
            BOARD_OFFSET_X - 2.0,
            BOARD_OFFSET_Y - 2.0,
            board_width + 4.0,
            board_height + 4.0,
            GB_MED_LIGHT,
        );

        draw_rectangle(
            BOARD_OFFSET_X,
            BOARD_OFFSET_Y,
            board_width,
            board_height,
            GB_LIGHT,
        );
    }

    fn draw_board(&self, state: &GameState) {
        for x in 0..=BOARD_WIDTH {
            let px = BOARD_OFFSET_X + x as f32 * BLOCK_SIZE;
            draw_line(
                px,
                BOARD_OFFSET_Y,
                px,
                BOARD_OFFSET_Y + BOARD_HEIGHT as f32 * BLOCK_SIZE,
                1.0,
                GB_MED_LIGHT,
            );
        }

        for y in 0..=BOARD_HEIGHT {
            let py = BOARD_OFFSET_Y + y as f32 * BLOCK_SIZE;
            draw_line(
                BOARD_OFFSET_X,
                py,
                BOARD_OFFSET_X + BOARD_WIDTH as f32 * BLOCK_SIZE,
                py,
                1.0,
                GB_MED_LIGHT,
            );
        }

        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                if state.board.has_block(x, y) {
                    self.draw_gb_block(x as f32, y as f32);
                }
            }
        }
    }

    fn draw_ghost_piece(&self, state: &GameState) {
        let ghost = state.get_ghost_piece();
        for (x, y) in ghost.get_blocks() {
            if y >= 0 {
                self.draw_ghost_block(x as f32, y as f32);
            }
        }
    }

    fn draw_current_piece(&self, state: &GameState) {
        for (x, y) in state.current_piece.get_blocks() {
            if y >= 0 {
                self.draw_gb_block(x as f32, y as f32);
            }
        }
    }

    fn draw_gb_block(&self, x: f32, y: f32) {
        let px = BOARD_OFFSET_X + x * BLOCK_SIZE + 1.0;
        let py = BOARD_OFFSET_Y + y * BLOCK_SIZE + 1.0;
        let size = BLOCK_SIZE - 2.0;

        draw_rectangle(px, py, size, size, GB_DARK);
        draw_rectangle(px + 1.0, py + 1.0, size - 6.0, size - 6.0, GB_MED_DARK);
        draw_rectangle(px + 3.0, py + 3.0, size - 10.0, size - 10.0, GB_MED_LIGHT);
    }

    fn draw_ghost_block(&self, x: f32, y: f32) {
        let px = BOARD_OFFSET_X + x * BLOCK_SIZE + 1.0;
        let py = BOARD_OFFSET_Y + y * BLOCK_SIZE + 1.0;
        let size = BLOCK_SIZE - 2.0;

        draw_rectangle_lines(px, py, size, size, 1.0, GB_MED_DARK);
        draw_rectangle_lines(px + 2.0, py + 2.0, size - 4.0, size - 4.0, 1.0, GB_MED_DARK);
    }

    fn draw_ui(&self, state: &GameState) {
        let ui_x = BOARD_OFFSET_X + BOARD_WIDTH as f32 * BLOCK_SIZE + 30.0;
        let mut ui_y = BOARD_OFFSET_Y;

        draw_text("TETRIS", ui_x, ui_y, 20.0, GB_DARK);
        ui_y += 35.0;

        self.draw_next_piece_box(state, ui_x, ui_y);
        ui_y += 100.0;

        draw_text("SCORE", ui_x, ui_y, 12.0, GB_DARK);
        ui_y += 15.0;
        draw_text(&format!("{:06}", state.scoring.score), ui_x, ui_y, 14.0, GB_DARK);
        ui_y += 25.0;

        draw_text("LINES", ui_x, ui_y, 12.0, GB_DARK);
        ui_y += 15.0;
        draw_text(
            &format!("{:03}", state.scoring.lines_cleared),
            ui_x,
            ui_y,
            14.0,
            GB_DARK,
        );
        ui_y += 25.0;

        draw_text("LEVEL", ui_x, ui_y, 12.0, GB_DARK);
        ui_y += 15.0;
        draw_text(&format!("{:02}", state.scoring.level), ui_x, ui_y, 14.0, GB_DARK);
        ui_y += 35.0;

        if (self.blink_timer * 2.0) as i32 % 2 == 0 {
            draw_text("PRESS H", ui_x, ui_y, 10.0, GB_MED_DARK);
            ui_y += 12.0;
            draw_text("FOR HELP", ui_x, ui_y, 10.0, GB_MED_DARK);
        }
    }

    fn draw_next_piece_box(&self, state: &GameState, x: f32, y: f32) {
        let box_size = 80.0;

        draw_rectangle(x - 4.0, y - 4.0, box_size + 8.0, box_size + 8.0, GB_DARK);
        draw_rectangle(x - 2.0, y - 2.0, box_size + 4.0, box_size + 4.0, GB_MED_DARK);
        draw_rectangle(x, y, box_size, box_size, GB_LIGHT);

        draw_text("NEXT", x + 5.0, y - 8.0, 12.0, GB_DARK);

        let offset_x = x + 20.0;
        let offset_y = y + 20.0;

        for (row, line) in state.next_piece.shape.iter().enumerate() {
            for (col, &filled) in line.iter().enumerate() {
                if filled {
                    let px = offset_x + col as f32 * 16.0;
                    let py = offset_y + row as f32 * 16.0;
                    draw_rectangle(px, py, 14.0, 14.0, GB_DARK);
                    draw_rectangle(px + 1.0, py + 1.0, 8.0, 8.0, GB_MED_DARK);
                    draw_rectangle(px + 2.0, py + 2.0, 6.0, 6.0, GB_MED_LIGHT);
                }
            }
        }
    }

    fn draw_help(&self) {
        let help_x = BOARD_OFFSET_X - 20.0;
        let help_y = BOARD_OFFSET_Y + BOARD_HEIGHT as f32 * BLOCK_SIZE + 20.0;
        let help_width = BOARD_WIDTH as f32 * BLOCK_SIZE + 40.0;
        let help_height = 140.0;

        draw_rectangle(
            help_x - 4.0,
            help_y - 4.0,
            help_width + 8.0,
            help_height + 8.0,
            GB_DARK,
        );
        draw_rectangle(
            help_x - 2.0,
            help_y - 2.0,
            help_width + 4.0,
            help_height + 4.0,
            GB_MED_DARK,
        );
        draw_rectangle(help_x, help_y, help_width, help_height, GB_LIGHT);

        let mut text_y = help_y + 15.0;
        draw_text("CONTROLS:", help_x + 10.0, text_y, 12.0, GB_DARK);
        text_y += 20.0;

        draw_text("A/← D/→  Move left/right", help_x + 10.0, text_y, 10.0, GB_DARK);
        text_y += 15.0;
        draw_text("W/↑      Rotate piece", help_x + 10.0, text_y, 10.0, GB_DARK);
        text_y += 15.0;
        draw_text("S/↓      Soft drop", help_x + 10.0, text_y, 10.0, GB_DARK);
        text_y += 15.0;
        draw_text("SPACE    Hard drop", help_x + 10.0, text_y, 10.0, GB_DARK);
        text_y += 15.0;
        draw_text("H        Toggle this help", help_x + 10.0, text_y, 10.0, GB_DARK);

        if (self.blink_timer * 3.0) as i32 % 2 == 0 {
            draw_text(
                "Press H to close",
                help_x + help_width - 120.0,
                help_y + help_height - 15.0,
                10.0,
                GB_MED_DARK,
            );
        }
    }

    fn draw_game_over(&self) {
        let overlay_color = Color::new(GB_DARK.r, GB_DARK.g, GB_DARK.b, 0.8);
        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), overlay_color);

        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;

        let box_width = 200.0;
        let box_height = 100.0;
        let box_x = center_x - box_width / 2.0;
        let box_y = center_y - box_height / 2.0;

        draw_rectangle(
            box_x - 4.0,
            box_y - 4.0,
            box_width + 8.0,
            box_height + 8.0,
            GB_DARK,
        );
        draw_rectangle(
            box_x - 2.0,
            box_y - 2.0,
            box_width + 4.0,
            box_height + 4.0,
            GB_MED_DARK,
        );
        draw_rectangle(box_x, box_y, box_width, box_height, GB_LIGHT);

        draw_text("GAME OVER", box_x + 20.0, box_y + 30.0, 16.0, GB_DARK);

        if (self.blink_timer * 2.0) as i32 % 2 == 0 {
            draw_text(
                "SPACE TO RESTART",
                box_x + 10.0,
                box_y + 55.0,
                12.0,
                GB_MED_DARK,
            );
        }
    }
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}
