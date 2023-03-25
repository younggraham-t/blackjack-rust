use yew::prelude::*;

use crate::Card;
use crate::Dealer;
use crate::deck::*;

use rand::thread_rng;
use rand::seq::SliceRandom;
use crate::card::Value;
use crate::card::Suit;
use gloo_console::log;
use wasm_bindgen::JsValue;
use strum::IntoEnumIterator; // 0.17.1

// #[derive(Properties, PartialEq)]
// struct CardListProps {
//     cards: Vec<Card>,
//     on_click: Callback<Card>
//
// }

// impl CardListProps {
//
//     pub fn draw_card(&mut self) -> Card {
//         //restock the pile if its empty
//         if self.cards.is_empty() {
//             self.cards = Deck::create_cards_vector();
//         }
//         //take the card off the pile
//         self.cards.pop().expect("yes")
//         
//     }
//
// }

#[derive(Properties, PartialEq)]
struct CardDetailsProps {
    card: Card,
}

#[function_component(CardDetails)]
fn card_details(CardDetailsProps { card }: &CardDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ card.name.clone() }</h3>
            <img src="assets/3S.png" alt="card" />
        </div>
    }
}

// #[function_component(CardList)]
// fn deck(Deck { cards, on_click }: &Deck) -> Html {
//     let on_click = on_click.clone();
//     cards.iter().map(|card| {
//         let on_card_select = {
//             let on_click = on_click.clone();
//             let card = card.clone();
//             Callback::from(move |_| {
//                 on_click.emit(card.clone())
//             })
//         };
//         html! {
//             <p key={card.id} onclick={on_card_select}>{format!("{} {}", card.value, card.suit)}</p>
//         }
//     }).collect()
// }


#[function_component(App)]
fn app() -> Html {
    let selected_card = use_state(|| None);

    let on_card_select = {
        let selected_card = selected_card.clone();
        Callback::from(move |card: Card|{
            let selected_card_name = JsValue::from(card.clone().name);
            log!(selected_card_name);
            selected_card.set(Some(card))
        })
    };

    let details = selected_card.as_ref().map(|card| html! {
        <CardDetails card={card.clone()} />
    });

    // let deck = Deck::new();
    // let DeckHtml = deck(deck);
    let cards = DeckProps::create_cards_vector();

    html! {
        html! {
            <>
                <div>
                    <Deck cards={cards} on_click={on_card_select.clone()}/>
                </div>
                { for details }
            </>
        }
    }
}

pub fn render() {
    
    yew::Renderer::<App>::new().render();
}
