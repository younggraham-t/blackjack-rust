// #![allow(unused_imports, dead_code)]
mod deck;
mod card;
mod game;
mod player;
mod app;
mod buttons;


// fn main() {
//     let mut dealer = Dealer::new();
//
//     dealer.run();
// }



fn main() {
    // yew::Renderer::<App>::new().render();
    app::render();
}
