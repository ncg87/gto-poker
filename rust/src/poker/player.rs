use crate::poker::card::{Card, CardError};
use crate::poker::deck::{Deck, DeckError};

use thiserror::Error;
use std::io::{self, Write};
use rand::Rng;

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

pub struct Player {
    name: String,
    hand: Vec<Card>,
    num_of_BB: u32,
    bet_in_round: u32,
    player_type: PlayerType,
}

impl Player {
    // Method to initialize a player
    pub fn new(name: String, buy_in: u32, player_type: PlayerType) -> Self {
        Player {
            name,
            hand: Vec::with_capacity(2),
            num_of_BB: buy_in,
            bet_in_round: 0,
            player_type,

        }
    }
    // Function to get starting catds
    pub fn receive_card(&mut self, card: Card) -> Result<(), PlayerError> {
        if self.hand.len() >= 2 {
            return Err(PlayerError::TooManyCards);
        }
        self.hand.push(card);
        Ok(())
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
    // Place bet function
    pub fn place_bet(&mut self, amount: u32) -> Result<u32, PlayerError> {
        if amount > self.chips {
            return Err(PlayerError::InsufficientChips);
        }
        self.chips -= amount;
        self.bet_in_round += amount;
        Ok(amount)
    }

    // If they win put
    pub fn win_pot(&mut self, amount: u32) {
        self.chips += amount;
    }
    // Reset their bet
    pub fn reset_bet_in_round(&mut self) {
        self.bet_in_round = 0;
    }
    // Function to get their chips
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
}

#[derive(Debug, Error)]
pub enum PlayerError {
    #[error("Player already has maximum number of cards")]
    TooManyCards,
    #[error("Invalid input")]
    InvalidInput,
    #[error("Invalid raise amount")]
    InvalidRaiseAmount,
    #[error("Insufficient chips")]
    InsufficientChips,
    #[error("IO error")]
    IoError,
    #[error("Card error: {0}")]
    CardError(#[from] CardError),
    #[error("Deck error: {0}")]
    DeckError(#[from] DeckError),
}
