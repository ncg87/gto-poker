// Importing the player module 
use crate::poker::player::{Player, PlayerError, PlayerType};
// Import the deck module
use crate::poker::deck::{Deck, DeckError};
//Import card module
use crate::poker::card::{Card, CardError};
// Import the variant module
use crate::poker::variant::PokerVariant;

// Used to create errors
use thiserror::Error;

// Enumeration of different rounds
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BettingRound{
    Preflop,
    Flop,
    Turn,
    River,
    Final
}

//Poker class, each game has a deck of cards and a arbitary # of players, why a vector is being used, and contains a pot of chips
pub struct PokerGame {
    // Vec can only grow from one end, like a stack
    players: Vec<Player>,
    // Deque can grow from both ends, like a queue
    deck: Deck,
    pot: u32,
    current_bet: u32,
    community_cards: Vec<Card>,
    rounds: Option<BettingRound>,
}



// Implementing functions and methods for the Poker game class, this is an essential part of rust, allowing for structs/enum to have functionality
// Basically adds a method to strc=cut
impl PokerGame {

    // The parameter is a Vector of names (the players) and it return a PokerGame object (self)
    // It is a function since it doesn't take self as a parameter
    pub fn new() -> Result<Self, GameError>  {

        // Initialize the game
        let game = PokerGame {
            players: Vec::new(),
            deck: Deck::new().unwrap(),
            pot: 0,
            current_bet: 0,
            community_cards: Vec::with_capacity(5), // :: is used to access associated function
            rounds: None,
        };

        Ok(game) // Same as return Ok(game)
    }
    // Initializes the players
    pub fn initialize_players(&mut self, player_names: Vec<String>) -> Result<(), GameError> {
        let total_players = player_names.len();

        // Check to see if number of players is in requirements
        if total_players < 2 || total_players > 8 {
            return Err(GameError::InvalidPlayerCount)
        }
        // Iterates through the vector of names
        for name in player_names {
            // Creates a new player with the name, 1000 chips, human player type, and texas holdem variant
            let player = Player::new(name, 1000, PlayerType::Human, PokerVariant::TexasHoldem);
            // Pushes the player onto the vector of players
            self.players.push(player);
        }
        Ok(())
    }
    // Initializes and shuffles a new deck for play
    pub fn new_deck(&mut self) -> Result<(), GameError> {
        self.deck.initialize()?;
        self.deck.shuffle();
        Ok(())
    }
    // Clears the deck
    pub fn clear_deck(&mut self) {
        self.deck.clear();
    }
    // Deals the Cards, returns none if successful and an Error if not
    pub fn deal_player_cards(&mut self) -> Result<(), GameError> {
        // Deals 1st card and then 2nd
        for _ in 0..2 {
            // Give each player a card, mutable since we are editting the data in the structure
            for player in &mut self.players {
                // Pop the card on the top of the deck and hand it out, only assigns the player a card if pop returns a card
                // Uses some to determine if the Option structure has a value or not *** ? ***
                // Some doesn't work here because it is an Option type, not a Result type, it can return an error not just a value (Some)
                if let Ok(card) = self.deck.draw().map_err(|_| GameError::DeckError) { // 
                    player.receive_card(card).map_err(|_| GameError::PlayerError); // Transforms the original error into our custom error and propagates it up
                }
            }
        } 
        // Ok is nessecary since rust require all all code paths to return a matching value (it is exhaustive)
        Ok(())
    }

    // Deals out cards to players, edits self since it is editing the players
    pub fn deal_community_cards(&mut self, num_cards: usize) -> Result<(), GameError> {
        // Burns the first card
        self.deck.draw()?;
        // Depending on round deals out a different number of cards
        for _ in 0..num_cards {
            // Checks if there are cards to deal
            if let Ok(card) = self.deck.draw() {
                // Pushes the card on the stack of our originals cards
                self.community_cards.push(card);
            }
            // If there are no cards left the pop returns an error
            else {
                return Err(GameError::DeckError(DeckError::NotEnoughCards));
            }
        }
        // Everything ran smoothly
        Ok(())
    }
    // Deals the flop, initial 3 cards, and changes the round
    pub fn flop(&mut self) -> Result<(), GameError> {
        self.deal_community_cards(3)?;
        self.rounds = Some(BettingRound::Flop);
        Ok(())
    }
    // Deals the turn, 4th card, and changes the round
    pub fn turn(&mut self) -> Result<(), GameError> {
        self.deal_community_cards(1)?;
        self.rounds = Some(BettingRound::Turn);
        Ok(())
    }
    // Deals the river, 5th card, and changes the round
    pub fn river(&mut self) -> Result<(), GameError> {
        self.deal_community_cards(1)?;
        self.rounds = Some(BettingRound::River);
        Ok(())
    }
    // Ends the game
    pub fn end_game(&mut self) -> Result<(), GameError> {
        self.rounds = Some(BettingRound::Final);
        // TODO: Implement end game logic, like showing the community cards and evaluating the hands, and determining the winner

        // Clear the community cards
        self.community_cards.clear();
        // Reset the pot
        self.pot = 0;
        // Reset the current bet
        self.current_bet = 0;
        // Reset the rounds
        self.rounds = None;
        // Clear the players hands
        for player in &mut self.players {
            player.clear_hand();
        }
        // Initialize the deck
        self.deck.initialize_and_shuffle()?;
        Ok(())
    }
    // Returns the number of players
    pub fn num_players(&self) -> usize {
        self.players.len()
    }
    // Checks if there are any players
    pub fn check_no_players(&self) -> Result<(), GameError> {
        if self.players.is_empty() {
            return Err(GameError::NoPlayers); // Reusing the existing error for invalid player count
        }
        Ok(())
    }
    // Returns the number of cards in the deck
    pub fn num_cards_in_deck(&self) -> usize {
        self.deck.remaining()
    }
    // Returns the number of community cards
    pub fn num_community_cards(&self) -> usize {
        self.community_cards.len()
    }
    // Returns the number of cards in the players hands
    pub fn num_cards_in_hands(&self) -> usize {
        self.players.iter().map(|player| player.get_hand().len()).sum()
    }
    // Returns the current community cards
    pub fn get_community_cards(&self) -> &Vec<Card> {
        &self.community_cards
    }
    // Returns the current round
    pub fn get_round(&self) -> Option<BettingRound> {
        self.rounds
    }
}

#[derive(Debug, Error)]
pub enum GameError {
    #[error("Invalid number of players")]
    InvalidPlayerCount,
    #[error("No players in the game")]
    NoPlayers,
    #[error("Player error: {0}")]
    PlayerError(#[from] PlayerError),
    #[error("Card error: {0}")]
    CardError(#[from] CardError),
    #[error("Deck error: {0}")]
    DeckError(#[from] DeckError),
}
