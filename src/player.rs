use crate::card::*;
use std::fmt;


const BLACKJACK: i32 = 21;

pub struct PlayerHand {
    held_cards: Vec<Card>, 
}

impl PlayerHand {
    pub fn new() -> Self {
        Self {
           held_cards: Vec::new(),
        } 
    }

    pub fn add_card_to_hand(&mut self, card: Card) {
        self.held_cards.push(card);
    }

    pub fn is_busted(&mut self) -> bool {
        self.get_hand_total() > BLACKJACK
    }

    pub fn get_hand_total(&mut self) -> i32 {
        let mut sum_of_hand: i32 = 0;
        let mut number_of_aces = 0;
        for card in &self.held_cards {
            let card_value: Value = card.value;
            if card_value == Value::Ace {
                number_of_aces += 1;
            } 
            else {
                sum_of_hand += card_value.get_numeric_value();
            }
        }

        for _ in 0..number_of_aces {
            let difference_to_21 = BLACKJACK - sum_of_hand;
            if difference_to_21 < 11 {
                sum_of_hand += 1;
            }
            else {
                sum_of_hand += 11;
            }
        }
        sum_of_hand
    }

}

//display docs: https://doc.rust-lang.org/std/fmt/trait.Display.html
impl fmt::Display for PlayerHand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string_vec: Vec<&str> = Vec::new();
        for card in &self.held_cards {
            string_vec.push(card.name.as_str());
        }
        let string = string_vec.join(", ");
        write!(f, "({})", string)
    }
}

