use std::io; // Used for input
use thiserror::Error; // Used for custom errors
use crate::cards::card::Card;
use crate::players::action::Action;
use crate::players::base::{PlayerFunctions, PlayerAction, PlayerError};

#[derive(Debug, Error)]
pub enum HumanError {
    #[error("Raise must be at least {0}")]
    RaiseMustBeAtLeast(u32),
    
}

impl From<HumanError> for PlayerError {
    fn from(value: HumanError) -> Self {
        PlayerError::HumanPlayerError(value.to_string())
    }
}


#[derive(Debug)]
pub struct HumanPlayer {
    name: String,
    hand: Vec<Card>,
    chips: u32,
    bet_in_round: u32,
}

impl PlayerFunctions for HumanPlayer {

    fn receive_card(&mut self, card: Card)-> Result<(), PlayerError>{
        self.hand.push(card);
        Ok(())
    }

    fn get_action(&mut self, current_bet: u32) -> PlayerAction {
        self.get_action_with_input(current_bet, None)
    }

    fn call(&mut self, current_bet: u32) -> Action {
        if current_bet > self.chips {
            return self.allin();
        }
        self.chips -= current_bet; // Remove chips from player
        self.bet_in_round += current_bet; // Tracks chips in pot
        return Action::Call;
    }

    fn raise(&mut self, current_bet: u32) -> Result<Action, PlayerError> {
        self.get_raise_with_input(current_bet, None)
    }

    fn allin(&mut self) -> Action {
        self.bet_in_round += self.chips; // Add all chips to the pot
        self.chips = 0; // Remove all chips from player
        return Action::AllIn(self.bet_in_round);
    }
    fn win(&mut self, pot: u32) {
        self.chips += pot;
    }
    
    fn get_chips(&self) -> u32 {
        self.chips
    }

}

impl HumanPlayer {
    // Allocates the structure to the heap
    pub fn new(name: String, chips: u32) -> HumanPlayer {
        HumanPlayer { name, hand: Vec::new(), chips, bet_in_round: 0 }
    }
    // Overloaded get_action to take an input
    fn get_action_with_input(&mut self, current_bet: u32, input: Option<&str>) -> PlayerAction {
        loop {
            println!("Enter your action {} (Check, Fold, Call, Raise): ", self.get_name()); // Ask for input
            
            let input = if let Some(input) = input{ // If input is provided, convert to string
                input.to_string()
            } else { // If no input is provided, read from stdin
                let mut input = String::new();
                if io::stdin().read_line(&mut input).is_err() {
                    println!("Error reading input!");
                    continue;
                }
                input.trim().to_lowercase()
            };

            match input.to_lowercase().trim() { // Remove whitespace from input and converts to str slice
                "check" => {
                    // If the current bet is greater than 0, the player must call
                    if current_bet > 0 || self.bet_in_round != current_bet {
                        println!("Active bet is {}, you must call or fold!", current_bet);
                        continue;
                    }
                    return PlayerAction::new(self, Action::Check);
                },
                "fold" => {
                    return PlayerAction::new(self, Action::Fold);
                },
                "call" => {
                    let action = self.call(current_bet);
                    return PlayerAction::new(self, action);
                },
                "raise" => {
                    match self.raise(current_bet) {
                        Ok(action) => {
                            return PlayerAction::new(self, action);
                        },
                        Err(e) => {
                            println!("{}", e);
                            continue;
                        },
                    }
                },
                _ => { // Any other input
                    println!("Invalid action, please try again!");
                    continue;
                },
            }
        }
    }

    fn get_raise_with_input(&mut self, current_bet: u32, input: Option<&str>) -> Result<Action, PlayerError> {
        loop {

            let trimmed_input = if let Some(input) = input {
                input.to_string() // Use provided input
            }
            else{
                println!("Enter the amount you want to Raise or go All In: ");
                let mut input = String::new(); // Reset the input each loop
                if io::stdin().read_line(&mut input).is_err() {
                    println!("Error reading input!");
                    continue;
                }
                input.replace(" ", "").to_lowercase().trim().to_string()
            };

            // Check for all in
            if trimmed_input == "allin" {
                return Ok(self.allin());
            }

            // Check for a raise
            if let Ok(amount) = trimmed_input.parse::<u32>() { // Parses the trimmed input to a u32
                // If amount typed is greater than the player's chips, go all in
                if amount > self.chips {
                    return Ok(self.allin());
                }
                // Raise must be at least double including the current bet
                if amount < 2 * current_bet {
                    return Err(HumanError::RaiseMustBeAtLeast(current_bet).into());
                }
                self.chips -= amount;
                self.bet_in_round += amount;
                    return Ok(Action::Raise(amount));
                }

            println!("Invalid amount, please try again!");
        }
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

// Unit tests
#[cfg(test)] // Trait to only compile cargo test is ran
mod tests {
    use super::*; // Import all code from parent module
    #[test]
    fn test_human_player_new() { // Test if we can create a new human player
        let player1 = HumanPlayer::new(String::from("Player 1"), 100);
        assert_eq!(player1.name, "Player 1");
        assert_eq!(player1.chips, 100);
    }
    #[test]
    fn test_human_check() { // Test if we can get an action from a human player with an input
        let mut player1 = HumanPlayer::new(String::from("Player 1"), 100);
        let action = player1.get_action_with_input(0, Some("check"));
        assert_eq!(action.action(), Action::Check);
        assert_eq!(player1.chips, 100);
        assert_eq!(player1.bet_in_round, 0);
    }
    #[test]
    fn test_human_fold() {
        let mut player1 = HumanPlayer::new(String::from("Player 1"), 100);
        let action = player1.get_action_with_input(10, Some("fold"));
        assert_eq!(action.action(), Action::Fold);
    }
    #[test]
    fn test_human_call() {
        let mut player1 = HumanPlayer::new(String::from("Player 1"), 100);
        let action = player1.get_action_with_input(10, Some("call"));
        assert_eq!(action.action(), Action::Call);
        assert_eq!(player1.chips, 90);
        assert_eq!(player1.bet_in_round, 10);
    }
    #[test]
    fn test_human_raise() {
        let mut player1 = HumanPlayer::new(String::from("Player 1"), 100);
        // Raise 30
        let action = player1.get_raise_with_input(10, Some("30"));
        assert_eq!(action.unwrap(), Action::Raise(30));
        assert_eq!(player1.chips, 70);
        assert_eq!(player1.bet_in_round, 30);
        // Raise 60
        let action2 = player1.get_raise_with_input(30, Some("60"));
        assert_eq!(action2.unwrap(), Action::Raise(60));
        assert_eq!(player1.chips, 10);
        assert_eq!(player1.bet_in_round, 90);
        // All in
        let action3 = player1.get_raise_with_input(60, Some("20"));
        assert_eq!(action3.unwrap(), Action::AllIn(100));
        assert_eq!(player1.chips, 0);
        assert_eq!(player1.bet_in_round, 100);
    }
    #[test]
    fn test_human_allin() {
        let mut player1 = HumanPlayer::new(String::from("Player 1"), 100);
        let action1 = player1.get_raise_with_input(0, Some("20"));
        assert_eq!(action1.unwrap(), Action::Raise(20));
        assert_eq!(player1.chips, 80);
        assert_eq!(player1.bet_in_round, 20);

        let action = player1.get_raise_with_input(10, Some("allin"));
        assert_eq!(action.unwrap(), Action::AllIn(100));
        assert_eq!(player1.chips, 0);
        assert_eq!(player1.bet_in_round, 100);
    }
    #[test]
    fn test_human_raise_must_be_at_least() {
        let mut player1 = HumanPlayer::new(String::from("Player 1"), 100);
        let action = player1.get_raise_with_input(15, Some("20"));
        assert_eq!(action.is_err(), true);
        assert_eq!(player1.chips, 100);
        assert_eq!(player1.bet_in_round, 0);
    }
}

