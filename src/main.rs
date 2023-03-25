#![allow(unused_imports)]
mod deck;
mod card;
mod game;
mod player;
mod app;

use card::Card;
use game::Dealer;
use deck::Deck;


// fn main() {
//     let mut dealer = Dealer::new();
//
//     dealer.run();
// }



fn main() {
    // yew::Renderer::<App>::new().render();
    app::render();
}
