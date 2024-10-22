use thiserror::Error;
use std::collections::HashSet;
// Enumeration similar to C
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] // Rust attribute, implements trats in the struct/enum following it
// Suit class, enumerated since there are only 4 suits
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}


// Attribute, Debug: allows for the structure to be printed just println!({:?}, variable) (anything with ! is a macro)
//            Clone:
//            Copy:
//            PartialEq:
//            Eq:
//            Hash: 
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// Card class, each card has a suit and a ranks defined by a u8, unsigned 8-bit integer
pub struct Card {
    suit: Suit,
    rank: u8,
}

// Functionallity behind Card, s.t. it can only be specific card 
impl Card {
    // Similar to a constructor, take in suit and rank and returns a card. Result is an enum used to return either Ok() or Err()
    pub fn new(suit: Suit, rank: u8) -> Result<Self, CardError> {
        // Check to see if the rank is valid, if it isn't it returns an error
        if rank < 2 || rank > 14 {
            return Err(CardError::InvalidCardRank(rank));
        }
        // If the card is valid return the card, last expression in a function is implictly returned
        Ok(Card { suit, rank }) // Implicitly returned
    }
    // Returns the suit
    pub fn suit(&self) -> Suit {
        self.suit
    }
    // Returns the rank
    pub fn rank(&self) -> u8 {
        self.rank
    }
}

// Custom errors for the deck
#[derive(Debug, Error)]
pub enum CardError {
    // Attribure from thiserror crate
    #[error("Invalid card rank: {0}")] //Custom error message, {n} represent the nth arguement returned by the error method
    InvalidCardRank(u8), // Holds a integer, numbers not in [2,14]
}

