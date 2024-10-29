use gto_poker::poker::player::{Player, PlayerType, PlayerError};
use gto_poker::poker::card::{Card, Suit};
use gto_poker::poker::position::Position;
use gto_poker::poker::variant::PokerVariant;

// Helper function to create a test player for Texas Hold'em
fn create_test_player() -> Player {
    Player::new(
        String::from("Test Player"), 
        1000, 
        PlayerType::Bot,
        PokerVariant::TexasHoldem
    )
}

#[test]
fn test_new_player() {
    let player = create_test_player();
    assert_eq!(player.get_chips(), 1000);
    assert_eq!(player.get_hand().len(), 0);
    assert_eq!(player.get_bet_in_round(), 0);
    assert_eq!(player.game_variant(), PokerVariant::TexasHoldem);
}

#[test]
fn test_receive_card_texas_holdem() {
    let mut player = create_test_player();
    
    // Test receiving first hole card
    let card1 = Card::new(Suit::Hearts, 10).unwrap();
    assert!(player.receive_card(card1).is_ok());
    assert_eq!(player.get_hand().len(), 1);
    assert!(!player.has_complete_hand());
    
    // Test receiving second hole card
    let card2 = Card::new(Suit::Spades, 14).unwrap();
    assert!(player.receive_card(card2).is_ok());
    assert_eq!(player.get_hand().len(), 2);
    assert!(player.has_complete_hand());
    
    // Test receiving third card (should fail for Texas Hold'em)
    let card3 = Card::new(Suit::Clubs, 2).unwrap();
    assert!(matches!(
        player.receive_card(card3),
        Err(PlayerError::TooManyCards)
    ));
    assert_eq!(player.get_hand().len(), 2);
}

#[test]
fn test_variant_specific_limits() {
    // Test for Texas Hold'em (2 hole cards)
    let mut holdem_player = Player::new(
        String::from("Holdem Player"),
        1000,
        PlayerType::Bot,
        PokerVariant::TexasHoldem
    );

    // Add 2 cards (should succeed)
    assert!(holdem_player.receive_card(Card::new(Suit::Hearts, 10).unwrap()).is_ok());
    assert!(holdem_player.receive_card(Card::new(Suit::Spades, 11).unwrap()).is_ok());
    assert!(holdem_player.has_complete_hand());

    // Add third card (should fail)
    assert!(matches!(
        holdem_player.receive_card(Card::new(Suit::Clubs, 12).unwrap()),
        Err(PlayerError::TooManyCards)
    ));
}

#[test]
fn test_clear_hand() {
    let mut player = create_test_player();
    
    // Add Texas Hold'em starting hand
    let card1 = Card::new(Suit::Hearts, 10).unwrap();
    let card2 = Card::new(Suit::Spades, 14).unwrap();
    player.receive_card(card1).unwrap();
    player.receive_card(card2).unwrap();
    
    assert_eq!(player.get_hand().len(), 2);
    assert!(player.has_complete_hand());
    
    player.clear_hand();
    assert_eq!(player.get_hand().len(), 0);
    assert!(!player.has_complete_hand());
}

#[test]
fn test_place_bet() {
    let mut player = create_test_player();
    
    // Test standard Texas Hold'em bet sizes
    let big_blind = 100;
    
    // Test min-raise
    assert!(player.place_bet(big_blind * 2).is_ok());
    assert_eq!(player.get_chips(), 800);
    assert_eq!(player.get_bet_in_round(), 200);
    
    // Test all-in
    let mut all_in_player = create_test_player();
    assert!(all_in_player.place_bet(1000).is_ok());
    assert_eq!(all_in_player.get_chips(), 0);
    assert_eq!(all_in_player.get_bet_in_round(), 1000);
    
    // Test invalid bet (more than chips)
    assert!(matches!(
        all_in_player.place_bet(1),
        Err(PlayerError::InsufficientChips)
    ));
}

#[test]
fn test_win_pot() {
    let mut player = create_test_player();
    
    // Simulate winning a standard Texas Hold'em pot
    player.place_bet(200).unwrap(); // Initial raise
    player.win_pot(600); // Win pot with two callers
    
    assert_eq!(player.get_chips(), 1400);
}

#[test]
fn test_reset_bet_in_round() {
    let mut player = create_test_player();
    
    // Simulate Texas Hold'em street betting
    player.place_bet(100).unwrap(); // Big blind
    assert_eq!(player.get_bet_in_round(), 100);
    
    player.reset_bet_in_round(); // Next street
    assert_eq!(player.get_bet_in_round(), 0);
}

#[test]
fn test_player_type() {
    let human_player = Player::new(
        String::from("Human"),
        1000,
        PlayerType::Human,
        PokerVariant::TexasHoldem
    );
    let bot_player = Player::new(
        String::from("Bot"),
        1000,
        PlayerType::Bot,
        PokerVariant::TexasHoldem
    );

    match human_player.get_player_type() {
        PlayerType::Human => assert!(true),
        _ => panic!("Expected Human player type"),
    }

    match bot_player.get_player_type() {
        PlayerType::Bot => assert!(true),
        _ => panic!("Expected Bot player type"),
    }
}

#[test]
fn test_texas_holdem_positions() {
    let mut player = create_test_player();
    
    // Test all valid Texas Hold'em positions
    let positions = vec![
        Position::Button,
        Position::SmallBlind,
        Position::BigBlind,
        Position::UTG,
        Position::MP,
        Position::HJ,
        Position::CO,
    ];

    for pos in positions {
        player.adjust_position(pos);
        assert_eq!(player.get_position(), Some(pos));
    }
}

#[test]
fn test_multiple_betting_rounds() {
    let mut player = create_test_player();
    let initial_chips = player.get_chips();
    
    // Simulate Texas Hold'em betting rounds
    // Preflop
    player.place_bet(100).unwrap(); // Big blind
    player.reset_bet_in_round();
    
    // Flop
    player.place_bet(200).unwrap();
    player.reset_bet_in_round();
    
    // Turn
    player.place_bet(300).unwrap();
    player.reset_bet_in_round();
    
    // River
    player.place_bet(400).unwrap();
    
    assert_eq!(player.get_chips(), initial_chips - 1000);
}