use crate::card::*;
use rand::thread_rng;
use rand::seq::SliceRandom;
use strum::IntoEnumIterator; // 0.17.1
// use strum_macros::EnumIter; // 0.17.1
use yew::prelude::*;

#[function_component(Deck)]
pub fn deck(DeckProps { cards, on_click }: &DeckProps) -> Html {
    let on_click = on_click.clone();
    cards.iter().map(|card| {
        let on_card_select = {
            let on_click = on_click.clone();
            let card = card.clone();
            Callback::from(move |_| {
                on_click.emit(card.clone())
            })
        };
        html! {
            <p key={card.id} onclick={on_card_select}>{format!("{} {}", card.value, card.suit)}</p>
        }
    }).collect()
}

#[derive(Properties, PartialEq)]
pub struct DeckProps {
    pub cards: Vec<Card>,
    pub on_click: Callback<Card>
}

impl DeckProps {
    // pub fn new() -> Self {
    //     let new_cards = Self::create_cards_vector();
    //
    //     Self { 
    //        cards: new_cards,
    //     }

//  }

    pub fn create_cards_vector() -> Vec<Card> {
        

        let mut cards: Vec<Card> = Vec::new();
        let mut card_id: u32 = 1;
        for value in Value::iter() {
            for suit in Suit::iter() {
                cards.push(Card{
                    id: card_id,
                    value,
                    suit,
                    name: format!("{} of {}", value, suit)});
                card_id += 1;
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
