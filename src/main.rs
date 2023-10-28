use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, Eq, Ord)]
enum CardType {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl PartialOrd for CardType {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
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

impl Card {
    fn get_type(&self) -> CardType {
        use Card::*;
        match self {
            Two(ct) | Three(ct) | Four(ct) | Five(ct) | Six(ct) | Seven(ct) | Eight(ct)
            | Nine(ct) | Ten(ct) | Jack(ct) | Queen(ct) | King(ct) | Ace(ct) => ct.clone(),
        }
    }

    fn get_value(&self) -> i32 {
        match self {
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
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_value = self.get_value();

        let other_value = other.get_value();

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

#[derive(Debug, Eq, Ord, Clone)]
enum DetValue {
    DetCard(Card),
    DetCardAndHighCard(Card, Card),
    DetCardOneCardTwoAndHigCard(Card, Card, Card),
    DetFullHand(Card, Card, Card, Card, Card),
}

impl DetValue {
    fn get_card(&self) -> Card {
        match self {
            DetValue::DetCard(c) => c.clone(),
            DetValue::DetCardAndHighCard(c, _) => c.clone(),
            DetValue::DetCardOneCardTwoAndHigCard(c, _, _) => c.clone(),
            DetValue::DetFullHand(c, _, _, _, _) => c.clone(),
        }
    }
}

impl PartialOrd for DetValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            DetValue::DetCard(s_card) => match other {
                DetValue::DetCard(o_card) => s_card.partial_cmp(&o_card),
                _ => None,
            },
            DetValue::DetCardAndHighCard(s_card1, s_card2) => match other {
                DetValue::DetCardAndHighCard(o_card1, o_card2) => {
                    if s_card1 != o_card1 {
                        s_card1.partial_cmp(&o_card1)
                    } else {
                        s_card2.partial_cmp(&o_card2)
                    }
                }
                _ => None,
            },
            DetValue::DetCardOneCardTwoAndHigCard(s_card1, s_card2, s_card3) => match other {
                DetValue::DetCardOneCardTwoAndHigCard(o_card1, o_card2, o_card3) => {
                    if s_card1 != o_card1 {
                        s_card1.partial_cmp(&o_card1)
                    } else if s_card2 != o_card2 {
                        s_card2.partial_cmp(&o_card2)
                    } else {
                        s_card3.partial_cmp(&o_card3)
                    }
                }
                _ => None,
            },
            DetValue::DetFullHand(s_card1, s_card2, s_card3, s_card4, s_card5) => match other {
                DetValue::DetFullHand(o_card1, o_card2, o_card3, o_card4, o_card5) => {
                    if s_card1 != o_card1 {
                        s_card1.partial_cmp(&o_card1)
                    } else if s_card2 != o_card2 {
                        s_card2.partial_cmp(&o_card2)
                    } else if s_card3 != o_card3 {
                        s_card3.partial_cmp(&o_card3)
                    } else if s_card4 != o_card4 {
                        s_card4.partial_cmp(&o_card4)
                    } else {
                        s_card5.partial_cmp(&o_card5)
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
#[derive(Debug, Eq, Ord, Clone)]
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

impl Value {
    fn get_card(&self) -> Card {
        use Value::*;
        match self {
            StraightFlush(dt) | FourOfaKind(dt) | FullHouse(dt) | Flush(dt) | Straight(dt)
            | ThreeOfaKind(dt) | TwoPair(dt) | OnePair(dt) | HighCard(dt) => dt.get_card(),
        }
    }
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

#[derive(Debug, Eq, Ord, Clone)]
struct Hand {
    cards_index: usize,
    hand_value: Value,
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.hand_value.partial_cmp(&other.hand_value)
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        match self.partial_cmp(other) {
            Some(Ordering::Equal) => true,
            _ => false,
        }
    }
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
        //println!("{:#?}", cards);

        Hand {
            cards_index,
            hand_value: Hand::score_hand(&mut cards),
        }
    }

    fn straight_flush(hand: &mut Vec<(Card, bool)>) -> Option<Value> {
        match Hand::flush(hand) {
            Some(_) => match Hand::straight(hand) {
                Some(_) => Some(Value::StraightFlush(DetValue::DetCard(
                    hand[hand.len() - 1].0.clone(),
                ))),
                None => None,
            },
            None => None,
        }
    }
    fn four_of_a_kind(hand: &mut Vec<(Card, bool)>) -> Option<Value> {
        if hand[0].0.get_value() == hand[2].0.get_value()
            || hand[4].0.get_value() == hand[2].0.get_value()
        {
            let aux_value = hand[2].0.get_value();
            let mut strikes = 0;
            let mut card_max = hand[2].0.clone();

            for (_, (l, _)) in hand.iter().enumerate() {
                if l.get_value() != aux_value {
                    strikes += 1;
                    if strikes > 1 {
                        return None;
                    }
                    card_max = l.clone();
                }
            }

            Some(Value::FourOfaKind(DetValue::DetCardAndHighCard(
                hand[2].0.clone(),
                card_max,
            )))
        } else {
            None
        }
    }
    fn full_house(hand: &mut Vec<(Card, bool)>) -> Option<Value> {
        if hand[0].0.get_value() == hand[2].0.get_value()
            && hand[4].0.get_value() == hand[3].0.get_value()
        {
            Some(Value::FullHouse(DetValue::DetCardAndHighCard(
                hand[0].0.clone(),
                hand[4].0.clone(),
            )))
        } else if hand[4].0.get_value() == hand[2].0.get_value()
            && hand[0].0.get_value() == hand[1].0.get_value()
        {
            Some(Value::FullHouse(DetValue::DetCardAndHighCard(
                hand[4].0.clone(),
                hand[0].0.clone(),
            )))
        } else {
            None
        }
    }
    fn flush(hand: &mut Vec<(Card, bool)>) -> Option<Value> {
        let aux: CardType = hand[0].0.get_type();
        for (_, (card, _)) in hand.iter().enumerate() {
            if card.get_type() == aux {
                continue;
            } else {
                return None;
            }
        }

        Some(Value::Flush(DetValue::DetCard(
            hand[hand.len() - 1].0.clone(),
        )))
    }
    fn straight(hand: &mut Vec<(Card, bool)>) -> Option<Value> {
        if hand[4].0.get_value() - hand[3].0.get_value() == 1
            && hand[3].0.get_value() - hand[2].0.get_value() == 1
            && hand[2].0.get_value() - hand[1].0.get_value() == 1
            && hand[1].0.get_value() - hand[0].0.get_value() == 1
        {
            Some(Value::Straight(DetValue::DetCard(
                hand[hand.len() - 1].0.clone(),
            )))
        } else {
            None
        }
    }
    fn three_of_a_kind(hand: &mut Vec<(Card, bool)>) -> Option<Value> {
        if hand[0].0.get_value() == hand[2].0.get_value() {
            Some(Value::ThreeOfaKind(DetValue::DetCardAndHighCard(
                hand[0].0.clone(),
                hand[4].0.clone(),
            )))
        } else if hand[4].0.get_value() == hand[2].0.get_value() {
            Some(Value::ThreeOfaKind(DetValue::DetCardAndHighCard(
                hand[4].0.clone(),
                hand[1].0.clone(),
            )))
        } else if hand[1].0.get_value() == hand[3].0.get_value() {
            Some(Value::ThreeOfaKind(DetValue::DetCardAndHighCard(
                hand[2].0.clone(),
                hand[4].0.clone(),
            )))
        } else {
            None
        }
    }
    fn two_pair(hand: &mut Vec<(Card, bool)>) -> Option<Value> {
        if hand[0].0.get_value() == hand[1].0.get_value()
            && hand[2].0.get_value() == hand[3].0.get_value()
        {
            Some(Value::TwoPair(DetValue::DetCardOneCardTwoAndHigCard(
                hand[0].0.clone(),
                hand[2].0.clone(),
                hand[4].0.clone(),
            )))
        } else if hand[0].0.get_value() == hand[1].0.get_value()
            && hand[4].0.get_value() == hand[3].0.get_value()
        {
            Some(Value::TwoPair(DetValue::DetCardOneCardTwoAndHigCard(
                hand[4].0.clone(),
                hand[0].0.clone(),
                hand[2].0.clone(),
            )))
        } else if hand[2].0.get_value() == hand[1].0.get_value()
            && hand[4].0.get_value() == hand[3].0.get_value()
        {
            Some(Value::TwoPair(DetValue::DetCardOneCardTwoAndHigCard(
                hand[4].0.clone(),
                hand[2].0.clone(),
                hand[0].0.clone(),
            )))
        } else {
            None
        }
    }

    fn one_pair(hand: &mut Vec<(Card, bool)>) -> Option<Value> {
        if hand[0].0.get_value() == hand[1].0.get_value() {
            hand[0].1 = true;
            hand[1].1 = true;

            Some(Value::OnePair(DetValue::DetCardAndHighCard(
                hand[0].0.clone(),
                Hand::high_card(hand).unwrap().get_card(),
            )))
        } else if hand[1].0.get_value() == hand[2].0.get_value() {
            hand[2].1 = true;
            hand[1].1 = true;
            Some(Value::OnePair(DetValue::DetCardAndHighCard(
                hand[1].0.clone(),
                Hand::high_card(hand).unwrap().get_card(),
            )))
        } else if hand[2].0.get_value() == hand[3].0.get_value() {
            hand[2].1 = true;
            hand[3].1 = true;
            Some(Value::OnePair(DetValue::DetCardAndHighCard(
                hand[2].0.clone(),
                Hand::high_card(hand).unwrap().get_card(),
            )))
        } else if hand[3].0.get_value() == hand[4].0.get_value() {
            hand[3].1 = true;
            hand[4].1 = true;
            Some(Value::OnePair(DetValue::DetCardAndHighCard(
                hand[3].0.clone(),
                Hand::high_card(hand).unwrap().get_card(),
            )))
        } else {
            None
        }
    }

    fn high_card(hand: &Vec<(Card, bool)>) -> Option<Value> {
        let mut aux = false;
        let mut cardaux: Option<Card> = None;
        for (_, (card, used)) in hand.iter().enumerate() {
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
            } else {
                aux = true;
            }
        }

        match cardaux {
            None => None,
            Some(card) => {
                if aux {
                    Some(Value::HighCard(DetValue::DetCard(card)))
                } else {
                    Some(Value::HighCard(DetValue::DetFullHand(
                        hand[4].0.clone(),
                        hand[3].0.clone(),
                        hand[2].0.clone(),
                        hand[1].0.clone(),
                        hand[0].0.clone(),
                    )))
                }
            }
        }
    }

    fn score_hand(hand: &mut Vec<(Card, bool)>) -> Value {
        Hand::straight_flush(hand).unwrap_or(Hand::four_of_a_kind(hand).unwrap_or(
            Hand::full_house(hand).unwrap_or(Hand::flush(hand).unwrap_or(
                Hand::straight(hand).unwrap_or(
                    Hand::three_of_a_kind(hand).unwrap_or(
                        Hand::two_pair(hand).unwrap_or(
                            Hand::one_pair(hand).unwrap_or(Hand::high_card(hand).unwrap()),
                        ),
                    ),
                ),
            )),
        ))
    }
}

fn main() {
    let c1 = Card::Two(CardType::Hearts);
    let c2 = Card::Two(CardType::Spades);
    let c3 = Card::Ace(CardType::Hearts);
    let c4 = Card::Two(CardType::Spades);

    let d1 = DetValue::DetCardAndHighCard(c1, c2);
    let d2 = DetValue::DetCardAndHighCard(c3, c4);

    let h1 = Value::ThreeOfaKind(d1);
    let h2 = Value::ThreeOfaKind(d2);

    println!("{}", h1 < h2);

    let hand1 = Hand::new("AS AC KS KC 6S", 1); // HC
    println!("{:#?}", hand1.hand_value);

    println!("============================================================================");

    /*let hand2 = Hand::new("2H 2D 2C 8H 5H", 0); // Color
    println!("{:#?}", hand2.hand_value);*/

    /*
    println!("============================================================================");

    let hand3 = Hand::new("4S 7H 6D 5S 3H", 0); // escalera sucia
    println!("{:#?}", hand3.hand_value);

    println!("============================================================================");

    let hand4 = Hand::new("4S 7S 6S 5S 3S", 0); // Escalera Color
    println!("{:#?}", hand4.hand_value);

    println!("============================================================================");

    let hand5 = Hand::new("4S 4H 4D 4S AS", 0); // 4
    println!("{:#?}", hand5.hand_value);

    println!("============================================================================");

    let hand6 = Hand::new("4S 4H AD AS AS", 0); // 3 y 2
    println!("{:#?}", hand6.hand_value);

    println!("============================================================================");

    let hand7 = Hand::new("4S 4H 7D AS 4S", 0); // 3
    println!("{:#?}", hand7.hand_value);

    println!("============================================================================");

    let hand8 = Hand::new("4S 4H 7D AS 7S", 0); //  2 2
    println!("{:#?}", hand8.hand_value);

    println!("============================================================================");

    let hand9 = Hand::new("4S 4H 7D AS QS", 0); // 2
    println!("{:#?}", hand9.hand_value);*/
}
