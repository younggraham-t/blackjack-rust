mod deck;
mod card;
mod game;
mod player;

// use crate::deck::Deck;
use card::Card;
use game::Dealer;
// use std::collections::HashMap;
// use std::io::stdin;
//

fn main() {
    let mut dealer = Dealer::new();

    dealer.run();
}

//
// fn run(dealer: Dealer) {
//     dealer.deal_cards();
//     let mut user_options: HashMap<String, fn(&Dealer)> = HashMap::new();
//     // type Binop = fn();
//     // let dp: Binop = Dealer::dealer_plays();
//     user_options.insert("stand".to_string(), dealer_plays(&dealer));
//
//
//     loop {
//         let mut player_input = String::new();
//         println!("Do you wish to 'hit' or 'stand'?");
//         stdin().read_line(&mut player_input).expect("yes");
//         player_input = player_input.to_lowercase();
//         match player_input.as_str() {
//             "stand" => {println!("stand");
//                 /*end loop and draw cards for the dealer*/ 
//                 break;},
//
//             "hit" => /*draw a card*/ println!("hit"),
//
//             _ => println!("Enter either 'hit' or 'stand'."),
//         }
//     }
//
// }
//
// fn dealer_plays(dealer: &Dealer) {
//
// }
