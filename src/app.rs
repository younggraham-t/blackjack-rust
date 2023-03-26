use yew::prelude::*;

use crate::card::*;
use crate::game::*;
use crate::deck::*;

use rand::thread_rng;
use rand::seq::SliceRandom;
use gloo_console::log;
use wasm_bindgen::JsValue;
use strum::IntoEnumIterator; // 0.17.1




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

                <img src="assets/cards/AC.png" alt="card" width="100"/>

            </>
        }
    }
}

pub fn render() {
    
    yew::Renderer::<App>::new().render();
}
