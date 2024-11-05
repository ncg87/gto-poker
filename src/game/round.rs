use crate::players::player::PlayerType;
use std::collections::VecDeque;

struct PokerRound {
    players: VecDeque<PlayerType>,
    current_bet: u32,
    pot: u32,
    active_players: usize,
}