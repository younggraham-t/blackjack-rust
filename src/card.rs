use std::fmt;

use gloo_console::log;
use wasm_bindgen::JsValue;

const CARD_WIDTH: &str = "100";

mod card_utils;
pub use card_utils::*;


#[derive(Clone, Debug, PartialEq)]
pub struct Card {
    pub id: u32,
    pub value: Value,
    pub suit: Suit,
    pub name: String,
}

impl Card {

    pub fn get_image_name(&self) -> String {
        
        let value_abbr = match self.value {
            Value::Jack  => "J".to_string(),
            Value::Queen => "Q".to_string(),
            Value::King  => "K".to_string(),
            Value:: Ace  => "A".to_string(),
            _ => self.value.get_numeric_value().to_string(),
        };

        let suit_abbr = match self.suit {
            Suit::Spades   => "S".to_string(),
            Suit::Hearts   => "H".to_string(),
            Suit::Clubs    => "C".to_string(),
            Suit::Diamonds => "D".to_string(),
        };
        // let suit_js = JsValue::from(suit_abbr.clone());
        // let value_js = JsValue::from(suit_abbr.clone());
        // log!(suit_js, value_js);
        format!("assets/cards/{}{}.png", value_abbr, suit_abbr)
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?})", self.name)
    }
}


