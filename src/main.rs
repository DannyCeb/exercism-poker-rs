#![allow(unused)]
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, Eq, Ord)]
enum CardType {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl PartialOrd for CardType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

impl PartialEq for CardType {
    fn eq(&self, other: &Self) -> bool {
        use CardType::*;
        match (self, other) {
            (Clubs, Clubs) | (Diamonds, Diamonds) | (Hearts, Hearts) | (Spades, Spades) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, Eq)]
enum Card {
    Two(CardType),
    Three(CardType),
    Four(CardType),
    Five(CardType),
    Six(CardType),
    Seven(CardType),
    Eight(CardType),
    Nine(CardType),
    Ten(CardType),
    Jack(CardType),
    Queen(CardType),
    King(CardType),
    Ace(CardType),
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_value = match self {
            Card::Two(_) => 2,
            Card::Three(_) => 3,
            Card::Four(_) => 4,
            Card::Five(_) => 5,
            Card::Six(_) => 6,
            Card::Seven(_) => 7,
            Card::Eight(_) => 8,
            Card::Nine(_) => 9,
            Card::Ten(_) => 10,
            Card::Jack(_) => 11,
            Card::Queen(_) => 12,
            Card::King(_) => 13,
            Card::Ace(_) => 14,
        };

        let other_value = match other {
            Card::Two(_) => 2,
            Card::Three(_) => 3,
            Card::Four(_) => 4,
            Card::Five(_) => 5,
            Card::Six(_) => 6,
            Card::Seven(_) => 7,
            Card::Eight(_) => 8,
            Card::Nine(_) => 9,
            Card::Ten(_) => 10,
            Card::Jack(_) => 11,
            Card::Queen(_) => 12,
            Card::King(_) => 13,
            Card::Ace(_) => 14,
        };

        self_value.partial_cmp(&other_value)
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        match self.partial_cmp(other) {
            Some(Ordering::Equal) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
enum DetValue {
    DetCard(Card),
    DetCardAndHigCard(Card, Card),
}

impl PartialOrd for DetValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            DetValue::DetCard(s_card) => match other {
                DetValue::DetCard(o_card) => s_card.partial_cmp(&o_card),
                _ => None,
            },
            DetValue::DetCardAndHigCard(s_card1, s_card2) => match other {
                DetValue::DetCardAndHigCard(o_card1, o_card2) => {
                    if s_card1 != o_card1 {
                        s_card1.partial_cmp(&o_card1)
                    } else {
                        s_card2.partial_cmp(&o_card2)
                    }
                }
                _ => None,
            },
        }
    }
}

impl PartialEq for DetValue {
    fn eq(&self, other: &Self) -> bool {
        match self.partial_cmp(other) {
            Some(Ordering::Equal) => true,
            _ => false,
        }
    }
}
#[derive(Debug)]
enum Value {
    StraightFlush(DetValue),
    FourOfaKind(DetValue),
    FullHouse(DetValue),
    Flush(DetValue),
    Straight(DetValue),
    ThreeOfaKind(DetValue),
    TwoPair(DetValue),
    OnePair(DetValue),
    HighCard(DetValue),
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Value::*;

        let rank = |v: &Value| match v {
            StraightFlush(_) => 8,
            FourOfaKind(_) => 7,
            FullHouse(_) => 6,
            Flush(_) => 5,
            Straight(_) => 4,
            ThreeOfaKind(_) => 3,
            TwoPair(_) => 2,
            OnePair(_) => 1,
            HighCard(_) => 0,
        };

        let s_rank = rank(self);
        let o_rank = rank(other);

        if s_rank == o_rank {
            match (self, other) {
                (StraightFlush(a), StraightFlush(b))
                | (FourOfaKind(a), FourOfaKind(b))
                | (FullHouse(a), FullHouse(b))
                | (Flush(a), Flush(b))
                | (Straight(a), Straight(b))
                | (ThreeOfaKind(a), ThreeOfaKind(b))
                | (TwoPair(a), TwoPair(b))
                | (OnePair(a), OnePair(b))
                | (HighCard(a), HighCard(b)) => a.partial_cmp(b),
                _ => None,
            }
        } else {
            rank(self).partial_cmp(&rank(other))
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match self.partial_cmp(other) {
            Some(Ordering::Equal) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
struct Hand {
    cards_index: usize,
    hand_value: Value,
}

impl Hand {
    fn new(hand: &str, cards_index: usize) -> Hand {
        let mut cards = Vec::new();
        for card in hand.split_whitespace() {
            let (value, suit) = card.split_at(card.len() - 1);
            let card_type = match suit {
                "C" => CardType::Clubs,
                "D" => CardType::Diamonds,
                "H" => CardType::Hearts,
                "S" => CardType::Spades,
                _ => panic!("Invalid suit"),
            };
            let card = match value {
                "2" => Card::Two(card_type),
                "3" => Card::Three(card_type),
                "4" => Card::Four(card_type),
                "5" => Card::Five(card_type),
                "6" => Card::Six(card_type),
                "7" => Card::Seven(card_type),
                "8" => Card::Eight(card_type),
                "9" => Card::Nine(card_type),
                "10" => Card::Ten(card_type),
                "J" => Card::Jack(card_type),
                "Q" => Card::Queen(card_type),
                "K" => Card::King(card_type),
                "A" => Card::Ace(card_type),
                _ => panic!("Invalid value"),
            };
            cards.push((card, false));
        }
        cards.sort();
        println!("{:#?}", cards);

        Hand {
            cards_index,
            hand_value: Hand::score_hand(&cards),
        }
    }

    fn high_card(hand: &Vec<(Card, bool)>) -> Option<Value> {
        let mut cardaux: Option<Card> = None;
        for (index, (card, used)) in hand.iter().enumerate() {
            if !used {
                cardaux = match cardaux {
                    None => Some(card.clone()),
                    Some(c) => {
                        if c > *card {
                            Some(c)
                        } else {
                            Some(card.clone())
                        }
                    }
                }
            }
        }

        match cardaux {
            None => None,
            Some(card) => Some(Value::HighCard(DetValue::DetCard(card))),
        }
    }

    fn score_hand(hand: &Vec<(Card, bool)>) -> Value {
        Hand::high_card(hand).unwrap()
    }
}

fn main() {
    let c1 = Card::Two(CardType::Hearts);
    let c2 = Card::Two(CardType::Spades);
    let c3 = Card::Ace(CardType::Hearts);
    let c4 = Card::Two(CardType::Spades);

    let d1 = DetValue::DetCardAndHigCard(c1, c2);
    let d2 = DetValue::DetCardAndHigCard(c3, c4);

    let h1 = Value::ThreeOfaKind(d1);
    let h2 = Value::ThreeOfaKind(d2);

    println!("{}", h1 < h2);

    let hand1 = Hand::new("4S JH 5D 3S 6H", 1);
    //let hand2 = Hand::new("3H 4H 5C 6C JD", 0);

    //println!("{:#?}", hand1);

    println!("============================================================================");
    //println!("{:#?}", hand2);
}
