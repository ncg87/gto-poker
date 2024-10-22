// Import statements, : used instead of / or .
use std::collections::VecDeque;
use rand::seq::SliceRandom;
use thiserror::Error;

// Importing the player module 
use crate::player::Player;
// Import action enum module
use crate::player::Action;

// Import the deck module
use crate::deck::Deck;


// Enumeration of different rounds
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BettingRound{
    Preflop,
    Flop,
    Turn,
    River,
    Final
}
// Game state of a single round
pub struct GameState {
    pub community_cards: Vec<Card>,
    pub current_bet: u32,
    pub pot: u32,
    pub player_bets: Vec<(String, u32)>, // (player_name, bet_amount)
}



// Creating errors, public so that it can be used outside of this module
#[derive(Debug, Error)]
// Different enumerations
pub enum GameError {
    // Attribure from this error crate
    #[error("Player's hand is full")]
    PlayerHandFull,
    #[error("Invalid action")]
    InvalidAction,
    #[error("Insufficient chips")]
    InsufficientChips,
}


//Poker class, each game has a deck of cards and a arbitary # of players, why a vector is being used, and contains a pot of chips
pub struct PokerGame {
    // Vec can only grow from one end, like a stack
    players: Vec<Player>,
    // Deque can grow from both ends, like a queue
    deck: Deque<Card>,
    pot: u32,
    community_cards: Vec<Card>,
}



// Implementing functions and methods for the Poker game class, this is an essential part of rust, allowing for structs/enum to have functionality
// Basically adds a method to strc=cut
impl PokerGame {
    // The parameter is a Vector of names (the players) and it return a PokerGame object (self)
    // It is a function since it doesn't take self as a parameter
    pub fn new() -> Result<Self, GameError> {

        // Initialize the game
        let mut game = PokerGame {
            players: Vec::new(Player),
            deck: VecDeque::new(Card),
            pot: 0,
            community_cards: Vec::with_capacity(5), // :: is used to access associated function
            rounds: BettingRound::Preflop,
        };

        // Initializes a new deck
        game.initialize_deck()?;
        // Shuffles the deck for randomization
        game.shuffle_deck();
        Ok(game); // Same as return Ok(game)
    } 


    // Deals the Cards, returns none if successful and an Error if not
    pub fn deal_player_cards(&mut self) -> Result<(), GameError> {
        // Deals 1st card and then 2nd
        for _ in 0..2 {
            // Give each player a card, mutable since we are editting the data in the structure
            for player in &mut self.players {
                // Pop the card on the top of the deck and hand it out, only assigns the player a card if pop returns a card
                // Uses some to determine if the Option structure has a value or not *** ? ***
                if let Some(card) = self.deck.pop_front() {
                    player.receive_card(card).map_err(|_| GameError::PlayerHandFull)?; // Transforms the original error into our custom error and propagates it up
                }
                // If there are no cards left the pop return error
                else {
                    return Err(GameError::NotEnoughCards)
                }
            }
        } 
        // Ok is nessecary since rust require all all code paths to return a matching value (it is exhaustive)
        Ok(())
    }

    // Deals out cards to players, edits self since it is editing the players
    pub fn deal_community_cards(&mut self, num_cards: usize) -> Result<(), GameError> {
        // Burns the first card
        self.deck.pop_front();
        // Depending on round deals out a different number of cards
        for _ in 0..num_cards {
            // Checks if there are cards to deal
            if let Some(card) = self.deck.pop_front() {
                // Pushes the card on the stack of our originals cards
                self.community_cards.push(card);
            }
            // If there are no cards left the pop returns an error
            else {
                return Err(GameError::NotEnoughCards);
            }
        }
        // Everything ran smoothly
        Ok(());
    }

    pub fn simulate_round(&mut self, player_names: Vec<String>) -> Result<(), GameError> {
        
        // Intialize players by coverting the original Vec<String> into an iterator, then mapping each name to a Player structure and initializing all the names and chips, then collects the elements into a new collection (Vec<PLayer>)
        let players = player_names.into_iter() // Convert into iterator of Strings
        .map(|name| Player::new(name,1000)) // Iterates over name and for each creates a Player object with specified name and chips
        .collect(); // Collects these objects into a new Vec
        
        // Put players into the object
        self.players = players;

        // Reset game state
        self.pot = 0;
        self.community_cards.clear();
        // Initalize to Preflop
        self.rounds = BettingRound::Preflop;

        // Shuffle the deck
        self.shuffle_deck();

        // Deal cards to players
        self.deal_player_cards()?;

        // Simulate betting rounds until we reach the final round
        while self.rounds != BettingRound::Final {
            // Check which round it is
            match self.rounds {
                BettingRound::Preflop => {
                    // Preflop betting
                    self.simulate_betting()?;
                    self.rounds = BettingRound::Flop;
                },
                BettingRound::Flop => {
                    // Deal flop
                    self.deal_community_cards(3)?;
                    self.simulate_betting()?;
                    self.rounds = BettingRound::Turn;
                },
                BettingRound::Turn => {
                    // Deal turn
                    self.deal_community_cards(1)?;
                    self.simulate_betting()?;
                    self.rounds = BettingRound::River;
                },
                BettingRound::River => {
                    // Deal river
                    self.deal_community_cards(1)?;
                    self.simulate_betting()?;
                    self.rounds = BettingRound::Final;
                },
                BettingRound::Final => {
                    // This should never be reached in the loop
                    unreachable!();
                },
            }
        }
        // Determine winner and distribute the pot
        self.determine_winner()?;

        Ok(())
        
    }
    

    // Simulates a betting round
    // Need to figure out how to keep track of activate players
    fn simulate_betting(&mut self) -> Result<(), GameError> {
        // Keep track of the current bet
        let mut current_bet = 0;
        // Keep track of the size of the pit
        let mut pot_in_round = 0;
        // Keep track of which players are playing
        let mut active_players: VecDeque<&mut Player> = self.players.iter_mut().collect();
        // Initialize the round
        let mut betting_round = true;
        // Keep betting active until the round ends
        while betting_round {
            // Cycle through all activate players
            for player in &mut active_players {
                // Check if a player is all-in if they are skip their turn
                if player.chips == 0 {
                    continue;
                }
                
                // Get the action of the player
                let action = player.get_player_action(player, current, bet)?;

                match action {
                    // If the player folds
                    Action::Fold => {
                        println!("{} folds", player.name);
                        active_players.retain(|p| p.name != player.name);
                    },
                    // If the player calls
                    Action::Call => {
                        let call_amount = current_bet.saturating_sub(player.bet_in_round);
                        if call_amount > 0 {
                            self.bet(player, call_amount)?;
                            pot_in_round += call_amount;
                            println!("{} calls {}", player.name, call_amount);
                        } else {
                            println!("{} checks", player.name);
                        }
                    },
                    // If the player raises
                    Action::Raise(raise_to) => {
                        if raise_to <= current_bet {
                            return Err(GameError::InvalidAction);
                        }
                        let raise_amount = raise_to - player.bet_in_round;
                        self.bet(player, raise_amount)?;
                        pot_in_round += raise_amount;
                        current_bet = raise_to;
                        println!("{} raises to {}", player.name, raise_to);
                        betting_round = true;
                    },
                }
                // End the round if there is one player left
                if active_players.len() == 1{
                    break;
                }
            }
        }
        // Add to the pot
        self.pot += pot_in_round;
        // Reset bet_in_round for all players
        for player in &mut self.players {
            player.bet_in_round = 0;
        }
        // Return success of the round
        Ok(())

    }
    fn bet(&mut self, player: &mut Player, amount: u32) -> Result<(), GameError> {
        if amount > player.chips {
            return Err(GameError::InsufficientChips);
        }
        player.chips -= amount;
        player.bet_in_round += amount;
        Ok(())
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game() -> Result<(), GameError> {
        // Initialize the players
        let player_names = vec!["Alice".to_string(), "Bob".to_string()];
        // Create a game with the players
        let game = PokerGame::new(player_names)?;
        // Check the num of players is consistant
        assert_eq!(game.players.len(), 2);
        // Check the length of the deck
        assert_eq!(game.deck.len(), 52);
        // Check the pot it initialize correctly
        assert_eq!(game.pot, 0);
        Ok(())
    }
    // Check that dealing the card to players work
    #[test]
    fn test_deal_cards() -> Result<(), GameError> {
        let player_names = vec!["Alice".to_string(), "Bob".to_string()];
        let mut game = PokerGame::new(player_names)?;
        // Deal out the cards
        game.deal_player_cards()?;
        assert_eq!(game.players[0].hand.len(), 2);
        assert_eq!(game.players[1].hand.len(), 2);
        assert_eq!(game.deck.len(), 48);
        Ok(())
    }
    // Check that invalid cards work
    #[test]
    fn test_invalid_card_rank() {
        assert!(Card::new(Suit::Hearts, 1).is_err());
        assert!(Card::new(Suit::Hearts, 15).is_err());
    }
}
