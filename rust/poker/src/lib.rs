/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
/// 
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};
use core::cmp::Ordering;

type RankCount = HashMap<u8, u8>;
type SuitCount = HashMap<char, u8>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
    HighCard(u8),
    OnePair(u8),
    TwoPair(u8, u8),
    ThreeOfAKind(u8),
    Straight(u8),
    Flush(u8),
    FullHouse(u8, u8),
    FourOfAKind(u8),
    StraightFlush(u8),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    rank: u8,
    suit: char,
}

impl Card {
    pub fn new(card: &str) -> Self {
        let rank = match &card[0..card.len() - 1] {
            "A" => 14,
            "K" => 13,
            "Q" => 12,
            "J" => 11,
            _ => card[0..card.len() - 1].parse().unwrap(),
        };
        let suit = card.chars().last().unwrap();
        Card { rank, suit }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub struct Hand<'a> {
    pub hand: &'a str,
    pub cards: Vec<Card>,
    pub rank: Rank,
}

impl<'a> Hand<'a> {
    pub fn new(hand: &'a str) -> Self {
        let mut cards: Vec<Card> = Vec::new();
        for card in hand.split_whitespace() {
            cards.push(Card::new(card));
        }
        cards.sort_by(|a, b| b.rank.cmp(&a.rank));
        let rank = rank_hand(&cards);
        Hand { hand, cards, rank }
    }
}

impl<'a> Display for Hand<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}:{:?}", self.cards, self.rank)
    }
}

impl<'a> Ord for Hand<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.rank > other.rank {
            Ordering::Greater
        } else if self.rank == other.rank {
            for i in 0..self.cards.len() {
                if self.cards[i].rank > other.cards[i].rank {
                    return Ordering::Greater;
                } else if self.cards[i].rank < other.cards[i].rank {
                    return Ordering::Less;
                }
            }
            Ordering::Equal
        } else {
            Ordering::Less
        }
    }
}

fn is_straight(cards: &Vec<Card>) -> Option<u8> {
    if cards.windows(2).all(|w| w[0].rank == w[1].rank + 1) {
        return Some(cards[0].rank);
    } else if cards[4].rank == 2 && cards[0].rank == 14 && cards[1..].windows(2).all(|w| w[0].rank == w[1].rank + 1) {
        return Some(cards[1].rank);
    }
    None
}

fn rank_hand(cards: &Vec<Card>) -> Rank {
    let mut rank = Rank::HighCard(cards[0].rank);
    let mut rank_count = RankCount::new();
    let mut suit_count = SuitCount::new();
    for card in cards.iter() {
        *rank_count.entry(card.rank).or_insert(0) += 1;
        *suit_count.entry(card.suit).or_insert(0) += 1;
    }
    let is_flush = suit_count.len() == 1;
    if let Some(r) = is_straight(cards) {
        if is_flush {
            rank = Rank::StraightFlush(r);
        } else {
            rank = Rank::Straight(r);
        }   
    } else if is_flush {
        rank = Rank::Flush(cards[0].rank);
    } else if rank_count.values().any(|&v| v == 5) {
        panic!("Five of a kind not implemented");
    } else if rank_count.values().any(|&v| v == 4) {
        let r = *rank_count.iter().find(|&(_, &v)| v == 4).unwrap().0;
        rank = Rank::FourOfAKind(r);
    } else if rank_count.values().any(|&v| v == 3) && rank_count.values().any(|&v| v == 2) {
        let r1 = *rank_count.iter().find(|&(_, &v)| v == 3).unwrap().0;
        let r2 = *rank_count.iter().find(|&(_, &v)| v == 2).unwrap().0;
        rank = Rank::FullHouse(r1, r2);
    } else if rank_count.values().any(|&v| v == 3) {
        let r = *rank_count.iter().find(|&(_, &v)| v == 3).unwrap().0;
        rank = Rank::ThreeOfAKind(r);
    } else if rank_count.values().filter(|&v| *v == 2).count() == 2 {
        let mut rs: Vec<u8> = rank_count.iter().filter(|&(_, &v)| v == 2).map(|(&k, _)| k).collect();
        rs.sort_by(|a, b| b.cmp(a));
        rank = Rank::TwoPair(rs[0], rs[1]);
    } else if rank_count.values().any(|&v| v == 2) {
        let r = *rank_count.iter().find(|&(_, &v)| v == 2).unwrap().0;
        rank = Rank::OnePair(r);
    }
    rank
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut winning_hand = Hand::new(hands[0]);
    let mut winning_hands: Vec<&'a str> = vec![winning_hand.hand];
    for hand in hands.iter().skip(1) {
        let hand = Hand::new(hand);
        match hand.cmp(&winning_hand) {
            Ordering::Greater => {
                winning_hand = hand;
                winning_hands = vec![winning_hand.hand];
            }
            Ordering::Equal => {
                winning_hands.push(hand.hand);
            }
            Ordering::Less => {}
        }
    }
    winning_hands
}
