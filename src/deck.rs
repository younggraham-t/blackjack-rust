use crate::Card;
use rand::thread_rng;
use rand::seq::SliceRandom;
use crate::card::Value;
use crate::card::Suit;
use strum::IntoEnumIterator; // 0.17.1
// use strum_macros::EnumIter; // 0.17.1


pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let new_cards = Self::create_cards_vector();

        Self { 
           cards: new_cards,
        }

    }

    fn create_cards_vector() -> Vec<Card> {
        
        // let card_values = vec![
        //     "Two", "Three", "Four", "Five",
        //     "Six", "Seven", "Eight", "Nine", "Ten",
        //     "Jack", "Queen", "King", "Ace"];
        // let card_suits = vec!["Hearts", "Clubs", "Diamonds", "Spades"];

        let mut cards: Vec<Card> = Vec::new();
        for value in Value::iter() {
            for suit in Suit::iter() {
                cards.push(Card{
                    value,
                    suit,
                    name: format!("{} of {}", value, suit)})
            }
        }
        cards.shuffle(&mut thread_rng());
        cards
    }

    pub fn draw_card(&mut self) -> Card {
        //restock the pile if its empty
        if self.cards.is_empty() {
            self.cards = Self::create_cards_vector();
        }
        //take the card off the pile
        self.cards.pop().expect("yes")
        
    }
}
