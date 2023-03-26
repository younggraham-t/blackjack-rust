use crate::card::*;
use std::fmt;
use yew::prelude::*;
use stylist::css;
use stylist::yew::*;
const BLACKJACK: i32 = 21;

#[styled_component(Player)]
pub fn player(PlayerHand { held_cards, on_click }: &PlayerHand) -> Html {
    // let style_format = format!(".column {{float: left; width:{}%; padding: 5px;}}", 
    //                            100/held_cards.len()
    //                            );
    // let style = use_style!(style_format);
    let on_click = on_click.clone();
        held_cards.iter().map(|card| {
            let on_card_select = {
                let on_click = on_click.clone();
                let card = card.clone();
                Callback::from(move |_| {
                    on_click.emit(card.clone())
                })
            };


            html! {

                // <div id={"player_cards"}> 
                    <CardDetails card={card.clone()} on_click={on_card_select} />
                // </div>
                // <p key={card.id} onclick={on_card_select}>{format!("{} {}", card.value, card.suit)}</p>
            }
        }).collect()
}

#[derive(Properties, PartialEq)]
pub struct PlayerHand {
    pub held_cards: Vec<Card>,
    pub on_click: Callback<Card>
}

impl PlayerHand {

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

