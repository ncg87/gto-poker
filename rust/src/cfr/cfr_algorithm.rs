use crate::poker_game::PokerGame;




pub struct CFRTrainer {
    game: PokerGame,
    // ... other fields ...
}

impl CFRTrainer {
    pub fn new(game: PokerGame) -> Self {
        CFRTrainer {
            game,
            // ... initialize other fields ...
        }
    }
    // ... other methods ...
}