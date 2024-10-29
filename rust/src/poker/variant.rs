// First, let's define the different poker variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PokerVariant {
    Kuhn,           // 1 card
    ThreeCard,      // 3 cards
    FiveCard,       // 5 cards
    TexasHoldem,    // 2 hole cards + 5 community
    OmahaHoldem,    // 4 hole cards + 5 community
}

impl PokerVariant {
    // Get the number of hole cards for each variant
    pub fn hole_cards(&self) -> usize {
        match self {
            PokerVariant::Kuhn => 1,
            PokerVariant::ThreeCard => 3,
            PokerVariant::FiveCard => 5,
            PokerVariant::TexasHoldem => 2,
            PokerVariant::OmahaHoldem => 4,
        }
    }

    // Get the number of community cards (if any)
    pub fn community_cards(&self) -> usize {
        match self {
            PokerVariant::TexasHoldem | PokerVariant::OmahaHoldem => 5,
            _ => 0,
        }
    }
}
