use gto_poker::poker::texas_hold_em::{PokerGame, GameError};
use gto_poker::poker::variant::PokerVariant;

fn create_game() -> PokerGame {
    PokerGame::new().expect("Failed to create game") // If not a semicolon, it will return the value of last line
}

#[test]
fn test_no_players() {
    let game = create_game();
    assert!(matches!(game.check_no_players(), Err(GameError::NoPlayers)));
}

#[test]
fn test_initialize_players() {
    let mut game = create_game();
    game.initialize_players(vec!["Alice".to_string(), "Bob".to_string()]).unwrap();
    assert_eq!(game.num_players(), 2);
}

#[test]
fn test_new_deck() {
    // Initializes a new deck
    let mut game = create_game();
    // Check there are 0 cards in the deck
    assert_eq!(game.num_cards_in_deck(), 0);
    // Initializes a new deck
    game.new_deck().unwrap();
    // Check there are 52 cards in the deck
    assert_eq!(game.num_cards_in_deck(), 52);
    // Clears the deck
    game.clear_deck();
    // Check there are 0 cards in the deck
    assert_eq!(game.num_cards_in_deck(), 0);
}

#[test]
fn test_deal_player_cards() {
    let mut game = create_game();
    game.initialize_players(vec!["Alice".to_string(), "Bob".to_string()]).unwrap();
    game.new_deck().unwrap();
    game.deal_player_cards().unwrap();
    assert_eq!(game.num_cards_in_hands(), 4);
    assert_eq!(game.num_cards_in_deck(), 48);
}

// Test the flop
#[test]
fn test_flop() {
    let mut game = create_game();
    game.initialize_players(vec!["Alice".to_string(), "Bob".to_string()]).unwrap();
    game.new_deck().unwrap();
    game.deal_player_cards().unwrap();
    game.flop().unwrap();
    assert_eq!(game.num_community_cards(), 3);
    assert_eq!(game.num_cards_in_deck(), 44);
}

// Test the turn
#[test]
fn test_turn() {
    let mut game = create_game();
    game.initialize_players(vec!["Alice".to_string(), "Bob".to_string()]).unwrap();
    game.new_deck().unwrap();
    assert_eq!(game.num_cards_in_deck(), 52);
    game.deal_player_cards().unwrap();
    assert_eq!(game.num_cards_in_deck(), 48);
    game.flop().unwrap();
    assert_eq!(game.num_cards_in_deck(), 44);
    game.turn().unwrap();
    assert_eq!(game.num_cards_in_deck(), 42);
    assert_eq!(game.num_community_cards(), 4);
}

// Test the river
#[test]
fn test_river() {
    let mut game = create_game();
    game.initialize_players(vec!["Alice".to_string(), "Bob".to_string()]).unwrap();
    game.new_deck().unwrap();
    assert_eq!(game.num_cards_in_deck(), 52);
    game.deal_player_cards().unwrap();
    assert_eq!(game.num_cards_in_deck(), 48);
    game.flop().unwrap();
    assert_eq!(game.num_cards_in_deck(), 44);
    game.turn().unwrap();
    assert_eq!(game.num_cards_in_deck(), 42);
    game.river().unwrap();
    assert_eq!(game.num_community_cards(), 5);
    assert_eq!(game.num_cards_in_deck(), 40);
}

// Test the get_community_cards
#[test]
fn test_get_community_cards() {
    let mut game = create_game();
    game.initialize_players(vec!["Alice".to_string(), "Bob".to_string()]).unwrap();
    game.new_deck().unwrap();
    game.deal_player_cards().unwrap();
    assert_eq!(game.num_cards_in_deck(), 48);
    // Alwats burns a card and then draws n cards (n depends on rounds)
    game.flop().unwrap();
    assert_eq!(game.num_cards_in_deck(), 44);
    game.turn().unwrap();
    assert_eq!(game.num_cards_in_deck(), 42);
    game.river().unwrap();
    assert_eq!(game.num_cards_in_deck(), 40);
    let community_cards = game.get_community_cards();
    assert_eq!(community_cards.len(), 5);
}

// Test the get_player_cards
#[test]
fn test_get_player_cards() { 
    let mut game = create_game();
    game.initialize_players(vec!["Alice".to_string(), "Bob".to_string()]).unwrap();
    game.new_deck().unwrap();
    game.deal_player_cards().unwrap();
    let player_cards = game.num_cards_in_hands();
    assert_eq!(player_cards, 4);
}

// Test end of round
#[test]
fn test_end_game() {
    let mut game = create_game();
    game.initialize_players(vec!["Alice".to_string(), "Bob".to_string()]).unwrap();
    game.new_deck().unwrap();
    game.deal_player_cards().unwrap();
    game.flop().unwrap();
    game.turn().unwrap();
    game.river().unwrap();
    game.end_game().unwrap();
    assert_eq!(game.num_community_cards(), 0);
    assert_eq!(game.num_cards_in_hands(), 0);
    assert_eq!(game.num_cards_in_deck(), 52);
}

