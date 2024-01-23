use crate::card::Card;

pub enum HandType {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    High,
}
impl HandType {
    pub fn get_value(&self) -> u8 {
        use HandType::*;
        return match self {
            FiveKind => 7,
            FourKind => 6,
            FullHouse => 5,
            ThreeKind => 4,
            TwoPair => 3,
            OnePair => 2,
            High => 1,
        };
    }
}
impl std::cmp::Eq for HandType {}
impl std::cmp::PartialEq for HandType {
    fn eq(&self, other: &Self) -> bool {
        return self.get_value() == other.get_value();
    }
}
impl std::cmp::PartialOrd for HandType {
    fn lt(&self, other: &Self) -> bool {
        return self.get_value() < other.get_value();
    }
    fn le(&self, other: &Self) -> bool {
        return self.get_value() <= other.get_value();
    }
    fn gt(&self, other: &Self) -> bool {
        return self.get_value() > other.get_value();
    }
    fn ge(&self, other: &Self) -> bool {
        return self.get_value() >= other.get_value();
    }
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering::*;
        if self.lt(other) {
            return Some(Less);
        }
        if self.gt(other) {
            return Some(Greater);
        }
        return Some(Equal);
    }
}
impl std::cmp::Ord for HandType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering::*;
        if self.get_value() < other.get_value() {
            return Less;
        }
        if self.get_value() > other.get_value() {
            return Greater;
        }
        return Equal;
    }
    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        use std::cmp::Ordering::*;
        return match self.cmp(&other) {
            Less | Equal => self,
            Greater => other,
        };
    }
    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        use std::cmp::Ordering::*;
        return match self.cmp(&other) {
            Greater | Equal => self,
            Less => other,
        };
    }
}

pub struct Hand {
    hand: [Card; 5],
    bid: u32,
    hand_type: HandType,
}
impl Hand {
    pub fn parse(l: String) -> Result<Self, String> {
        let seperator_index = match l.find(' ') {
            Some(i) => i,
            None => return Err(format!("line does not contain a space separator: {}", l)),
        };

        const HAND_PLACHOLDER: Option<Card> = None;
        let mut hand = [HAND_PLACHOLDER; 5];
        for (i, c) in l[0..seperator_index].chars().enumerate() {
            if i > 4 {
                return Err(format!(
                    "Line is not properly formatted for the parser: {}",
                    l
                ));
            }

            hand[i] = match Card::parse(c) {
                Ok(card) => Some(card),
                Err(e) => return Err(e),
            };
        }
        if hand.contains(&HAND_PLACHOLDER) {
            return Err("Cards in hand were not properly parsed!".to_string());
        }
        let hand = hand.map(|c| c.unwrap());

        let bid = match l[seperator_index + 1..l.len()].parse::<u32>() {
            Ok(b) => b,
            Err(e) => return Err(e.to_string()),
        };

        let hand_type = Self::get_type(&hand);

        return Ok(Self {
            hand,
            bid,
            hand_type,
        });
    }

    pub fn calc_winnings(&self, rank: u32) -> u32 {
        return self.bid * rank;
    }

    fn get_type(hand: &[Card; 5]) -> HandType {
        let mut layout = [0u8; 13];
        for c in hand.iter() {
            layout[(c.get_value() - 2) as usize] += 1;
        }

        use HandType::*;
        if layout.contains(&5) {
            return FiveKind;
        }
        if layout.contains(&4) {
            return FourKind;
        }
        if layout.contains(&3) {
            if layout.contains(&2) {
                return FullHouse;
            }
            return ThreeKind;
        }
        let mut pair_count = 0;
        for c in layout.iter() {
            if *c == 2 {
                pair_count += 1;
            }
            if pair_count == 2 {
                return TwoPair;
            }
        }
        if pair_count == 1 {
            return OnePair;
        }
        return High;
    }
}
impl std::cmp::Eq for Hand {}
impl std::cmp::PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        use std::cmp::Ordering::*;
        match self.hand_type.cmp(&other.hand_type) {
            Equal => (),
            _ => return false,
        }

        for i in 0..self.hand.len() {
            if self.hand[i] != other.hand[i] {
                return false;
            }
        }
        return true;
    }
}
impl std::cmp::Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering::*;
        let type_cmp = self.hand_type.cmp(&other.hand_type);
        if type_cmp != Equal {
            return type_cmp;
        }

        for i in 0..self.hand.len() {
            let card_cmp = self.hand[i].cmp(&other.hand[i]);
            if card_cmp != Equal {
                return card_cmp;
            }
        }
        return Equal;
    }
}
impl PartialOrd for Hand {
    fn lt(&self, other: &Self) -> bool {
        use std::cmp::Ordering::*;
        let type_cmp = self.hand_type.cmp(&other.hand_type);
        match type_cmp {
            Less => return true,
            Equal => (),
            Greater => return false,
        }

        for i in 0..self.hand.len() {
            let card_cmp = self.hand[i].cmp(&other.hand[i]);
            match card_cmp {
                Less => return true,
                Equal => (),
                Greater => return false,
            }
        }
        return false;
    }
    fn gt(&self, other: &Self) -> bool {
        use std::cmp::Ordering::*;
        let type_cmp = self.hand_type.cmp(&other.hand_type);
        match type_cmp {
            Less => return false,
            Equal => (),
            Greater => return true,
        }

        for i in 0..self.hand.len() {
            let card_cmp = self.hand[i].cmp(&other.hand[i]);
            match card_cmp {
                Less => return false,
                Equal => (),
                Greater => return true,
            }
        }
        return false;
    }
    fn le(&self, other: &Self) -> bool {
        use std::cmp::Ordering::*;
        let type_cmp = self.hand_type.cmp(&other.hand_type);
        match type_cmp {
            Less => return true,
            Equal => (),
            Greater => return false,
        }

        for i in 0..self.hand.len() {
            let card_cmp = self.hand[i].cmp(&other.hand[i]);
            match card_cmp {
                Less => return true,
                Equal => (),
                Greater => return false,
            }
        }
        return true;
    }
    fn ge(&self, other: &Self) -> bool {
        use std::cmp::Ordering::*;
        let type_cmp = self.hand_type.cmp(&other.hand_type);
        match type_cmp {
            Less => return false,
            Equal => (),
            Greater => return true,
        }

        for i in 0..self.hand.len() {
            let card_cmp = self.hand[i].cmp(&other.hand[i]);
            match card_cmp {
                Less => return false,
                Equal => (),
                Greater => return true,
            }
        }
        return true;
    }
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering::*;
        let type_cmp = self.hand_type.cmp(&other.hand_type);
        if type_cmp != Equal {
            return Some(type_cmp);
        }

        for i in 0..self.hand.len() {
            let card_cmp = self.hand[i].cmp(&other.hand[i]);
            if card_cmp != Equal {
                return Some(card_cmp);
            }
        }
        return Some(Equal);
    }
}
