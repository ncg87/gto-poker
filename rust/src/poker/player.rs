use crate::poker::card::{Card, CardError};
use crate::poker::deck::DeckError;
use crate::poker::position::Position;
use crate::poker::variant::PokerVariant;

use thiserror::Error;
use std::io::{self, Write};


// Player class, each player has a name, # of BB, and 2 cards
// Vectors are used since they are dynamic, can have a dynamic size and are similar to C++ vectors
// Structs in Rust are similar to in C but are able to have associated functions and methods
// The 3 actions a player can make at any turn
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    // These other two are actions to take
    Fold,
    Call,
    Raise(u32), // (type) is the type of data it holds, this action is abstract, as to amount raised that is specified.
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerType {
    // May add different bots in the future
    Human,
    Bot,
}

// Option type has two values, None or Some
// - None inducated a lack of value
// - Some represent a stuct that wraps a value with type T
pub struct Player {
    name: String,
    hand: Vec<Card>,
    chips: u32,
    bet_in_round: u32,
    position: Option<Position>, // Indicates there is either no value or a Position structure
    player_type: PlayerType,
    game_variant: PokerVariant,
}


impl Player {
    // Method to initialize a player
    pub fn new(name: String, buy_in: u32, player_type: PlayerType, variant: PokerVariant) -> Self {
        Player {
            name,
            // Adjustable depending on the vorker variant plaued
            hand: Vec::with_capacity(variant.hole_cards()),
            chips: buy_in,
            bet_in_round: 0,
            position: None,
            player_type,
            game_variant: variant,

        }
    }
    // Sets the position of the player on the table
    pub fn set_position(&mut self, position: Position) {
        self.position = Some(position);
    }
    // Function to get starting catds
    pub fn receive_card(&mut self, card: Card) -> Result<(), PlayerError> {
        if self.hand.len() >= self.game_variant.hole_cards() {
            return Err(PlayerError::TooManyCards);
        }
        // Pushing a card onto the vector
        self.hand.push(card);
        Ok(())
    }
    // Function to check if hand is complete
    pub fn has_complete_hand(&self) -> bool {
        self.hand.len() == self.game_variant.hole_cards()
    }
    // Clear hand
    pub fn clear_hand(&mut self) {
        self.hand.clear();
    }
    // Get action when players turn
    pub fn get_action(&self, current_bet: u32) -> Result<Action, PlayerError> {
        match self.player_type {
            PlayerType::Human => self.get_human_action(current_bet),
            PlayerType::Bot => self.get_bot_action(current_bet),
        }
    }   
    // Get action manually
    fn get_human_action(&self, current_bet: u32) -> Result<Action, PlayerError> {
        print!("Your action (fold/call/raise): ");
        io::stdout().flush().map_err(|_| PlayerError::IoError)?;

        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(|_| PlayerError::IoError)?;

        match input.trim().to_lowercase().as_str() {
            "fold" => Ok(Action::Fold),
            "call" => Ok(Action::Call),
            "raise" => {
                print!("Enter raise amount: ");
                io::stdout().flush().map_err(|_| PlayerError::IoError)?;
                let mut amount = String::new();
                io::stdin().read_line(&mut amount).map_err(|_| PlayerError::IoError)?;
                let amount: u32 = amount.trim().parse().map_err(|_| PlayerError::InvalidInput)?;
                if amount <= current_bet {
                    return Err(PlayerError::InvalidRaiseAmount);
                }
                Ok(Action::Raise(amount))
            }
            _ => Err(PlayerError::InvalidInput),
        }
    }
    // Get bot action
    pub fn get_bot_action(&self, _current_bet: u32) -> Result<Action, PlayerError> {
        Ok(Action::Fold)
    }
    // Place bet function
    pub fn place_bet(&mut self, amount: u32) -> Result<(), PlayerError> {
        if amount > self.chips {
            return Err(PlayerError::InsufficientChips);
        }
        self.chips -= amount;
        self.bet_in_round += amount;
        Ok(())
    }

    // If they win put
    pub fn win_pot(&mut self, amount: u32) {
        self.chips += amount;
    }
    // Reset their bet, mutable since we are editing self
    pub fn reset_bet_in_round(&mut self) {
        self.bet_in_round = 0;
    }
    // Function to get their chips, need a reference to self to pass/borrow its data to 
    pub fn get_chips(&self) -> u32 {
        self.chips
    }
    // Function to get the hand of the player
    pub fn get_hand(&self) -> &[Card] {
        &self.hand
    }
    // Function to get the amount bet in the round
    pub fn get_bet_in_round(&self) -> u32 {
        self.bet_in_round
    }
    // Function to get type of player
    pub fn get_player_type(&self) -> PlayerType {
        self.player_type
    }
    // Function to get player position
    pub fn get_position(&self) -> Option<Position> {
        self.position
    }
    // Function to get game variant
    pub fn game_variant(&self) -> PokerVariant {
        self.game_variant // Doesn't need & since this strucutre implements Copy, the implict copying, copies since it is a small cheap structure to copy
    }
    // Function to adjust position, uses a mutable reference, only one at a time, dropped at end of the function
    pub fn adjust_position(&mut self, position: Position) {
        self.position = Some(position);
    }

    // Note: Mutable or Shared references are mutally exclusive

    // Function to get name
    pub fn get_name(&self) -> &String {
        // & is a shared reference, able to have as many of these as you want, quick but read-only
        &self.name // Strings aren't given the copy attribute since they are heap-allocated
    }
}

#[derive(Debug, Error)]
pub enum PlayerError {
    #[error("Player has maximum number of cards for this poker variant")]
    TooManyCards,
    #[error("Invalid input")]
    InvalidInput,
    #[error("Invalid raise amount")]
    InvalidRaiseAmount,
    #[error("Insufficient chips")]
    InsufficientChips,
    #[error("IO error")]
    IoError,
    #[error("Invalid number of cards for variant")]
    InvalidCardCount,
    #[error("Card error: {0}")]
    CardError(#[from] CardError),
    #[error("Deck error: {0}")]
    DeckError(#[from] DeckError),
}
