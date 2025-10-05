use macroquad::prelude::*;
use macroquad::rand::{gen_range, srand};

// Board dimensions
const BOARD_WIDTH: i32 = 10;
const BOARD_HEIGHT: i32 = 20;

// Display settings
const BLOCK_SIZE: f32 = 24.0;
const BOARD_OFFSET_X: f32 = 60.0;
const BOARD_OFFSET_Y: f32 = 40.0;

// Game Boy color palette
const GB_DARK: Color = Color::new(0.06, 0.22, 0.06, 1.0);      // Dark green
const GB_MED_DARK: Color = Color::new(0.19, 0.38, 0.19, 1.0);  // Medium dark green
const GB_MED_LIGHT: Color = Color::new(0.55, 0.67, 0.06, 1.0); // Medium light green
const GB_LIGHT: Color = Color::new(0.68, 0.89, 0.18, 1.0);     // Light green

// Game settings
const INITIAL_DROP_SPEED: f32 = 0.8;
const MIN_DROP_SPEED: f32 = 0.1;
const SPEED_INCREASE_PER_LEVEL: f32 = 0.07;
const LINES_PER_LEVEL: u32 = 10;

// Scoring
const SCORE_SINGLE: u32 = 40;
const SCORE_DOUBLE: u32 = 100;
const SCORE_TRIPLE: u32 = 300;
const SCORE_TETRIS: u32 = 1200;

#[derive(Clone, Copy, PartialEq)]
struct Block {
    filled: bool,
}

#[derive(Clone, Copy, Debug)]
enum TetrominoType {
    I = 0,
    O = 1,
    T = 2,
    S = 3,
    Z = 4,
    J = 5,
    L = 6,
}

impl TetrominoType {
    fn get_shape(&self) -> Vec<Vec<bool>> {
        match self {
            // I piece (Cyan)
            TetrominoType::I => vec![vec![true, true, true, true]],
            
            // O piece (Yellow)
            TetrominoType::O => vec![
                vec![true, true], 
                vec![true, true]
            ],
            
            // T piece (Purple)
            TetrominoType::T => vec![
                vec![false, true, false],
                vec![true, true, true],
            ],
            
            // S piece (Green)
            TetrominoType::S => vec![
                vec![false, true, true],
                vec![true, true, false],
            ],
            
            // Z piece (Red)
            TetrominoType::Z => vec![
                vec![true, true, false],
                vec![false, true, true],
            ],
            
            // J piece (Blue)
            TetrominoType::J => vec![
                vec![true, false, false],
                vec![true, true, true],
            ],
            
            // L piece (Orange)
            TetrominoType::L => vec![
                vec![false, false, true],
                vec![true, true, true],
            ],
        }
    }
}

#[derive(Clone)]
struct Tetromino {
    shape: Vec<Vec<bool>>,
    piece_type: TetrominoType,
    x: i32,
    y: i32,
}

struct PieceBag {
    pieces: Vec<TetrominoType>,
    index: usize,
}

impl PieceBag {
    fn new() -> Self {
        let mut bag = PieceBag {
            pieces: vec![
                TetrominoType::I, TetrominoType::O, TetrominoType::T, TetrominoType::S,
                TetrominoType::Z, TetrominoType::J, TetrominoType::L
            ],
            index: 0,
        };
        bag.shuffle();
        bag
    }
    
    fn shuffle(&mut self) {
        // Fisher-Yates shuffle
        for i in (1..self.pieces.len()).rev() {
            let j = gen_range(0, i + 1);
            self.pieces.swap(i, j);
        }
        self.index = 0;
    }
    
    fn next_piece(&mut self) -> TetrominoType {
        if self.index >= self.pieces.len() {
            println!("Reshuffling bag!"); // Debug output
            self.shuffle();
        }
        
        let piece = self.pieces[self.index];
        println!("Bag giving piece {:?} (index {})", piece, self.index); // Debug output
        self.index += 1;
        piece
    }
}

impl Tetromino {
    fn new_from_type(piece_type: TetrominoType) -> Self {
        println!("Creating piece: {:?}", piece_type); // Debug output
        Tetromino {
            shape: piece_type.get_shape(),
            piece_type,
            x: BOARD_WIDTH / 2 - 2,
            y: 0,
        }
    }
    
    fn rotate(&mut self) {
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
    
    fn get_blocks(&self) -> Vec<(i32, i32)> {
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
    
    fn move_by(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }
}

struct Game {
    board: Vec<Vec<Option<Block>>>,
    current_piece: Tetromino,
    next_piece: Tetromino,
    piece_bag: PieceBag,
    score: u32,
    lines_cleared: u32,
    level: u32,
    drop_timer: f32,
    drop_speed: f32,
    game_over: bool,
    blink_timer: f32,
    show_help: bool,
}

impl Game {
    fn new() -> Self {
        let mut piece_bag = PieceBag::new();
        let current_piece_type = piece_bag.next_piece();
        let next_piece_type = piece_bag.next_piece();
        
        Game {
            board: vec![vec![None; BOARD_WIDTH as usize]; BOARD_HEIGHT as usize],
            current_piece: Tetromino::new_from_type(current_piece_type),
            next_piece: Tetromino::new_from_type(next_piece_type),
            piece_bag,
            score: 0,
            lines_cleared: 0,
            level: 1,
            drop_timer: 0.0,
            drop_speed: INITIAL_DROP_SPEED,
            game_over: false,
            blink_timer: 0.0,
            show_help: true,
        }
    }
    
    fn is_valid_position(&self, piece: &Tetromino) -> bool {
        for (x, y) in piece.get_blocks() {
            if x < 0 || x >= BOARD_WIDTH || y >= BOARD_HEIGHT {
                return false;
            }
            if y >= 0 && self.board[y as usize][x as usize].is_some() {
                return false;
            }
        }
        true
    }
    
    fn place_piece(&mut self) {
        for (x, y) in self.current_piece.get_blocks() {
            if y >= 0 && y < BOARD_HEIGHT && x >= 0 && x < BOARD_WIDTH {
                self.board[y as usize][x as usize] = Some(Block { filled: true });
            }
        }
        
        // Check for game over
        if self.current_piece.y <= 0 {
            self.game_over = true;
        }
        
        // Clear lines
        self.clear_lines();
        
        // Spawn next piece using the bag system
        self.current_piece = self.next_piece.clone();
        self.current_piece.x = BOARD_WIDTH / 2 - 2;
        self.current_piece.y = 0;
        
        let next_piece_type = self.piece_bag.next_piece();
        self.next_piece = Tetromino::new_from_type(next_piece_type);
    }
    
    fn clear_lines(&mut self) {
        let mut lines_to_clear = Vec::new();
        
        for y in 0..BOARD_HEIGHT {
            let mut full = true;
            for x in 0..BOARD_WIDTH {
                if self.board[y as usize][x as usize].is_none() {
                    full = false;
                    break;
                }
            }
            if full {
                lines_to_clear.push(y as usize);
            }
        }
        
        for &line in lines_to_clear.iter().rev() {
            self.board.remove(line);
            self.board.insert(0, vec![None; BOARD_WIDTH as usize]);
        }
        
        let cleared = lines_to_clear.len() as u32;
        self.lines_cleared += cleared;
        
        // Update score
        let line_score = match cleared {
            1 => SCORE_SINGLE,
            2 => SCORE_DOUBLE,
            3 => SCORE_TRIPLE,
            4 => SCORE_TETRIS,
            _ => 0,
        };
        self.score += line_score * self.level;
        
        // Increase level every 10 lines
        let new_level = self.lines_cleared / LINES_PER_LEVEL + 1;
        if new_level != self.level {
            self.level = new_level;
            self.drop_speed = (INITIAL_DROP_SPEED - (self.level - 1) as f32 * SPEED_INCREASE_PER_LEVEL)
                .max(MIN_DROP_SPEED);
        }
    }
    
    fn move_piece(&mut self, dx: i32, dy: i32) -> bool {
        let mut new_piece = self.current_piece.clone();
        new_piece.move_by(dx, dy);
        
        if self.is_valid_position(&new_piece) {
            self.current_piece = new_piece;
            true
        } else {
            false
        }
    }
    
    fn rotate_piece(&mut self) {
        let mut new_piece = self.current_piece.clone();
        new_piece.rotate();
        
        if self.is_valid_position(&new_piece) {
            self.current_piece = new_piece;
        }
    }
    
    fn hard_drop(&mut self) {
        while self.move_piece(0, 1) {
            // Continue dropping until it can't move down
        }
    }
    
    fn update(&mut self, dt: f32) {
        self.blink_timer += dt;
        
        if self.game_over {
            return;
        }
        
        self.drop_timer += dt;
        if self.drop_timer >= self.drop_speed {
            if !self.move_piece(0, 1) {
                self.place_piece();
            }
            self.drop_timer = 0.0;
        }
    }
    
    fn handle_input(&mut self) {
        if self.game_over {
            if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) {
                *self = Game::new();
            }
            return;
        }
        
        // Hide help after first input
        if is_key_pressed(KeyCode::A) || is_key_pressed(KeyCode::Left) ||
           is_key_pressed(KeyCode::D) || is_key_pressed(KeyCode::Right) ||
           is_key_pressed(KeyCode::S) || is_key_pressed(KeyCode::Down) ||
           is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Up) ||
           is_key_pressed(KeyCode::Space) {
            self.show_help = false;
        }
        
        // Movement (A/D or Arrow Keys)
        if is_key_pressed(KeyCode::A) || is_key_pressed(KeyCode::Left) {
            self.move_piece(-1, 0);
        }
        if is_key_pressed(KeyCode::D) || is_key_pressed(KeyCode::Right) {
            self.move_piece(1, 0);
        }
        
        // Soft drop (S or Down)
        if is_key_pressed(KeyCode::S) || is_key_pressed(KeyCode::Down) {
            self.move_piece(0, 1);
        }
        
        // Rotate (W or Up)
        if is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Up) {
            self.rotate_piece();
        }
        
        // Hard drop (Space)
        if is_key_pressed(KeyCode::Space) {
            self.hard_drop();
        }
        
        // Toggle help
        if is_key_pressed(KeyCode::H) {
            self.show_help = !self.show_help;
        }
    }
    
    fn get_ghost_piece(&self) -> Tetromino {
        let mut ghost = self.current_piece.clone();
        while self.is_valid_position(&ghost) {
            ghost.y += 1;
        }
        ghost.y -= 1;
        ghost
    }
    
    fn draw(&self) {
        clear_background(GB_LIGHT);
        
        // Draw Game Boy style border
        self.draw_gb_border();
        
        // Draw board
        self.draw_board();
        
        // Draw ghost piece (preview)
        let ghost_piece = self.get_ghost_piece();
        for (x, y) in ghost_piece.get_blocks() {
            if y >= 0 {
                self.draw_ghost_block(x as f32, y as f32);
            }
        }
        
        // Draw current piece
        for (x, y) in self.current_piece.get_blocks() {
            if y >= 0 {
                self.draw_gb_block(x as f32, y as f32, true);
            }
        }
        
        // Draw UI
        self.draw_ui();
        
        // Draw help if enabled
        if self.show_help {
            self.draw_help();
        }
        
        if self.game_over {
            self.draw_game_over();
        }
    }
    
    fn draw_gb_border(&self) {
        // Outer border
        let border_thickness = 8.0;
        let board_width = BOARD_WIDTH as f32 * BLOCK_SIZE;
        let board_height = BOARD_HEIGHT as f32 * BLOCK_SIZE;
        
        // Dark outer border
        draw_rectangle(
            BOARD_OFFSET_X - border_thickness,
            BOARD_OFFSET_Y - border_thickness,
            board_width + border_thickness * 2.0,
            board_height + border_thickness * 2.0,
            GB_DARK,
        );
        
        // Medium border
        draw_rectangle(
            BOARD_OFFSET_X - border_thickness + 2.0,
            BOARD_OFFSET_Y - border_thickness + 2.0,
            board_width + (border_thickness - 4.0) * 2.0,
            board_height + (border_thickness - 4.0) * 2.0,
            GB_MED_DARK,
        );
        
        // Inner light border
        draw_rectangle(
            BOARD_OFFSET_X - 2.0,
            BOARD_OFFSET_Y - 2.0,
            board_width + 4.0,
            board_height + 4.0,
            GB_MED_LIGHT,
        );
        
        // Board background
        draw_rectangle(
            BOARD_OFFSET_X,
            BOARD_OFFSET_Y,
            board_width,
            board_height,
            GB_LIGHT,
        );
    }
    
    fn draw_board(&self) {
        // Draw grid lines
        for x in 0..=BOARD_WIDTH {
            let px = BOARD_OFFSET_X + x as f32 * BLOCK_SIZE;
            draw_line(px, BOARD_OFFSET_Y, px, BOARD_OFFSET_Y + BOARD_HEIGHT as f32 * BLOCK_SIZE, 1.0, GB_MED_LIGHT);
        }
        
        for y in 0..=BOARD_HEIGHT {
            let py = BOARD_OFFSET_Y + y as f32 * BLOCK_SIZE;
            draw_line(BOARD_OFFSET_X, py, BOARD_OFFSET_X + BOARD_WIDTH as f32 * BLOCK_SIZE, py, 1.0, GB_MED_LIGHT);
        }
        
        // Draw placed blocks
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                if self.board[y as usize][x as usize].is_some() {
                    self.draw_gb_block(x as f32, y as f32, true);
                }
            }
        }
    }
    
    fn draw_gb_block(&self, x: f32, y: f32, filled: bool) {
        let px = BOARD_OFFSET_X + x * BLOCK_SIZE + 1.0;
        let py = BOARD_OFFSET_Y + y * BLOCK_SIZE + 1.0;
        let size = BLOCK_SIZE - 2.0;
        
        if filled {
            // Filled block with Game Boy shading
            draw_rectangle(px, py, size, size, GB_DARK);
            draw_rectangle(px + 1.0, py + 1.0, size - 6.0, size - 6.0, GB_MED_DARK);
            draw_rectangle(px + 3.0, py + 3.0, size - 10.0, size - 10.0, GB_MED_LIGHT);
        }
    }
    
    fn draw_ghost_block(&self, x: f32, y: f32) {
        let px = BOARD_OFFSET_X + x * BLOCK_SIZE + 1.0;
        let py = BOARD_OFFSET_Y + y * BLOCK_SIZE + 1.0;
        let size = BLOCK_SIZE - 2.0;
        
        // Ghost block (dotted outline)
        draw_rectangle_lines(px, py, size, size, 1.0, GB_MED_DARK);
        draw_rectangle_lines(px + 2.0, py + 2.0, size - 4.0, size - 4.0, 1.0, GB_MED_DARK);
    }
    
    fn draw_ui(&self) {
        let ui_x = BOARD_OFFSET_X + BOARD_WIDTH as f32 * BLOCK_SIZE + 30.0;
        let mut ui_y = BOARD_OFFSET_Y;
        
        // Title
        draw_text("TETRIS", ui_x, ui_y, 20.0, GB_DARK);
        ui_y += 35.0;
        
        // Next piece box
        let next_box_size = 80.0;
        draw_rectangle(ui_x - 4.0, ui_y - 4.0, next_box_size + 8.0, next_box_size + 8.0, GB_DARK);
        draw_rectangle(ui_x - 2.0, ui_y - 2.0, next_box_size + 4.0, next_box_size + 4.0, GB_MED_DARK);
        draw_rectangle(ui_x, ui_y, next_box_size, next_box_size, GB_LIGHT);
        
        draw_text("NEXT", ui_x + 5.0, ui_y - 8.0, 12.0, GB_DARK);
        
        // Draw next piece
        let next_offset_x = ui_x + 20.0;
        let next_offset_y = ui_y + 20.0;
        for (row, line) in self.next_piece.shape.iter().enumerate() {
            for (col, &filled) in line.iter().enumerate() {
                if filled {
                    let px = next_offset_x + col as f32 * 16.0;
                    let py = next_offset_y + row as f32 * 16.0;
                    draw_rectangle(px, py, 14.0, 14.0, GB_DARK);
                    draw_rectangle(px + 1.0, py + 1.0, 8.0, 8.0, GB_MED_DARK);
                    draw_rectangle(px + 2.0, py + 2.0, 6.0, 6.0, GB_MED_LIGHT);
                }
            }
        }
        
        ui_y += next_box_size + 20.0;
        
        // Stats
        draw_text("SCORE", ui_x, ui_y, 12.0, GB_DARK);
        ui_y += 15.0;
        draw_text(&format!("{:06}", self.score), ui_x, ui_y, 14.0, GB_DARK);
        ui_y += 25.0;
        
        draw_text("LINES", ui_x, ui_y, 12.0, GB_DARK);
        ui_y += 15.0;
        draw_text(&format!("{:03}", self.lines_cleared), ui_x, ui_y, 14.0, GB_DARK);
        ui_y += 25.0;
        
        draw_text("LEVEL", ui_x, ui_y, 12.0, GB_DARK);
        ui_y += 15.0;
        draw_text(&format!("{:02}", self.level), ui_x, ui_y, 14.0, GB_DARK);
        ui_y += 35.0;
        
        // Blinking "PRESS H FOR HELP"
        if (self.blink_timer * 2.0) as i32 % 2 == 0 {
            draw_text("PRESS H", ui_x, ui_y, 10.0, GB_MED_DARK);
            ui_y += 12.0;
            draw_text("FOR HELP", ui_x, ui_y, 10.0, GB_MED_DARK);
        }
    }
    
    fn draw_help(&self) {
        // Help overlay
        let help_x = BOARD_OFFSET_X - 20.0;
        let help_y = BOARD_OFFSET_Y + BOARD_HEIGHT as f32 * BLOCK_SIZE + 20.0;
        let help_width = BOARD_WIDTH as f32 * BLOCK_SIZE + 40.0;
        let help_height = 140.0;
        
        // Help box
        draw_rectangle(help_x - 4.0, help_y - 4.0, help_width + 8.0, help_height + 8.0, GB_DARK);
        draw_rectangle(help_x - 2.0, help_y - 2.0, help_width + 4.0, help_height + 4.0, GB_MED_DARK);
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
        
        // Blinking close instruction
        if (self.blink_timer * 3.0) as i32 % 2 == 0 {
            draw_text("Press H to close", help_x + help_width - 120.0, help_y + help_height - 15.0, 10.0, GB_MED_DARK);
        }
    }
    
    fn draw_game_over(&self) {
        let overlay_color = Color::new(GB_DARK.r, GB_DARK.g, GB_DARK.b, 0.8);
        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), overlay_color);
        
        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;
        
        // Game over box
        let box_width = 200.0;
        let box_height = 100.0;
        let box_x = center_x - box_width / 2.0;
        let box_y = center_y - box_height / 2.0;
        
        draw_rectangle(box_x - 4.0, box_y - 4.0, box_width + 8.0, box_height + 8.0, GB_DARK);
        draw_rectangle(box_x - 2.0, box_y - 2.0, box_width + 4.0, box_height + 4.0, GB_MED_DARK);
        draw_rectangle(box_x, box_y, box_width, box_height, GB_LIGHT);
        
        draw_text("GAME OVER", box_x + 20.0, box_y + 30.0, 16.0, GB_DARK);
        
        // Blinking restart instruction
        if (self.blink_timer * 2.0) as i32 % 2 == 0 {
            draw_text("SPACE TO RESTART", box_x + 10.0, box_y + 55.0, 12.0, GB_MED_DARK);
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Game Boy Tetris".to_owned(),
        window_width: 480,
        window_height: 640,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Seed the random number generator with current time
    srand(macroquad::miniquad::date::now() as u64);
    
    let mut game = Game::new();
    
    loop {
        let dt = get_frame_time();
        
        game.handle_input();
        game.update(dt);
        game.draw();
        
        next_frame().await;
    }
}
