use crate::config::*;

pub struct Scoring {
    pub score: u32,
    pub lines_cleared: u32,
    pub level: u32,
}

impl Scoring {
    pub fn new() -> Self {
        Scoring {
            score: 0,
            lines_cleared: 0,
            level: 1,
        }
    }

    /// Add score based on lines cleared
    pub fn add_lines(&mut self, lines: u32) {
        self.lines_cleared += lines;

        let line_score = match lines {
            1 => SCORE_SINGLE,
            2 => SCORE_DOUBLE,
            3 => SCORE_TRIPLE,
            4 => SCORE_TETRIS,
            _ => 0,
        };

        self.score += line_score * self.level;
        self.update_level();
    }

    /// Update level based on lines cleared
    fn update_level(&mut self) {
        let new_level = self.lines_cleared / LINES_PER_LEVEL + 1;
        if new_level != self.level {
            self.level = new_level;
        }
    }

    /// Get current drop speed based on level
    pub fn get_drop_speed(&self) -> f32 {
        (INITIAL_DROP_SPEED - (self.level - 1) as f32 * SPEED_INCREASE_PER_LEVEL)
            .max(MIN_DROP_SPEED)
    }

    /// Reset scoring
    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.score = 0;
        self.lines_cleared = 0;
        self.level = 1;
    }
}

impl Default for Scoring {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scoring_creation() {
        let scoring = Scoring::new();
        assert_eq!(scoring.score, 0);
        assert_eq!(scoring.level, 1);
    }

    #[test]
    fn test_single_line_score() {
        let mut scoring = Scoring::new();
        scoring.add_lines(1);
        assert_eq!(scoring.score, SCORE_SINGLE);
        assert_eq!(scoring.lines_cleared, 1);
    }

    #[test]
    fn test_tetris_score() {
        let mut scoring = Scoring::new();
        scoring.add_lines(4);
        assert_eq!(scoring.score, SCORE_TETRIS);
    }

    #[test]
    fn test_level_progression() {
        let mut scoring = Scoring::new();
        scoring.add_lines(10); // Should advance to level 2
        assert_eq!(scoring.level, 2);
    }

    #[test]
    fn test_score_multiplier() {
        let mut scoring = Scoring::new();
        scoring.add_lines(10); // Level 2
        let initial_score = scoring.score;
        
        scoring.add_lines(1); // Single line at level 2
        assert_eq!(scoring.score, initial_score + SCORE_SINGLE * 2);
    }

    #[test]
    fn test_drop_speed() {
        let mut scoring = Scoring::new();
        let initial_speed = scoring.get_drop_speed();
        
        scoring.add_lines(10); // Level 2
        let new_speed = scoring.get_drop_speed();
        
        assert!(new_speed < initial_speed);
    }
}
