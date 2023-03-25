mod deck;
mod card;
mod game;
mod player;
// // mod graphics;
//
// use crate::deck::Deck;
use card::Card;
use game::Dealer;
// use std::collections::HashMap;
// use std::io::stdin;
// //
//
// fn main() {
//     let mut dealer = Dealer::new();
//
//     dealer.run();
// }

use yew::prelude::*;

// #[function_component]
// fn App() -> Html {
//     let counter = use_state(|| 0);
//     let onclick = {
//         let counter = counter.clone();
//         move |_| {
//             let value = *counter + 1;
//             counter.set(value);
//         }
//     };
//
//     html! {
//         <div>
//             <button {onclick}>{ "+1" }</button>
//             <p>{ *counter }</p>
//         </div>
//     }
// }

#[function_component(App)]
fn app() -> Html {
html! {
    <>
        <h1>{ "RustConf Explorer" }</h1>
        <div>
            <h3>{"Videos to watch"}</h3>
            <p>{ "John Doe: Building and breaking things" }</p>
            <p>{ "Jane Smith: The development process" }</p>
            <p>{ "Matt Miller: The Web 7.0" }</p>
            <p>{ "Tom Jerry: Mouseless development" }</p>
        </div>
        <div>
            <h3>{ "John Doe: Building and breaking things" }</h3>
            <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
        </div>
    </>
}
}

fn main() {
    yew::Renderer::<App>::new().render();
}
