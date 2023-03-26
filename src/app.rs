use yew::prelude::*;

use crate::card::*;
// use crate::game::*;
use crate::deck::*;
use crate::player::*;

use gloo_console::log;
use wasm_bindgen::JsValue;
use stylist::yew::*;
use stylist::css;



#[function_component(App)]
fn app() -> Html {
    let selected_card = use_state(|| None);

    let on_card_select = {
        let selected_card = selected_card.clone();
        Callback::from(move |card: Card|{
            log!(JsValue::from(card.clone().name));
            selected_card.set(Some(card))
        })
    };

    // let details = selected_card.as_ref().map(|card| html! {
    //     <CardDetails card={card.clone() } />
    // });

    // let deck = Deck::new();
    // let DeckHtml = deck(deck);
    let cards = DeckProps::create_cards_vector();
    let mut deck_props = DeckProps {cards, on_click: on_card_select.clone()};
    let cards = vec![deck_props.draw_card(), deck_props.draw_card()];

    html! {
        html! {
            <>
                <div class="w3-container">
                <h1>{ "Player Hand" }</h1>
                </div> 
                
                
                <Player held_cards={cards} on_click={on_card_select.clone()}/>
                
                
                
                // { for details }
            </>
        }
    }
}

pub fn render() {
    
    yew::Renderer::<App>::new().render();
}
