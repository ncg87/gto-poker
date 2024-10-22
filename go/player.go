package main

import "fmt"

type Card struct {
    Suit   string
    Rank   string
    Value  int
}

type Player struct {
    Name   string
    Chips  int
    Hand   []Card
    Folded bool
}

func NewPlayer(name string, startingChips int) *Player {
    return &Player{
        Name:   name,
        Chips:  startingChips,
        Hand:   make([]Card, 0),
        Folded: false,
    }
}

func (p *Player) ReceiveCard(card Card) {
    p.Hand = append(p.Hand, card)
}

func (p *Player) Bet(amount int) bool {
    if amount > p.Chips {
        return false
    }
    p.Chips -= amount
    return true
}

func (p *Player) Fold() {
    p.Folded = true
}

func (p *Player) ShowHand() {
    fmt.Printf("%s's hand:\n", p.Name)
    for _, card := range p.Hand {
        fmt.Printf("  %s of %s\n", card.Rank, card.Suit)
    }
}