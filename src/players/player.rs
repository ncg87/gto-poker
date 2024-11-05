pub use crate::players::human_player::HumanPlayer;
pub use crate::players::ai_player::AIPlayer;
use crate::players::base::{PlayerAction, PlayerFunctions};

#[derive(Debug)]
pub enum PlayerType {
    HumanPlayer(HumanPlayer),
    AIPlayer(AIPlayer),
}

impl PlayerType {
    pub fn new_human_player(name: String, chips: u32) -> PlayerType {
        PlayerType::HumanPlayer(HumanPlayer::new(name, chips))
    }
    pub fn get_name(&self) -> &str {
        match self {
            PlayerType::HumanPlayer(player)=> player.get_name(),
            PlayerType::AIPlayer(_) => "AI Player",
        }
    }
    pub fn get_action<T: PlayerFunctions>(&mut self, current_bet: u32) -> PlayerAction {
        match self {
            PlayerType::HumanPlayer(player)=> player.get_action(current_bet),
            PlayerType::AIPlayer(player) => player.get_action(current_bet),
        }
    }
    pub fn get_chips(&self) -> u32 {
        match self {
            PlayerType::HumanPlayer(player)=> player.get_chips(),
            PlayerType::AIPlayer(player) => player.get_chips(),
        }
    }
}


