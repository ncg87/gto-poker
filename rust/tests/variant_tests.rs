use gto_poker::poker::variant::PokerVariant;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hole_cards() {
        assert_eq!(PokerVariant::Kuhn.hole_cards(), 1);
        assert_eq!(PokerVariant::ThreeCard.hole_cards(), 3);
        assert_eq!(PokerVariant::FiveCard.hole_cards(), 5);
        assert_eq!(PokerVariant::TexasHoldem.hole_cards(), 2);
        assert_eq!(PokerVariant::OmahaHoldem.hole_cards(), 4);
    }

    #[test]
    fn test_community_cards() {
        assert_eq!(PokerVariant::Kuhn.community_cards(), 0);
        assert_eq!(PokerVariant::ThreeCard.community_cards(), 0);
        assert_eq!(PokerVariant::FiveCard.community_cards(), 0);
        assert_eq!(PokerVariant::TexasHoldem.community_cards(), 5);
        assert_eq!(PokerVariant::OmahaHoldem.community_cards(), 5);
    }

    #[test]
    fn test_variant_equality() {
        assert_eq!(PokerVariant::Kuhn, PokerVariant::Kuhn);
        assert_ne!(PokerVariant::Kuhn, PokerVariant::ThreeCard);
        
        let variant = PokerVariant::TexasHoldem;
        let copied = variant;
        assert_eq!(variant, copied);
    }

    #[test]
    fn test_variant_debug_formatting() {
        assert_eq!(format!("{:?}", PokerVariant::Kuhn), "Kuhn");
        assert_eq!(format!("{:?}", PokerVariant::ThreeCard), "ThreeCard");
        assert_eq!(format!("{:?}", PokerVariant::FiveCard), "FiveCard");
        assert_eq!(format!("{:?}", PokerVariant::TexasHoldem), "TexasHoldem");
        assert_eq!(format!("{:?}", PokerVariant::OmahaHoldem), "OmahaHoldem");
    }

    #[test]
    fn test_variant_clone() {
        let variant = PokerVariant::TexasHoldem;
        let cloned = variant.clone();
        assert_eq!(variant, cloned);
        
        // Ensure clone works for all variants
        for &variant in &[
            PokerVariant::Kuhn,
            PokerVariant::ThreeCard,
            PokerVariant::FiveCard,
            PokerVariant::TexasHoldem,
            PokerVariant::OmahaHoldem,
        ] {
            let cloned = variant.clone();
            assert_eq!(variant, cloned);
        }
    }
}