use crate::players::action::Action;
use crate::cards::card::Card;
use thiserror::Error;

pub struct PlayerAction<'a>(&'a dyn PlayerFunctions, Action); // Box is a smart pointer(allocates on heap) while dyn is a trait object

impl<'a> PlayerAction<'a> {
    
    pub fn new(player: &'a dyn PlayerFunctions, action: Action) -> Self {
        PlayerAction(player, action)
    }
    pub fn action(&self) -> Action {
        self.1
    }
    pub fn player(&self) -> &'a dyn PlayerFunctions {
        self.0
    }
}

pub trait PlayerFunctions {

    fn receive_card(&mut self, card: Card)-> Result<(), PlayerError>;

    fn get_action(&mut self, current_bet: u32) -> PlayerAction;

    fn call(&mut self, current_bet: u32) -> Action;

    fn raise(&mut self, current_bet: u32)-> Result<Action, PlayerError> ;

    fn allin(&mut self) -> Action;

    fn win(&mut self, amount: u32);

    fn get_chips(&self) -> u32;

}

#[derive(Debug, Error)]
pub enum PlayerError {
    PlayerError,
    HumanPlayerError(String),
}

impl std::fmt::Display for PlayerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
