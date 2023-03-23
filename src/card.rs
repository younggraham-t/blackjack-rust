use strum_macros::EnumIter; // 0.17.1
use std::fmt;
#[derive(Debug, EnumIter, Copy, Clone, PartialEq)]
pub enum Value {
    Ace = 1, Two, Three, Four, Five,
    Six, Seven, Eight, Nine, Ten,
    Jack, Queen, King,
}

impl Value {
    pub fn get_numeric_value(self) -> i32 {
        match self {
            Value::Jack | Value::Queen | Value::King => 10,
            Value::Ace => 11,
            _ => self as i32,
        }
    }
}

// implementation found at https://stackoverflow.com/questions/32710187/how-do-i-get-an-enum-as-a-string
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Debug, EnumIter, Copy, Clone)]
pub enum Suit {
    Diamonds, Hearts, Clubs, Spades,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct Card {
    pub value: Value,
    pub suit: Suit,
    pub name: String,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?})", self.name)
    }
}

