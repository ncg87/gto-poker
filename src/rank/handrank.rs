use crate::cards::card::Card;

enum HandRank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}

struct Hand {
    cards: [Card; 5], // initializing a array of 5 cards
    rank: HandRank,
}