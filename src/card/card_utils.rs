use strum_macros::EnumIter; // 0.17.1
use std::fmt;
use yew::prelude::*;
use crate::card::*;
use stylist::css;
use stylist::yew::*;

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


#[derive(Properties, PartialEq, Clone)]
pub struct CardDetailsProps {
    pub card: Card,
    // pub on_click: Callback<MouseEvent>
}

#[styled_component(CardDetails)]
pub fn card_details(CardDetailsProps { card }: &CardDetailsProps) -> Html {
    html! {
        <div class={css!(
                r#".column{
                    display: inline-block;}
        "#)}>
            // <h3>{ card.name.clone() }</h3>
            <img src={card.get_image_name()} alt="card" width={CARD_WIDTH} />
        </div>
    }
}


