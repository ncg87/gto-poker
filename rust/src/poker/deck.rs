use crate::poker::card::{Card, Suit, CardError};
use rand::seq::SliceRandom;
use thiserror::Error;

// Basic structure of a Deck
pub struct Deck {
    cards: Vec<Card>
}

// Functions of a Deck
impl Deck {
    // Initializing a new deck
    pub fn new() -> Result<Self, DeckError> {
        // Initializes an array to store cards
        let deck = Deck { cards: Vec::with_capacity(52)};
        // Returns the object
        Ok(deck)
    }

    // Intializes a deck of 52 cards
    // Takes a mutable version of self to edit, returns Ok(), success with no value, or an error if caused (in this case only if a card is out of boudns)
    pub fn initialize(&mut self) -> Result<(), DeckError>{
        // Clear the Vec of cards, an function of the Deque structure
        self.cards.clear();
        // Creates 52 cards, one for each suit and each rank, 2-10 and 11-14 represent Jack to Ace
        // &suit dereferences the reference to each suit, since we want the suit not &Suit, we need & so that we read the array, not take ownership of it
        for &suit in &[Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] { // Iterates through all suits
            for rank in 2..=14 { // iterates through all cards from 2 to 15
                self.cards.push(Card::new( suit, rank )?); // Creates a new card and pushes to the deck, the '?' is incase of error when creating a card so it can propagate up
            }
        }
        // If no errors return success
        Ok(())
    }

    // Shuffles deck so each dealing is random, mutable so it is editing itself
    pub fn shuffle(&mut self) {
        // Generates a random number
        let mut rng = rand::thread_rng(); // returns a random number local to the current thread
        // Makes the Vec contiguous (makes sure all elements are stored in a continous block of memory), and then shuffles it (contiguous makes it more efficent)
        self.cards.shuffle(&mut rng) //&mut rng since the number has to update as it shuffles
    }
    // Initializes and shuffles the deck
    pub fn initialize_and_shuffle(&mut self) -> Result<(), DeckError> {
        self.initialize()?;
        self.shuffle();
        Ok(())
    }
    // Draws a card
    pub fn draw(&mut self) -> Result<Card, DeckError> {
        self.cards.pop().ok_or(DeckError::NotEnoughCards)
    }
    // Check how many cards are remaining
    pub fn remaining(&self) -> usize {
        self.cards.len()
    }
    // Clears the deck
    pub fn clear(&mut self) {
        self.cards.clear();
    }
}

// Custom errors for the deck
#[derive(Debug, Error)]
pub enum DeckError {
    // Attribute from thiserror crate
    #[error("Not enough cards in the deck")] //Custom error message
    NotEnoughCards,
    #[error("Card error: {0}")]
    CardError(#[from] CardError),
}