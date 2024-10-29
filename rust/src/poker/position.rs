// All different positions up to 8 players
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Position {
    SmallBlind,
    BigBlind,
    UTG,      // Under the Gun
    UTG1,     // UTG+1
    MP,       // Middle Position
    HJ,       // Hijack
    CO,       // Cut Off
    Button,      // Button/Dealer
}
// Implementation of the class
impl Position {
    // Get all possible positions
    pub fn all_positions() -> Vec<Position> {
        vec![
            Position::Button,
            Position::SmallBlind,
            Position::BigBlind,
            Position::UTG,
            Position::UTG1,
            Position::MP,
            Position::HJ,
            Position::CO,
        ]
    }

    // Get positions for a specific number of players
    pub fn positions_for_players(num_players: usize) -> Vec<Position> {
        match num_players {
            2 => vec![Position::Button, Position::BigBlind], // Heads up: no SB position
            3 => vec![Position::Button, Position::SmallBlind, Position::BigBlind],
            4 => vec![Position::Button, Position::SmallBlind, Position::BigBlind, Position::UTG],
            5 => vec![Position::Button, Position::SmallBlind, Position::BigBlind, Position::UTG, Position::CO],
            6 => vec![Position::Button, Position::SmallBlind, Position::BigBlind, Position::UTG, Position::HJ, Position::CO],
            7 => vec![Position::Button, Position::SmallBlind, Position::BigBlind, Position::UTG, Position::MP, Position::HJ, Position::CO],
            8 => vec![Position::Button, Position::SmallBlind, Position::BigBlind, Position::UTG, Position::UTG1, Position::MP, Position::HJ, Position::CO],
            _ => Vec::new(), // Handle invalid number of players
        }
    }

    // Get the next position in rotation
    pub fn next_position(&self, num_players: usize) -> Position {
        let positions = Position::positions_for_players(num_players);
        let current_index = positions.iter().position(|&p| p == *self).unwrap_or(0);
        let next_index = (current_index + 1) % positions.len();
        positions[next_index]
    }

    // Check if position is in blinds
    pub fn is_blind(&self) -> bool {
        matches!(self, Position::SmallBlind | Position::BigBlind)
    }

    // Get position relative to button
    pub fn distance_from_btn(&self) -> usize {
        match self {
            Position::Button => 0,
            Position::SmallBlind => 1,
            Position::BigBlind => 2,
            Position::UTG => 3,
            Position::UTG1 => 4,
            Position::MP => 5,
            Position::HJ => 6,
            Position::CO => 7,
        }
    }
}
