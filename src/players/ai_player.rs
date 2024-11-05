use crate::cards::card::Card;
use crate::players::action::Action;
use crate::players::base::{PlayerFunctions, PlayerAction, PlayerError};

#[derive(Debug)]
pub struct AIPlayer {
}

impl PlayerFunctions for AIPlayer {
    fn get_action(&mut self, current_bet: u32) -> PlayerAction {
        todo!()
    }
    fn receive_card(&mut self, _card: Card) -> Result<(), PlayerError> {
        todo!()
    }
    fn call(&mut self, _current_bet: u32) -> Action {
        todo!()
    }
    fn raise(&mut self, _current_bet: u32) -> Result<Action, PlayerError> {
        todo!()
    }
    fn allin(&mut self) -> Action {
        todo!()
    }
    fn win(&mut self, _amount: u32) {
        todo!()
    }
    fn get_chips(&self) -> u32 {
        todo!()
    }
}
