use gto_poker::poker::deck::{Deck, DeckError};
use gto_poker::poker::card::{Card, Suit};

#[cfg(test)]
mod deck_tests {
    use super::*;

    #[test]
    fn test_new_deck() {
        let deck = Deck::new().unwrap();
        assert_eq!(deck.remaining(), 52);
    }

    #[test]
    fn test_draw_card() {
        let mut deck = Deck::new().unwrap();
        let card = deck.draw().unwrap();
        assert_eq!(deck.remaining(), 51);
        assert!(card.rank() >= 2 && card.rank() <= 14);
    }

    #[test]
    fn test_draw_all_cards() {
        let mut deck = Deck::new().unwrap();
        for _ in 0..52 {
            deck.draw().unwrap();
        }
        assert_eq!(deck.remaining(), 0);
        assert!(matches!(deck.draw(), Err(DeckError::NotEnoughCards)));
    }

    #[test]
    fn test_shuffle() {
        let mut deck1 = Deck::new().unwrap();
        let mut deck2 = Deck::new().unwrap();

        // Store the first few cards of deck1
        let first_cards: Vec<Card> = (0..5).map(|_| deck1.draw().unwrap()).collect();

        // Shuffle deck2
        deck2.shuffle();

        // Draw the first few cards of deck2
        let shuffled_cards: Vec<Card> = (0..5).map(|_| deck2.draw().unwrap()).collect();

        // Check that the cards are different (this has a very small chance of failing even if shuffle works correctly)
        assert_ne!(first_cards, shuffled_cards);
    }

    #[test]
    fn test_remaining_cards() {
        let mut deck = Deck::new().unwrap();
        assert_eq!(deck.remaining(), 52);

        for i in (1..=52).rev() {
            deck.draw().unwrap();
            assert_eq!(deck.remaining(), i - 1);
        }
    }

    #[test]
    fn test_initialize_with_all_cards() {
        let mut deck = Deck::new().unwrap();
        let mut card_counts = std::collections::HashMap::new();

        for _ in 0..52 {
            let card = deck.draw().unwrap();
            *card_counts.entry((card.suit(), card.rank())).or_insert(0) += 1;
        }

        for suit in &[Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
            for rank in 2..=14 {
                assert_eq!(card_counts[&(*suit, rank)], 1, "Card {:?} {} should appear exactly once", suit, rank);
            }
        }
    }
}