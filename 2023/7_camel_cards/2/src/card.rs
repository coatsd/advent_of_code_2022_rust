pub enum Card {
    Ace,
    King,
    Queen,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}
impl Card {
    pub fn parse(c: char) -> Result<Card, String> {
        use Card::*;
        let result = match c {
            'A' => Ace,
            'K' => King,
            'Q' => Queen,
            'T' => Ten,
            '9' => Nine,
            '8' => Eight,
            '7' => Seven,
            '6' => Six,
            '5' => Five,
            '4' => Four,
            '3' => Three,
            '2' => Two,
            'J' => Joker,
            _ => return Err(format!("Invalid character while parsing card: {}", c)),
        };

        return Ok(result);
    }

    pub fn get_value(&self) -> u8 {
        use Card::*;
        return match self {
            Ace => 13,
            King => 12,
            Queen => 11,
            Ten => 10,
            Nine => 9,
            Eight => 8,
            Seven => 7,
            Six => 6,
            Five => 5,
            Four => 4,
            Three => 3,
            Two => 2,
            Joker => 1,
        };
    }
}
impl std::cmp::Eq for Card {}
impl std::cmp::PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        return self.get_value() == other.get_value();
    }
}
impl std::cmp::PartialOrd for Card {
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
        if self.gt(other) {
            return Some(Greater);
        }
        if self.lt(other) {
            return Some(Less);
        }
        return Some(Equal);
    }
}
impl std::cmp::Ord for Card {
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
