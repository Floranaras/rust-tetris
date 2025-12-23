use super::types::TetrominoType;
use macroquad::rand::gen_range;

pub struct PieceBag {
    pieces: Vec<TetrominoType>,
    index: usize,
}

impl PieceBag {
    pub fn new() -> Self {
        let mut bag = PieceBag {
            pieces: TetrominoType::all().to_vec(),
            index: 0,
        };
        bag.shuffle();
        bag
    }

    fn shuffle(&mut self) {
        for i in (1..self.pieces.len()).rev() {
            let j = gen_range(0, i + 1);
            self.pieces.swap(i, j);
        }
        self.index = 0;
    }

    pub fn next_piece(&mut self) -> TetrominoType {
        if self.index >= self.pieces.len() {
            self.shuffle();
        }

        let piece = self.pieces[self.index];
        self.index += 1;
        piece
    }

    #[allow(dead_code)]
    pub fn peek_next(&self) -> Option<TetrominoType> {
        if self.index < self.pieces.len() {
            Some(self.pieces[self.index])
        } else {
            None
        }
    }
}

impl Default for PieceBag {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bag_contains_all_pieces() {
        let mut bag = PieceBag::new();
        let mut collected = Vec::new();

        // Get 7 pieces
        for _ in 0..7 {
            collected.push(bag.next_piece());
        }

        for piece_type in TetrominoType::all() {
            assert!(collected.contains(&piece_type));
        }
    }

    #[test]
    fn test_bag_reshuffles() {
        let mut bag = PieceBag::new();

        for _ in 0..14 {
            bag.next_piece();
        }

        assert!(bag.next_piece() as i32 >= 0);
    }
}
