// tests/card_tests.rs

use gto_poker::poker::card::{Card, Suit, CardError};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_card_creation() {
        let card = Card::new(Suit::Hearts, 10).unwrap();
        assert_eq!(card.suit(), Suit::Hearts);
        assert_eq!(card.rank(), 10);
    }

    #[test]
    fn test_invalid_card_rank_low() {
        let result = Card::new(Suit::Spades, 1);
        assert!(matches!(result, Err(CardError::InvalidCardRank(1))));
    }

    #[test]
    fn test_invalid_card_rank_high() {
        let result = Card::new(Suit::Diamonds, 15);
        assert!(matches!(result, Err(CardError::InvalidCardRank(15))));
    }

    #[test]
    fn test_valid_card_ranks() {
        for rank in 2..=14 {
            let card = Card::new(Suit::Clubs, rank);
            assert!(card.is_ok());
        }
    }

    #[test]
    fn test_card_equality() {
        let card1 = Card::new(Suit::Hearts, 7).unwrap();
        let card2 = Card::new(Suit::Hearts, 7).unwrap();
        let card3 = Card::new(Suit::Spades, 7).unwrap();
        
        assert_eq!(card1, card2);
        assert_ne!(card1, card3);
    }

    #[test]
    fn test_card_clone() {
        let card = Card::new(Suit::Diamonds, 12).unwrap();
        let cloned_card = card.clone();
        
        assert_eq!(card, cloned_card);
    }

    #[test]
    fn test_suit_variants() {
        let suits = [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades];
        for &suit in &suits {
            let card = Card::new(suit, 10).unwrap();
            assert_eq!(card.suit(), suit);
        }
    }

    #[test]
    fn test_card_debug_output() {
        let card = Card::new(Suit::Hearts, 14).unwrap();
        let debug_output = format!("{:?}", card);
        assert!(debug_output.contains("Hearts"));
        assert!(debug_output.contains("14"));
    }
}