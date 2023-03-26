use strum_macros::EnumIter; // 0.17.1
use std::fmt;

use gloo_console::log;
use wasm_bindgen::JsValue;
use yew::prelude::*;

const CARD_WIDTH: &str = "100";

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


#[derive(Debug, EnumIter, Copy, Clone, PartialEq)]
pub enum Suit {
    Diamonds, Hearts, Clubs, Spades,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


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
        let suit_js = JsValue::from(suit_abbr.clone());
        let value_js = JsValue::from(suit_abbr.clone());
        log!(suit_js, value_js);
        format!("assets/cards/{}{}.png", value_abbr, suit_abbr)
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?})", self.name)
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct CardDetailsProps {
    pub card: Card,
}

#[function_component(CardDetails)]
pub fn card_details(CardDetailsProps { card }: &CardDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ card.name.clone() }</h3>
            <img src={card.get_image_name()} alt="card" width={CARD_WIDTH}/>
        </div>
    }
}

